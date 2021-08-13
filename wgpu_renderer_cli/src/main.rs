mod error;

use cgmath::{point2, point3, Deg, InnerSpace, Matrix4, Point2, Point3, Rad, Transform};
use error::{Error, Result};
use image::{imageops, ImageBuffer, Rgba};
use mesh::{Mesh, MeshBuilder};
use std::cmp;
use std::io::BufReader;

fn main() -> Result<()> {
    let matches = clap::App::new("Part Viewer")
        .arg(
            clap::Arg::with_name("INPUT")
                .help("The input STL file to use")
                .required(true)
                .index(1),
        )
        .arg(
            clap::Arg::with_name("OUTPUT")
                .help("The output destination")
                .required(true)
                .index(2),
        )
        .arg(
            clap::Arg::with_name("WIDTH")
                .help("Width of the output image in pixels")
                .required(true)
                .index(3),
        )
        .arg(
            clap::Arg::with_name("HEIGHT")
                .help("Height of the output image in pixels")
                .required(true)
                .index(4),
        )
        .arg(
            clap::Arg::with_name("CAMERA VERTICAL FOV")
                .help("The camera's vertical field of view in degrees. Default is 45.")
                .required(false)
                .index(5),
        )
        .arg(
            clap::Arg::with_name("CAMERA POSITION POLAR ANGLE")
                .help("The camera's spherical position theta component. This is the angle between the camera and the z axis. Default is 90.")
                .required(false)
                .index(6),
        )
        .arg(
            clap::Arg::with_name("CAMERA POSITION AZIMUTHAL ANGLE")
                .help("The camera's spherical position phi component. This is the angle between the camera and the x axis in the xy plane. Default is 0.")
                .required(false)
                .index(7),
        )
        .arg(
            clap::Arg::with_name("LIGHT POSITION POLAR ANGLE")
                .help("The light's spherical position theta component. This is the angle between the light and the z axis. Default is 0.")
                .required(false)
                .index(8),
        )
        .arg(
            clap::Arg::with_name("LIGHT POSITION AZIMUTHAL ANGLE")
                .help("The light's spherical position phi component. This is the angle between the light and the x axis in the xy plane. Default is 0.")
                .required(false)
                .index(9),
        )
        .arg(
            clap::Arg::with_name("LIGHT INTENSITY")
                .help("The light's intensity. Default is 1.0.")
                .required(false)
                .index(10),
        )
        .arg(
            clap::Arg::with_name("CROP")
                .short("c")
                .help("Enables cropping"),
        )
        .get_matches();

    // The first four arguments are required by Clap, so unwrapping them is ok.
    let src_path = matches.value_of("INPUT").unwrap();
    let dst_path = matches.value_of("OUTPUT").unwrap();
    let width = matches.value_of("WIDTH").unwrap().parse::<u32>()?;
    let height = matches.value_of("HEIGHT").unwrap().parse::<u32>()?;

    let camera_fovy = Deg(matches
        .value_of("CAMERA VERTICAL FOV")
        .unwrap_or("45")
        .parse::<f32>()?);
    let camera_theta = Deg(matches
        .value_of("CAMERA POSITION POLAR ANGLE")
        .unwrap_or("90")
        .parse::<f32>()?);
    let camera_phi = Deg(matches
        .value_of("CAMERA POSITION AZIMUTHAL ANGLE")
        .unwrap_or("0")
        .parse::<f32>()?);
    let light_theta = Deg(matches
        .value_of("LIGHT POSITION POLAR ANGLE")
        .unwrap_or("0")
        .parse::<f32>()?);
    let light_phi = Deg(matches
        .value_of("LIGHT POSITION AZIMUTHAL ANGLE")
        .unwrap_or("0")
        .parse::<f32>()?);
    let point_light_intensity = matches
        .value_of("LIGHT INTENSITY")
        .unwrap_or("1.0")
        .parse::<f32>()?;
    let is_crop_on = matches.is_present("CROP");

    let file = std::fs::File::open(&src_path)?;
    let mut reader = BufReader::new(&file);
    let mut mesh = MeshBuilder::from_stl(&mut reader)?.build();

    let (bounds_min, bounds_max) = mesh.bounding_box().ok_or(Error::EmptyMesh)?;
    let center = bounds_min + (bounds_max - bounds_min) / 2.0;
    let center_to_origin = Point3::new(0.0f32, 0.0f32, 0.0f32) - center;
    mesh.transform(Matrix4::from_translation(center_to_origin));

    let bounding_sphere_radius = max_distance_from_origin(&mesh);
    let camera_dist = bounding_sphere_radius / f32::sin(Rad::from(camera_fovy / 2.0).0);
    let camera_position = (Matrix4::from_angle_z(camera_phi) * Matrix4::from_angle_y(camera_theta))
        .transform_point(Point3::new(0.0, 0.0, camera_dist));
    let point_light_position = (Matrix4::from_angle_z(light_phi)
        * Matrix4::from_angle_y(light_theta))
    .transform_point(Point3::new(0.0, 0.0, camera_dist));

    let config = wgpu_renderer::Config {
        mesh: &mesh,
        width,
        height,
        camera_fovy,
        camera_position,
        point_light_position,
        point_light_intensity,
    };
    let pixels = futures::executor::block_on(wgpu_renderer::render(config))?;
    let mut image: ImageBuffer<Rgba<u8>, _> =
        ImageBuffer::from_raw(width, height, pixels).ok_or(Error::ImageContainerTooSmall)?;

    if is_crop_on {
        let (crop_bounds_min, crop_bounds_max) =
            non_transparent_bounds(&image).ok_or(Error::ZeroAreaImage)?;
        let crop_bounds_diag = crop_bounds_max - crop_bounds_min;
        image = imageops::crop_imm(
            &image,
            crop_bounds_min.x,
            crop_bounds_min.y,
            crop_bounds_diag.x + 1,
            crop_bounds_diag.y + 1,
        )
        .to_image();
    }

    image.save(dst_path)?;
    Ok(())
}

/// Return the maximum distance between any vertex and the origin.
fn max_distance_from_origin(mesh: &Mesh) -> f32 {
    mesh.positions
        .iter()
        .fold(0.0f32, |acc, p| {
            let dist2 = (p - point3(0.0, 0.0, 0.0)).magnitude2();
            acc.max(dist2)
        })
        .sqrt()
}

/// Return the min and max (inclusive) pixels of a 2D bounding box around any
/// non-transparent content in the image.
fn non_transparent_bounds(
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> Option<(Point2<u32>, Point2<u32>)> {
    let mut min_max = None;

    for (x, y, rgba) in image.enumerate_pixels() {
        if rgba.0[3] == 0 {
            continue;
        }

        min_max = match min_max {
            None => Some((point2(x, y), point2(x, y))),
            Some((min, max)) => {
                let new_min = point2(cmp::min(min.x, x), cmp::min(min.y, y));
                let new_max = point2(cmp::max(max.x, x), cmp::max(max.y, y));
                Some((new_min, new_max))
            }
        };
    }

    min_max
}
