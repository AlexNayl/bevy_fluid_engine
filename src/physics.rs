use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub mod liquids;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(liquids::LiquidsPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, startup_system);
    }
}


fn startup_system(mut config : ResMut<RapierConfiguration>){
    config.gravity = Vec3{
        x:0.0, y:0.0, z:-9.8
    }
}