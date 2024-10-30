pub mod camera {
    use bevy::{input::ButtonInput, math::Vec3, prelude::*, render::camera::Camera};

    // A simple camera system for moving and zooming the camera.
    #[allow(dead_code)]
    pub fn movement(
        time: Res<Time>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    ) {
        for (mut transform, mut ortho) in query.iter_mut() {
            let mut direction = Vec3::ZERO;

            if keyboard_input.pressed(KeyCode::KeyA) {
                direction -= Vec3::new(1.0, 0.0, 0.0);
            }

            if keyboard_input.pressed(KeyCode::KeyD) {
                direction += Vec3::new(1.0, 0.0, 0.0);
            }

            if keyboard_input.pressed(KeyCode::KeyW) {
                direction += Vec3::new(0.0, 1.0, 0.0);
            }

            if keyboard_input.pressed(KeyCode::KeyS) {
                direction -= Vec3::new(0.0, 1.0, 0.0);
            }

            if keyboard_input.pressed(KeyCode::KeyZ) || keyboard_input.pressed(KeyCode::BracketLeft)
            {
                ortho.scale *= 1.01;
            }

            if keyboard_input.pressed(KeyCode::KeyX)
                || keyboard_input.pressed(KeyCode::BracketRight)
            {
                ortho.scale /= 1.01;
            }

            ortho.scale = ortho.scale.clamp(0.05, 5.0);

            let z = transform.translation.z;
            transform.translation += time.delta_seconds() * direction * 150.;
            // Important! We need to restore the Z values when moving the camera around.
            // Bevy has a specific camera setup and this can mess with how our layers are shown.
            transform.translation.z = z;
        }
    }
}
