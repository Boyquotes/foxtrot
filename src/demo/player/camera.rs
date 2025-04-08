//! See <https://bevyengine.org/examples/camera/first-person-view-model/>

use avian3d::prelude::{PhysicsStepSet, TransformInterpolation};
use bevy::{
    color::palettes::tailwind, pbr::NotShadowCaster, prelude::*, render::view::RenderLayers,
};

use crate::screens::Screen;

use super::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_view_model);
    app.add_observer(add_render_layers_to_point_light);
    app.add_systems(PostUpdate, sync_with_player);
}

#[derive(Debug, Component)]
#[require(Transform, Visibility)]
pub(crate) struct PlayerCameraParent;

#[derive(Debug, Component)]
struct WorldModelCamera;

/// Used implicitly by all entities without a `RenderLayers` component.
/// Our world model camera and all objects other than the player are on this layer.
/// The light source belongs to both layers.
pub(crate) const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
pub(crate) const VIEW_MODEL_RENDER_LAYER: usize = 1;

#[derive(Debug, Component, Deref, DerefMut)]
pub(crate) struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(
            // These factors are just arbitrary mouse sensitivity values.
            // It's often nicer to have a faster horizontal sensitivity than vertical.
            // We use a component for them so that we can make them user-configurable at runtime
            // for accessibility reasons.
            // It also allows you to inspect them in an editor if you `Reflect` the component.
            Vec2::new(0.003, 0.002),
        )
    }
}

fn spawn_view_model(
    trigger: Trigger<OnAdd, Player>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    commands
        .spawn((
            PlayerCameraParent,
            CameraSensitivity::default(),
            StateScoped(Screen::Gameplay),
        ))
        .with_children(|parent| {
            parent.spawn((
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }),
            ));

            // Spawn view model camera.
            parent.spawn((
                Camera3d::default(),
                Camera {
                    // Bump the order to render on top of the world model.
                    order: 1,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    ..default()
                }),
                // Only render objects belonging to the view model.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));

            // Spawn the player's right arm.
            parent.spawn((
                Mesh3d(arm),
                MeshMaterial3d(arm_material),
                Transform::from_xyz(0.2, -0.1, -0.25),
                // Ensure the arm is only rendered by the view model camera.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                // The arm is free-floating, so shadows would look weird.
                NotShadowCaster,
            ));
        });
}

fn sync_with_player(
    player_camera_parent: Option<Single<&mut Transform, With<PlayerCameraParent>>>,
    player: Option<Single<&Transform, (With<Player>, Without<PlayerCameraParent>)>>,
) {
    if let Some(mut player_camera_parent) = player_camera_parent {
        if let Some(player) = player {
            player_camera_parent.translation = player.translation;
        }
    }
}

fn add_render_layers_to_point_light(trigger: Trigger<OnAdd, PointLight>, mut commands: Commands) {
    let entity = trigger.entity();
    commands.entity(entity).insert(RenderLayers::from_layers(&[
        DEFAULT_RENDER_LAYER,
        VIEW_MODEL_RENDER_LAYER,
    ]));
}
