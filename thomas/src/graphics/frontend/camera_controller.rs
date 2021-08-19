use cgmath::Vector3;
pub struct CameraController {
    speed: f32,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }

    pub fn tick(&self, ctx: &mut crate::context::Context) {
        let camera = &mut ctx.graphics.camera;

        use cgmath::InnerSpace;

        // Pretty sure forward is the angle of where the camera is currently looking
        // Recall target is (0, 0, 0) (origin)
        // Then the eye is where the person is standing
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when camera gets too close to the
        // center of the scene.
        // Basically don't go beyond the origin
        if ctx.keyboard.plus && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if ctx.keyboard.minus {
            camera.eye -= forward_norm * self.speed;
        }

        // let right = forward_norm.cross(camera.up);
        let right = forward_norm.cross(camera.up);
        let up = Vector3::unit_y();

        // Redo radius calc in case the up/ down is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if ctx.keyboard.d {
            // Rescale the distance between the target and eye so
            // that it doesn't change. The eye therefore still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + (right * self.speed)).normalize() * forward_mag;
        }
        if ctx.keyboard.a {
            camera.eye = camera.target - (forward - (right * self.speed)).normalize() * forward_mag;
        }
        if ctx.keyboard.w {
            camera.eye = camera.target - (forward + (up * self.speed)).normalize() * forward_mag;
        }
        if ctx.keyboard.s {
            camera.eye = camera.target - (forward - (up * self.speed)).normalize() * forward_mag;
        }
    }
}
