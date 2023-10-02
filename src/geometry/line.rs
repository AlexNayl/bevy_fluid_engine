use bevy::prelude::*;

impl super::Line {
    pub fn from_two_points(p1:&Vec3, p2:&Vec3) -> Self{
        Self { zero_point: *p1, direction: *p2 - *p1 }
    }
}