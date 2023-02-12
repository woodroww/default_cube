use bevy::{pbr::NotShadowCaster, prelude::*, gltf::GltfMesh};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_mod_picking::*;
use camera::{CameraPlugin, PanOrbitCamera};

mod camera;

#[derive(Component)]
struct Clickable;

#[derive(Resource)]
struct AppAssets; 

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Default Cube".to_string(),
                width: 800.0,
                height: 800.0,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin)
        //.add_plugin(EguiPlugin) // either EguiPlugin or WorldInspectorPlugin, not both
        .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
        .add_system(check_load)
        .add_startup_system(spawn_cubes)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_axis)
        .add_plugin(CameraPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin)
        .add_system(keyboard_system)
        .add_system(egui_check)
        .run();
}

/// Disable bevy_mod_picking plugin if the cursor is in a egui window
fn egui_check(
    mut state: ResMut<PickingPluginsState>,
    mut egui_context: ResMut<EguiContext>,
) {
    let ctx = egui_context.ctx_mut();
    let pointer_over_area = ctx.is_pointer_over_area();
    let using_pointer = ctx.is_using_pointer();
    let wants_pointer = ctx.wants_pointer_input();
    if wants_pointer || pointer_over_area || using_pointer {
        state.enable_picking = false;
        state.enable_highlighting = false;
        state.enable_interacting = false;
    } else {
        state.enable_picking = true;
        state.enable_highlighting = true;
        state.enable_interacting = true;
    }
}

fn load_assets(mut commands: Commands, _asset_server: Res<AssetServer>) {
    //let mesh = asset_server.load("cam.glb#Mesh0");
    commands.insert_resource(AppAssets);
}

fn check_load() {}
/*
fn check_load(
    mut commands: Commands,
    app_assets: Res<AppAssets>,
    asset_server: Res<AssetServer>,
    //mut mesh_assets: ResMut<Assets<Mesh>>,
    //mut image_assets: ResMut<Assets<Image>>,
    gltf_assets: ResMut<Assets<GltfMesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut loaded: Local<bool>,
) {
    use bevy::asset::LoadState;
    if !*loaded && asset_server.get_load_state(app_assets.something.clone()) == LoadState::Loaded {
        let gltf_mesh = gltf_assets.get(&app_assets.something.clone()).unwrap();
        let bevy_mesh = gltf_mesh.primitives[0].mesh.clone();
        let bevy_material = match gltf_mesh.primitives[0].material.clone() {
            Some(material) => material,
            None => materials.add(Color::rgb(0.1, 0.1, 1.0).into()),
        };
        *loaded = true;
    }
}
*/

fn keyboard_system(
    _keyboard_input: Res<Input<KeyCode>>,
) {
    //if keyboard_input.just_pressed(KeyCode::Space) {
    //}
}

fn spawn_camera(mut commands: Commands) {
    let focus: Vec3 = Vec3::ZERO;

    let mut transform = Transform::default();
    transform.translation = Vec3 {
        x: -2.0,
        y: 2.5,
        z: 5.0,
    };
    transform.look_at(focus, Vec3::Y);

    let camera = Camera3dBundle {
        transform,
        ..Default::default()
    };

    commands.spawn((
        camera,
        PanOrbitCamera {
            radius: (transform.translation - focus).length(),
            focus,
            ..Default::default()
        },
        PickingCameraBundle::default(),
        bevy_transform_gizmo::GizmoPickSource::default(),
    ));
}

fn spawn_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // asset Highlighting for each entity
    // resource DefaultHighlighting global default

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(-1.0, 0.0, 0.0),
            ..default()
        },
        Highlighting {
            initial: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            hovered: Some(materials.add(Color::rgb(0.9, 0.8, 0.7).into())),
            pressed: Some(materials.add(Color::rgb(0.0, 1.0, 0.0).into())),
            selected: Some(materials.add(Color::rgb(0.7, 0.6, 0.5).into())),
        },
        PickableBundle::default(),
        bevy_transform_gizmo::GizmoTransformable,
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.5, 0.1, 0.1).into()),
            transform: Transform::from_xyz(1.0, 0.0, 0.0),
            ..default()
        },
        PickableBundle::default(),
        bevy_transform_gizmo::GizmoTransformable,
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

#[derive(Component)]
struct VisibleAxis;

fn spawn_axis(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let length = 20.0;
    let width = 0.05;
    let x = shape::Box::new(length, width, width);
    let y = shape::Box::new(width, length, width);
    let z = shape::Box::new(width, width, length);

    let empty_transform = Transform::from_translation(Vec3::ZERO);
    let empty: Entity = commands
        .spawn_empty()
        .insert(TransformBundle::from_transform(empty_transform))
        .insert(Visibility { is_visible: false })
        .insert(ComputedVisibility::default())
        .insert(Name::from("Main Axis"))
        .id();

    let mut transform = Transform::default();
    transform.translation.x = length / 2.0;

    commands.entity(empty).add_children(|commands| {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(x)),
                material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                transform,
                ..default()
            },
            NotShadowCaster,
            VisibleAxis,
        ));
        let mut transform = Transform::default();
        transform.translation.y = length / 2.0;
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(y)),
                material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                transform,
                ..default()
            },
            NotShadowCaster,
            VisibleAxis,
        ));
        let mut transform = Transform::default();
        transform.translation.z = length / 2.0;
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(z)),
                material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
                transform,
                ..default()
            },
            NotShadowCaster,
            VisibleAxis,
        ));
    });
}
