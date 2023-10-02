use bevy::prelude::*;

pub mod buoyancy;
pub struct LiquidsPlugin;

impl Plugin for LiquidsPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(buoyancy::BuoyancyPlugin);
    }
}

#[derive(Component, Default)]
pub struct Liquid{
}



