use std::f32::consts::PI;

use animation::NpcAnimationState;
use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_tnua::{TnuaAnimatingState, prelude::*};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use bevy_trenchbroom::prelude::*;

use crate::third_party::bevy_trenchbroom::GetTrenchbroomModelPath as _;

use super::animation::AnimationPlayerAncestor;
mod ai;
mod animation;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((ai::plugin, animation::plugin));
    app.register_type::<Npc>();
}

#[derive(PointClass, Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
#[require(Transform, Visibility)]
#[model("models/fox/Fox.gltf")]
#[component(on_add = Self::on_add)]
pub(crate) struct Npc;

const NPC_RADIUS: f32 = 0.8;
const NPC_FLOAT_HEIGHT: f32 = 1.3;

impl Npc {
    fn on_add(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
        if world.is_scene_world() {
            return;
        }
        let model = world
            .resource::<AssetServer>()
            .load(format!("{}#Scene0", Self::model_path()));
        world
            .commands()
            .entity(entity)
            .insert((
                Npc,
                TrenchBroomGltfRotationFix,
                TransformInterpolation,
                Collider::capsule(NPC_RADIUS, 0.3),
                TnuaController::default(),
                TnuaAvian3dSensorShape(Collider::cylinder(NPC_RADIUS - 0.01, 0.0)),
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
                TnuaAnimatingState::<NpcAnimationState>::default(),
                AnimationPlayerAncestor,
            ))
            .with_child((
                SceneRoot(model),
                Transform::from_xyz(0.0, -NPC_FLOAT_HEIGHT, 0.0)
                    .with_rotation(Quat::from_rotation_y(PI)),
            ));
    }
}
