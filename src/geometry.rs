use bevy::prelude::*;


pub mod plane;
pub mod line;

pub struct GeometryPlugin;

impl Plugin for GeometryPlugin{
    fn build(&self, _app: &mut App) {
        
    }
}

pub struct Plane{
    ///A geometric plane defined using (p - zero_point) dot normal = 0
    pub normal: Vec3,
    pub zero_point: Vec3
}

pub struct Line{
    //A geometric line defined by p = direction * t + zero_point
    pub zero_point: Vec3,
    pub direction: Vec3
}