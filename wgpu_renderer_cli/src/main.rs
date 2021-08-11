mod bounding_box;

use bounding_box::BoundingBox;
use mesh::MeshBuilder;
use std::io::BufReader;

fn main() {
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
        .get_matches();

    let src_path = matches.value_of("INPUT").unwrap();
    let dst_path = matches.value_of("OUTPUT").unwrap();
    let width = matches.value_of("WIDTH").unwrap().parse::<u32>().unwrap();
    let height = matches.value_of("HEIGHT").unwrap().parse::<u32>().unwrap();

    let aspect = width as f32 / height as f32;

    let file = std::fs::File::open(&src_path).unwrap();
    let mut reader = BufReader::new(&file);
    let mesh = MeshBuilder::from_stl(&mut reader);
    let mut bounding_box = BoundingBox::new(&mesh);

    // Shift the model and its bounding box so that the bounding box is centered
    // on the origin.
    let model_translation = bounding_box.center_to_origin();
    bounding_box.shift(model_translation);

    let look_down_axis = bounding_box.largest_cross_section_axis();
    let camera_position = bounding_box.pick_camera_position(aspect, camera_fovy, &look_down_axis);
    let point_light_position = bounding_box.pick_light_position(&look_down_axis);

    let descrip = screenshot::ScreenshotDescriptor {
        mesh: &mesh,
        dst_path,
        width,
        height,
        model_translation,
        point_light_position,
        camera_position,
        camera_fovy,
    };
    futures::executor::block_on(screenshot::run(descrip));
}
