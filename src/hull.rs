use bevy::prelude::*;
use std::sync::Arc;

pub mod hull_shape;
pub mod clipping;

pub struct HullPlugin;

impl Plugin for HullPlugin{
    fn build(&self, _app: &mut App) {
    }
}


#[derive(Component)]
pub struct Hull{
    shape: Arc<hull_shape::HullShape>
}

impl TryFrom <Mesh> for Hull {
    type Error = hull_shape::into::HullShapeIntoError;

    fn try_from(value: Mesh) -> Result<Self, Self::Error> {
        return Ok(Hull { shape: Arc::new(value.try_into()?) });
    }
}

#[derive(Default)]
pub struct ClippedHull{
    shape: Arc<hull_shape::HullShape>,
    indices: Vec<clipping::ClippedIndex>,
    patch_vertices: Vec<Vec3>
}