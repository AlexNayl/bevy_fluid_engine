use bevy::{prelude::*, pbr::wireframe::{Wireframe, WireframePlugin},};
use bevy_fluid_engine::hull::*;
use bevy_fluid_engine::geometry::*;
use bevy_debug_camera::{DebugCamera, DebugCameraPlugin};

#[derive(Component)]
struct Target;
#[derive(Component)]
struct MovingPlane;

fn main(){
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(WireframePlugin);
    app.add_plugins(DebugCameraPlugin::default());
    app.add_systems(Startup, startup_system);
    app.add_systems(Update, move_plane_system);
    app.add_systems(Update, clip_system);
    app.run();
}



fn startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    //camera
    commands
        .spawn(Camera3dBundle::default())
        .insert(DebugCamera {
            position: Vec3::new(-5., 1., 0.),
            ..default()
        });

    //let mesh: Mesh = shape::Cube::default().into();
    let mesh:Mesh = shape::Cylinder::default().into();
    // let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![
    //     [0.0,0.0,1.0],
    //     [0.0,1.0,0.0],
    //     [0.0,0.0,-1.0]
    // ]);
    // mesh.set_indices(Some(Indices::U32(vec![
    //     0,1,2
    // ])));
   let hull_mesh_handle = meshes.add(mesh.clone());


    
    //Target
    commands.spawn(PbrBundle {
        mesh: hull_mesh_handle,
        material: materials.add(Color::BLUE.into()),
        transform: Transform::default(),
        ..default()
    }).insert(Target).insert(Hull::try_from(mesh).unwrap()).insert(Wireframe);

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
        material: materials.add(Color::rgba(0.3, 0.5, 0.3,0.2).into()),
        transform: Transform::default().looking_at(Vec3::Y, Vec3::Z),
        ..default()
    }).insert(MovingPlane);

    // light
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

fn move_plane_system(
    mut plane_query: Query<&mut Transform, With<MovingPlane>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>
){
    let mut plane_transform = plane_query.single_mut();
    const STEP_SIZE:f32 = 0.2;

    const UP_DIR : Vec3 = Vec3::Z;
    if keys.pressed(KeyCode::Equals){
        plane_transform.translation += UP_DIR * STEP_SIZE * time.delta_seconds();
    }else if keys.pressed(KeyCode::Minus){
        plane_transform.translation -= UP_DIR * STEP_SIZE * time.delta_seconds();
    }

}

fn clip_system(
    plane_query: Query<&Transform, With<MovingPlane>>,
    mut target_query: Query<(&Transform,&mut Handle<Mesh>,&Hull), With<Target>>,
    mut meshes: ResMut<Assets<Mesh>>
){
    //get geometric plane
    let plane_transform = plane_query.single();
    let (target_transform, mut target_handle, hull) = target_query.single_mut();

    let local_plane = Plane{
        zero_point : plane_transform.translation - target_transform.translation,
        normal : plane_transform.rotation.mul_vec3(Vec3::Y)
    };
    
    let clipped_hull = hull.clip_with_plane(&local_plane);

    let new_mesh : Mesh = Mesh::from(clipped_hull);
    target_handle.make_strong(&meshes);
    let _ = meshes.set(target_handle.as_ref(), new_mesh);
}