use bevy::prelude::*;

use avian_pickup::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::gameplay::player::default_input::{DropProp, PickupProp};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(pull_prop.param_warn_once());
    app.add_observer(throw_prop.param_warn_once());
    app.add_observer(drop_prop.param_warn_once());
}

fn pull_prop(
    _trigger: Trigger<Fired<PickupProp>>,
    actor: Single<Entity, With<AvianPickupActor>>,
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
) {
    avian_pickup_input_writer.send(AvianPickupInput {
        action: AvianPickupAction::Pull,
        actor: *actor,
    });
}

fn throw_prop(
    _trigger: Trigger<Started<PickupProp>>,
    actor: Single<Entity, With<AvianPickupActor>>,
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
) {
    avian_pickup_input_writer.send(AvianPickupInput {
        action: AvianPickupAction::Throw,
        actor: *actor,
    });
}

fn drop_prop(
    _trigger: Trigger<Started<DropProp>>,
    actor: Single<Entity, With<AvianPickupActor>>,
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
) {
    avian_pickup_input_writer.send(AvianPickupInput {
        action: AvianPickupAction::Drop,
        actor: *actor,
    });
}
