use anyhow::*;
use cgmath::{point3, Deg, InnerSpace, Matrix4, Point3, Rad, Transform};
use image::{ImageBuffer, Rgba};
use mesh::{Mesh, MeshBuilder};
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
                .help("The camera's vertical field of view in degrees")
                .required(true)
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
        .get_matches();

    let src_path = matches.value_of("INPUT").unwrap();
    let dst_path = matches.value_of("OUTPUT").unwrap();
    let width = matches.value_of("WIDTH").unwrap().parse::<u32>().unwrap();
    let height = matches.value_of("HEIGHT").unwrap().parse::<u32>().unwrap();
    let camera_fovy = Deg(matches
        .value_of("CAMERA VERTICAL FOV")
        .unwrap()
        .parse::<f32>()
        .unwrap());
    let camera_theta = match matches.value_of("CAMERA POSITION POLAR ANGLE") {
        Some(theta) => theta.parse::<f32>().unwrap(),
        None => 90.0,
    };
    let camera_phi = match matches.value_of("CAMERA POSITION AZIMUTHAL ANGLE") {
        Some(phi) => phi.parse::<f32>().unwrap(),
        None => 0.0,
    };
    let light_theta = match matches.value_of("LIGHT POSITION POLAR ANGLE") {
        Some(theta) => theta.parse::<f32>().unwrap(),
        None => 0.0,
    };
    let light_phi = match matches.value_of("LIGHT POSITION AZIMUTHAL ANGLE") {
        Some(phi) => phi.parse::<f32>().unwrap(),
        None => 0.0,
    };

    let file = std::fs::File::open(&src_path).unwrap();
    let mut reader = BufReader::new(&file);
    let mut mesh = MeshBuilder::from_stl(&mut reader)?.build();

    let (bounds_min, bounds_max) = mesh.bounding_box().unwrap();
    let center = bounds_min + (bounds_max - bounds_min) / 2.0;
    let center_to_origin = Point3::new(0.0f32, 0.0f32, 0.0f32) - center;
    mesh.transform(Matrix4::from_translation(center_to_origin));

    let bounding_sphere_radius = max_distance_from_origin(&mesh);
    let camera_dist = bounding_sphere_radius / f32::sin(Rad::from(camera_fovy / 2.0).0);
    let camera_position = (Matrix4::from_angle_z(Deg(camera_phi))
        * Matrix4::from_angle_y(Deg(camera_theta)))
    .transform_point(Point3::new(0.0, 0.0, camera_dist));
    let point_light_position = (Matrix4::from_angle_z(Deg(light_phi))
        * Matrix4::from_angle_y(Deg(light_theta)))
    .transform_point(Point3::new(0.0, 0.0, camera_dist));

    let config = wgpu_renderer::Config {
        mesh: &mesh,
        width,
        height,
        point_light_position,
        camera_position,
        camera_fovy,
    };
    let pixels = futures::executor::block_on(wgpu_renderer::render(config));
    let mut img_buffer: ImageBuffer<Rgba<u8>, _> =
        ImageBuffer::from_raw(width, height, pixels).unwrap();
    img_buffer.save(dst_path).unwrap();
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
