use bevy::prelude::*;

use crate::hull::*;
use crate::geometry::*;

#[test]
fn test_clipping(){
    let initial_mesh : Mesh = shape::Torus::default().into();
    let hull = Hull::try_from(initial_mesh.clone()).unwrap();

    let low_plane = Plane{
        normal: Vec3::Y,
        zero_point: Vec3 { x: 0.0, y: -100.0, z: 0.0 }
    };

    let result = hull.clip_with_plane(&low_plane);
    assert_eq!(result.indices.len(), 0, "Result should have no indices after clipping.");

    let high_plane = Plane{
        normal: Vec3::Y,
        zero_point: Vec3 { x: 0.0, y: 100.0, z: 0.0 }
    };

    let result = hull.clip_with_plane(&high_plane);

    assert_eq!(result.indices.len(), initial_mesh.indices().unwrap().len(), "Result should have same amount of indices as original.")
}