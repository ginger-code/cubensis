//This file is adapted from source code for crate "arcball": https://github.com/Twinklebear/arcball/blob/debb2ce19f7be54e609a08d52721afd52963c084/src/lib.rs
//Copied and modified to accommodate cgmath upgrades
use cgmath::num_traits::clamp;
use cgmath::prelude::*;
use cgmath::{Matrix4, Quaternion, Vector2, Vector3, Vector4};

/// The Arcball camera (Ken Shoemake)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ArcballCamera {
    pub translation: Matrix4<f32>,
    pub center_translation: Matrix4<f32>,
    pub rotation: Quaternion<f32>,
    pub camera: Matrix4<f32>,
    pub inv_camera: Matrix4<f32>,
    pub zoom_speed: f32,
    pub inv_screen: [f32; 2],
    pub perspective_projection: Matrix4<f32>,
    pub projection: Matrix4<f32>,
}

impl ArcballCamera {
    /// Create a new Arcball camera focused at the `center` point, which will zoom at `zoom_speed`
    /// `screen` should be `[screen_width, screen_height]`.
    pub fn new(center: Vector3<f32>, zoom_speed: f32, screen: [f32; 2]) -> ArcballCamera {
        let perspective_projection =
            cgmath::perspective(cgmath::Deg(80.0), screen[0] / screen[1], 0.01, 200.0);
        let mut cam = ArcballCamera {
            translation: Matrix4::from_translation(Vector3::new(
                f32::zero(),
                f32::zero(),
                -f32::one(),
            )),
            center_translation: Matrix4::from_translation(center).invert().unwrap(),
            rotation: Quaternion::new(f32::one(), f32::zero(), f32::zero(), f32::zero()),
            camera: Matrix4::<f32>::one(),
            inv_camera: Matrix4::<f32>::one(),
            zoom_speed,
            inv_screen: [f32::one() / screen[0], f32::one() / screen[1]],
            perspective_projection,
            projection: perspective_projection * Matrix4::<f32>::one(),
        };
        cam.update_camera();
        cam
    }
    /// Get the view matrix computed by the camera.
    pub fn get_mat4(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * self.camera
    }
    /// Get the inverse view matrix
    pub fn _get_inv_camera(&self) -> Matrix4<f32> {
        self.inv_camera
    }
    /// Get the camera eye position
    pub fn _eye_pos(&self) -> Vector3<f32> {
        Vector3::new(
            self.inv_camera[3].x,
            self.inv_camera[3].y,
            self.inv_camera[3].z,
        )
    }
    /// Get the camera view direction
    pub fn _eye_dir(&self) -> Vector3<f32> {
        let dir =
            self.inv_camera * Vector4::new(f32::zero(), f32::zero(), -f32::one(), f32::zero());
        Vector3::new(dir.x, dir.y, dir.z).normalize()
    }
    /// Get the camera view direction
    pub fn _up_dir(&self) -> Vector3<f32> {
        let dir = self.inv_camera * Vector4::new(f32::zero(), f32::one(), f32::zero(), f32::zero());
        Vector3::new(dir.x, dir.y, dir.z).normalize()
    }
    /// Rotate the camera, mouse positions should be in pixel coordinates.
    ///
    /// Rotates from the orientation at the previous mouse position specified by `mouse_prev`
    /// to the orientation at the current mouse position, `mouse_cur`.
    pub fn rotate(&mut self, mouse_prev: Vector2<f32>, mouse_cur: Vector2<f32>) {
        let m_cur = Vector2::new(
            clamp(mouse_cur.x * 2.0 * self.inv_screen[0] - 1.0, -1.0, 1.0),
            clamp(1.0 - 2.0 * mouse_cur.y * self.inv_screen[1], -1.0, 1.0),
        );
        let m_prev = Vector2::new(
            clamp(mouse_prev.x * 2.0 * self.inv_screen[0] - 1.0, -1.0, 1.0),
            clamp(1.0 - 2.0 * mouse_prev.y * self.inv_screen[1], -1.0, 1.0),
        );
        let mouse_cur_ball = ArcballCamera::screen_to_arcball(m_cur);
        let mouse_prev_ball = ArcballCamera::screen_to_arcball(m_prev);
        self.rotation = mouse_cur_ball * mouse_prev_ball * self.rotation;
        self.update_camera();
    }
    /// Zoom the camera in by some amount. Positive values zoom in, negative zoom out.
    pub fn zoom(&mut self, amount: f32, elapsed: f32) {
        let motion = Vector3::new(f32::zero(), f32::zero(), amount);
        self.translation =
            Matrix4::from_translation(motion * self.zoom_speed * elapsed) * self.translation;
        self.update_camera();
    }
    /// Pan the camera following the motion of the mouse. The mouse delta should be in pixels.
    pub fn pan(&mut self, mouse_delta: Vector2<f32>) {
        let zoom_dist = self.translation[3][3].abs();
        let delta = Vector4::new(
            mouse_delta.x * self.inv_screen[0],
            -mouse_delta.y * self.inv_screen[1],
            f32::zero(),
            f32::zero(),
        ) * zoom_dist;
        let motion = self.inv_camera * delta;
        self.center_translation =
            Matrix4::from_translation(Vector3::new(motion.x, motion.y, motion.z))
                * self.center_translation;
        self.update_camera();
    }
    /// Update the screen dimensions, e.g. if the window has resized.
    pub fn resize(&mut self, width: f32, height: f32) {
        self.inv_screen[0] = f32::one() / width;
        self.inv_screen[1] = f32::one() / height;
    }
    fn update_camera(&mut self) {
        self.camera = self.translation * Matrix4::from(self.rotation) * self.center_translation;
        self.inv_camera = self.camera.invert().unwrap();
        self.projection = self.perspective_projection * self.get_mat4();
    }
    fn screen_to_arcball(p: Vector2<f32>) -> Quaternion<f32> {
        let dist = cgmath::dot(p, p);
        // If we're on/in the sphere return the point on it
        if dist <= f32::one() {
            Quaternion::new(f32::zero(), p.x, p.y, f32::sqrt(f32::one() - dist))
        } else {
            let unit_p = p.normalize();
            Quaternion::new(f32::zero(), unit_p.x, unit_p.y, f32::zero())
        }
    }
}

unsafe impl bytemuck::Zeroable for ArcballCamera {}
unsafe impl bytemuck::Pod for ArcballCamera {}

// pub struct ArcballCameraRaw
#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);
