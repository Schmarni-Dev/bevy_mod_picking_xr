//! A minimal example of how to add picking support to your bevy app.

use bevy::{prelude::*, utils::Uuid};
use bevy_mod_picking::prelude::*;
use bevy_mod_picking_xr::raycast_backend::XrRaycastBackend;
use bevy_oxr::{
    resources::XrSession,
    xr_init::{xr_only, XrSetup},
    xr_input::{
        actions::{ActionHandednes, ActionType, SetupActionSets, XrActionSets, XrBinding},
        interactions::XRRayInteractor,
        trackers::{OpenXRController, OpenXRLeftController, OpenXRRightController, OpenXRTracker},
    },
    DefaultXrPlugins,
};
use picking_core::pointer::InputPress;

fn main() {
    App::new()
        .add_plugins(DefaultXrPlugins.set(low_latency_window_plugin()))
        .add_systems(Startup, spawn_controllers_example)
        // All you need to do is add the picking plugin, with your backend of choice enabled in the
        // cargo features. By default, the bevy_mod_raycast backend is enabled via the
        // `backend_raycast` feature.
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(XrRaycastBackend)
        .add_systems(Startup, setup)
        .add_systems(XrSetup, setup_xr_actions)
        .add_systems(Update, trigger.run_if(xr_only()))
        .run();
}

const ACTION_SET: &str = "bevy_mod_picking_xr";

fn setup_xr_actions(mut action_sets: ResMut<SetupActionSets>) {
    let bindings = &[XrBinding::new(
        "select",
        "/user/hand/right/input/trigger/value",
    )];
    let set = action_sets.add_action_set(ACTION_SET, "Bevy Mod Picking".into(), 0);
    set.new_action(
        "select",
        "Select".into(),
        ActionType::Bool,
        ActionHandednes::Single,
    );
    set.suggest_binding("/interaction_profiles/oculus/touch_controller", bindings);
}

fn trigger(
    action_sets: Res<XrActionSets>,
    session: Res<XrSession>,
    mut w: EventWriter<InputPress>,
    q: Query<&PointerId, With<XRRayInteractor>>,
    mut old: Local<bool>,
) {
    let new = action_sets
        .get_action_bool(ACTION_SET, "select")
        // Safe
        .unwrap()
        .state(&session, openxr::Path::NULL)
        // unsafe ig
        .unwrap()
        .current_state;

    if new && !*old {
        for p in &q {
            w.send(InputPress {
                pointer_id: *p,
                direction: pointer::PressDirection::Down,
                button: PointerButton::Primary,
            })
        }
    }
    if !new && *old {
        for p in &q {
            w.send(InputPress {
                pointer_id: *p,
                direction: pointer::PressDirection::Up,
                button: PointerButton::Primary,
            })
        }
    }
    *old = new;
}

fn spawn_controllers_example(mut commands: Commands, mut assets: ResMut<Assets<Image>>) {
    //left hand
    commands.spawn((
        OpenXRLeftController,
        OpenXRController,
        OpenXRTracker,
        SpatialBundle::default(),
    ));
    //right hand
    commands.spawn((
        OpenXRRightController,
        XRRayInteractor,
        PointerId::Custom(Uuid::new_v4()),
        PointerLocation {
            location: Some(pointer::Location {
                target: bevy::render::camera::NormalizedRenderTarget::Image(
                    assets.add(Image::default()),
                ),
                position: Vec2::ZERO,
            }),
        },
        OpenXRController,
        OpenXRTracker,
        SpatialBundle::default(),
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        PickableBundle::default(), // Optional: adds selection, highlighting, and helper components.
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        PickableBundle::default(), // Optional: adds selection, highlighting, and helper components.
    ));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, -4.0),
        ..default()
    });
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));
}
