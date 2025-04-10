use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use super::Player;

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.add_input_context::<Player>() // All contexts should be registered.
        .add_observer(binding); // Add observer to setup bindings.
}

// All actions should implement the `InputAction` trait.
// It can be done manually, but we provide a derive for convenience.
// The only necessary parameter is `output`, which defines the output type.
#[derive(Debug, InputAction)]
#[input_action(output = Vec3)]
pub(crate) struct Move;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub(crate) struct Jump;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
pub(crate) struct Rotate;

fn binding(trigger: Trigger<Binding<Player>>, mut players: Query<&mut Actions<Player>>) {
    const DEFAULT_SPEED: f32 = 10.0;
    let mut actions = players.get_mut(trigger.entity()).unwrap();

    // Mappings like WASD or sticks are very common,
    // so we provide built-ins to assign all keys/axes at once.
    // We don't assign any conditions and in this case the action will
    // be triggered with any non-zero value.
    actions
        .bind::<Move>()
        .to((Cardinal::wasd_keys(), GamepadStick::Left))
        .with_modifiers((
            DeadZone::default(), // Apply non-uniform normalization to ensure consistent speed, otherwise diagonal movement will be faster.
            SmoothNudge::default(), // Make movement smooth and independent of the framerate. To only make it framerate-independent, use `DeltaScale`.
            Scale::splat(DEFAULT_SPEED), // Additionally multiply by a constant to achieve the desired speed.
            Negate::y(),
            SwizzleAxis::XZY,
        ));

    // Multiple inputs can be assigned to a single action,
    // and the action will respond to any of them.
    actions
        .bind::<Jump>()
        .to((KeyCode::Space, GamepadButton::South));

    const DEFAULT_SENSITIVITY: f32 = 0.002;
    actions
        .bind::<Rotate>()
        .to((Input::mouse_motion(), GamepadStick::Right))
        .with_modifiers((Negate::all(), Scale::splat(DEFAULT_SENSITIVITY)));
}
