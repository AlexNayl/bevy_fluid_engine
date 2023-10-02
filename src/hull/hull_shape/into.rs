use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}};
use itertools::Itertools;
use bevy::utils::thiserror::Error;
use crate::hull::{ClippedHull, clipping::ClippedIndex};

use super::*;

impl HullVertex {
    fn add_face(&mut self, face_index: usize){
        for other_face_vertex in self.face_indices.iter(){
            if face_index == *other_face_vertex{
                return;
            }
        }

        self.face_indices.push(face_index);
    }
}

impl Into<Mesh> for HullShape{
    fn into(self) -> bevy::prelude::Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        
        //get positions as a vector
        let mut positions = Vec::new();

        for vertex in self.vertices.iter(){
            positions.push([vertex.position.x, vertex.position.y, vertex.position.z]);
        }
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

        //build face indexes
        let mut indices = Vec::new();
        for face in self.faces.iter(){
            for edge_index in face.edge_indexes.iter(){
                let edge = self.edges.get(*edge_index).unwrap();
                indices.push(edge.vertex_indexes[0] as u32);
            }

        }

        mesh.set_indices(Some(Indices::U32(indices)));


        return mesh;
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum HullShapeIntoError{
    #[error("Can only be used on triangle mesh topologies.")]
    WrongTopologyType,
    #[error("Mesh must contain position attribute.")]
    MissingPositionAttribute
}

// Conversion from mesh into HullShape
impl TryInto<HullShape> for Mesh{
    type Error = HullShapeIntoError;

    fn try_into(self) -> Result<HullShape, Self::Error>{
        // Sanity Checks
        if self.primitive_topology() != PrimitiveTopology::TriangleList{
            return Err(HullShapeIntoError::MissingPositionAttribute);
        }else if !self.contains_attribute(Mesh::ATTRIBUTE_POSITION){
            return Err(HullShapeIntoError::MissingPositionAttribute);
        }

        //----process vertices
        //get vertices positions
        let vertices = self.attribute(Mesh::ATTRIBUTE_POSITION).expect("Mesh does not have position attribute")
        .as_float3().expect("Vertex attribute format error.");

        //get faces
        let faces = self.indices().expect("Does not contain faces").iter().chunks(3);

        let mut output = HullShape::default();

        //import vertices
        for vertex in vertices{
            output.vertices.push(
                HullVertex{
                    position: Vec3::from_array(*vertex),
                    ..default()
                }
            );
        }

        //----face processing
        for face in &faces{
            //first, extract the vertex indices from the face chunk
            let mut face_vertices:[usize;3] = [0,0,0];
            let mut count:usize = 0;
            for vertex_index in face{
                face_vertices[count] = vertex_index;
                count += 1;
            }
            if count != 3{
                //edge case, probably should not happen
                continue
            }

            let this_face_index = output.faces.len();

            let mut face_edge_indexes = [0,0,0];
            //-----edge processing, check if the edge already exists, otherwise create it
            for i in 0..3{
                

                let vertex1_index = face_vertices[i];
                let vertex2_index = face_vertices[(i+1)%3]; //wraps around if at the end
                let vertex1 = output.vertices.get_mut(vertex1_index).unwrap();


                // also add the face to the vertices while iterating
                vertex1.add_face(this_face_index);

                let mut found_edge = false;
                //try to find if its an existing edge
                for edge_index in vertex1.edge_indices.iter(){
                    let edge = output.edges.get(*edge_index).unwrap();
                    if edge.face_indexes[0] == vertex2_index || edge.face_indexes[1] == vertex2_index{
                        found_edge = true;
                        face_edge_indexes[i] = *edge_index;
                    }
                }
                if !found_edge{
                    //edge doesn't exist yet, create it
                    face_edge_indexes[i] = output.edges.len();
                    output.edges.push(
                        HullEdge { vertex_indexes: [vertex1_index, vertex2_index], face_indexes: [this_face_index,0], ..default() }
                    );
                }

            }

            //create the new face
            output.faces.push(
                HullFace { edge_indexes: face_edge_indexes, vertex_indices: face_vertices}
            );
        } 

        return Ok(output);
    }
}

impl From <ClippedHull> for Mesh{
    fn from(value: ClippedHull) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        
        //get positions as a vector
        let mut positions = Vec::<[f32;3]>::new();
        let shape = value.shape.clone();

        //load vertices into the mesh
        for vertex in shape.vertices.iter(){
            positions.push([vertex.position.x,vertex.position.y,vertex.position.z]);
        }
        for vertex in value.patch_vertices.iter(){
            positions.push([vertex.x, vertex.y, vertex.z])
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

        //build face indices and load them into the mesh
        let mut indices = Vec::<u32>::new();
        for clipped_index in value.indices{
            if let ClippedIndex::OriginalIndex(index) = clipped_index{
                indices.push(index as u32);
            }else if let ClippedIndex::PatchIndex(index) = clipped_index{
                indices.push((index + shape.vertices.len()) as u32);
            }
        }

        mesh.set_indices(Some(Indices::U32(indices)));

        return mesh;
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_into(){
        //take a complex mesh, convert it into a hull, then convert it back
        let original_mesh: Mesh = shape::Torus::default().into();

        let hull : HullShape = original_mesh.clone().try_into().unwrap();

        //make sure edges don't have the same vertices on both ends
        for edge in hull.edges.iter(){
            assert_ne!(edge.vertex_indexes[0], edge.vertex_indexes[1]);
        }

        let new_mesh : Mesh = hull.into();

        assert_eq!(original_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap(), new_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap());
        assert_eq!(original_mesh.indices().unwrap().iter().collect_vec(), new_mesh.indices().unwrap().iter().collect_vec());
    }
}

