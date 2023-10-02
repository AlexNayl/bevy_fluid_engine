use bevy::prelude::*;

pub mod into;

#[derive(Default)]
pub(super) struct HullVertex{
    pub(super) position: Vec3,
    pub(super) edge_indices: Vec<usize>,
    pub(super) face_indices: Vec<usize>
}

#[derive(Default)]
pub(super) struct HullEdge{
    pub(super) vertex_indexes: [usize;2],
    pub(super) face_indexes: [usize;2],
}

#[derive(Default)]
pub(super) struct HullFace{
    pub(super) edge_indexes : [usize;3],
    pub(super) vertex_indices: [usize;3]
}



#[derive(Default)]
pub struct HullShape{
    pub(super) vertices: Vec<HullVertex>,
    pub(super) edges: Vec<HullEdge>,
    pub(super) faces: Vec<HullFace>
}

