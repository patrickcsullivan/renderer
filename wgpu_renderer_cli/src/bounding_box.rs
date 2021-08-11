use cgmath::{Angle, Vector3};

pub enum Axis {
    X,
    Y,
    Z,
}

pub struct BoundingBox {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub z_min: f32,
    pub z_max: f32,
}

impl BoundingBox {
    pub fn new(mesh: &nom_stl::Mesh) -> BoundingBox {
        let mut bounding_box = BoundingBox {
            x_min: 0.0,
            x_max: 0.0,
            y_min: 0.0,
            y_max: 0.0,
            z_min: 0.0,
            z_max: 0.0,
        };

        for triangle in mesh.triangles() {
            for vertex in triangle.vertices().iter() {
                // Shif the coordinates around since the renderer will rotate
                // the model -90 degrees around the x axis.
                let (x, y, z) = (vertex[0], vertex[2], -1.0 * vertex[1]);
                if x < bounding_box.x_min {
                    bounding_box.x_min = x;
                }
                if x > bounding_box.x_max {
                    bounding_box.x_max = x;
                }
                if y < bounding_box.y_min {
                    bounding_box.y_min = y;
                }
                if y > bounding_box.y_max {
                    bounding_box.y_max = y;
                }
                if z < bounding_box.z_min {
                    bounding_box.z_min = z;
                }
                if z > bounding_box.z_max {
                    bounding_box.z_max = z;
                }
            }
        }

        bounding_box
    }

    pub fn shift(&mut self, vec: Vector3<f32>) {
        self.x_min += vec.x;
        self.x_max += vec.x;
        self.y_min += vec.y;
        self.y_max += vec.y;
        self.z_min += vec.z;
        self.z_max += vec.z;
    }

    pub fn dx(&self) -> f32 {
        self.x_max - self.x_min
    }

    pub fn dy(&self) -> f32 {
        self.y_max - self.y_min
    }

    pub fn dz(&self) -> f32 {
        self.z_max - self.z_min
    }

    pub fn center(&self) -> cgmath::Point3<f32> {
        let x = self.x_min + self.dx() / 2.0;
        let y = self.y_min + self.dy() / 2.0;
        let z = self.z_min + self.dz() / 2.0;
        cgmath::Point3::new(x, y, z)
    }

    pub fn center_to_origin(&self) -> cgmath::Vector3<f32> {
        let origin = cgmath::Point3::new(0.0, 0.0, 0.0);
        origin - self.center()
    }

    /// Returns the axis along which the bounding box has the larget cross
    /// section area.
    pub fn largest_cross_section_axis(&self) -> Axis {
        let x_axis_area = self.dy() * self.dz();
        let y_axis_area = self.dx() * self.dz();
        let z_axis_area = self.dx() * self.dy();

        if x_axis_area > z_axis_area {
            Axis::X
        } else {
            Axis::Z
        }
        // if y_axis_area > x_axis_area && y_axis_area > z_axis_area {
        //     Axis::Y
        // } else if x_axis_area > z_axis_area {
        //     Axis::X
        // } else {
        //     Axis::Z
        // }
    }

    /// Returns the width and height of the bounding box cross section when
    /// viewed along the given axis.
    pub fn visible_size(&self, axis: &Axis) -> (f32, f32) {
        match axis {
            Axis::X => (self.dz(), self.dy()),
            Axis::Y => (self.dx(), self.dz()),
            Axis::Z => (self.dx(), self.dy()),
        }
    }

    /// Returns a position on the given axis where the camera will be filled
    /// with the part at the origin.
    pub fn pick_camera_position(
        &self,
        aspect: f32,
        fovy: cgmath::Deg<f32>,
        axis: &Axis,
    ) -> cgmath::Point3<f32> {
        let (visible_width, visible_height) = self.visible_size(axis);
        let theta = cgmath::Deg(90.0) - fovy / 2.0;
        let target_height = if aspect >= visible_width / visible_height {
            visible_height
        } else {
            1.0 / aspect * visible_width
        };
        let camera_to_box = theta.sin() * target_height;
        match axis {
            Axis::X => cgmath::Point3::new(camera_to_box + self.dx(), 0.0, 0.0),
            Axis::Y => cgmath::Point3::new(0.0, camera_to_box + self.dy(), 0.0),
            Axis::Z => cgmath::Point3::new(0.0, 0.0, camera_to_box + self.dz()),
        }
    }

    pub fn pick_light_position(&self, axis: &Axis) -> cgmath::Point3<f32> {
        match axis {
            Axis::X => cgmath::Point3::new(self.dx(), self.dy(), 0.0),
            Axis::Y => cgmath::Point3::new(0.0, self.dy(), -1.0 * self.dz()),
            Axis::Z => cgmath::Point3::new(0.0, self.dy(), self.dz()),
        }
    }
}
