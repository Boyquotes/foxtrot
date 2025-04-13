use crate::third_party::avian3d::CollisionLayer;
use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_tnua::TnuaNotPlatform;
use bevy_trenchbroom::{class::QuakeClass, prelude::*};

use super::assets::LevelAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(add_not_platform_to_props);
}

macro_rules! dynamic_prop {
    ($name:ident, $model:expr) => {
        #[derive(PointClass, Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
        #[reflect(Component)]
        #[require(Transform, Visibility)]
        #[model($model)]
        #[component(on_add = on_add_dynamic_prop::<$name>)]
        pub(crate) struct $name;
    };
}

dynamic_prop!(Book, "models/book/book.gltf");
dynamic_prop!(Plate, "models/plate/plate.gltf");
dynamic_prop!(Mug, "models/mug/mug.gltf");

fn on_add_dynamic_prop<T: QuakeClass>(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
    if world.is_scene_world() {
        return;
    }
    let model = world.resource::<LevelAssets>().model_for_class::<T>();
    world.commands().entity(entity).insert((
        TrenchBroomGltfRotationFix,
        TransformInterpolation,
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh)
            .with_default_layers(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))
            // About the density of oak wood (600-800 kg/m^3)
            .with_default_density(800.0),
        RigidBody::Dynamic,
        SceneRoot(model),
        TnuaNotPlatform,
    ));
}

fn add_not_platform_to_props(
    trigger: Trigger<OnAdd, ColliderParent>,
    mut commands: Commands,
    q_collider_parent: Query<&ColliderParent>,
    q_tnua_not_platform: Query<&TnuaNotPlatform>,
) {
    let parent = q_collider_parent.get(trigger.entity()).unwrap();
    if !q_tnua_not_platform.contains(parent.get()) {
        return;
    }
    commands.entity(trigger.entity()).insert(TnuaNotPlatform);
}
