pub mod camera_controller;

pub struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect_ratio: f32,
    fov_y_radians: f32,
    z_near: f32,
    z_far: f32,
    opengl_to_wgpu_matrix: cgmath::Matrix4<f32>,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Camera {
            // x is your horizontal location <- ->
            // y is your vertical location V ^
            // z is how far away or close you are to obj
            eye: (0.0, 0.0, 1.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: (0.0, 1.0, 0.0).into(),
            aspect_ratio: width / height,
            fov_y_radians: 45.0,
            z_near: 0.1,
            z_far: 100.0,
            // opengl_to_wgpu_matrix: glam::Mat4::from_cols_array_2d(&[
            //     [1.0, 0.0, 0.0, 0.0],
            //     [0.0, 1.0, 0.0, 0.0],
            //     [0.0, 0.0, 0.5, 0.0],
            //     [0.0, 0.0, 0.5, 1.0],
            // ]),
            opengl_to_wgpu_matrix: cgmath::Matrix4::new(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.5, 0.0,
                0.0, 0.0, 0.5, 1.0,
            ),
        }
    }
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // Moves world to be at position & rotation of camera?
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);

        // Gives effect of depth
        // let proj = glam::::new(self.aspect, self.fovy, self.znear, self.zfar);

        let proj = cgmath::perspective(
            cgmath::Deg(self.fov_y_radians),
            self.aspect_ratio,
            self.z_near,
            self.z_far,
        );

        // Order matters! Proj before view
        self.opengl_to_wgpu_matrix * proj * view
    }
}
