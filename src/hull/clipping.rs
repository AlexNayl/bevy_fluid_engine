use crate::geometry::Line;

use super::*;

#[cfg(test)]
mod tests;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum ClippedIndex{
    OriginalIndex(usize),
    PatchIndex(usize),
}

#[derive(PartialEq)]
enum ProcessedEdge{
    FullySubmerged,
    FullyClipped,
    PartialClip{
        original_vertex_index:usize,
        patch_vertex_index:usize
    }
}


impl Hull{
    pub fn clip_with_plane(&self, plane: &crate::geometry::Plane) -> ClippedHull{
        //Returns the sliced hull below the cut plane
        let shape = self.shape.as_ref();

        //contains the result
        let mut result = ClippedHull{shape: self.shape.clone(), ..Default::default()};

        //used later during face processing, initializes all to 0
        let mut  face_submerged_vertices_count = Vec::<u8>::new();
        face_submerged_vertices_count.resize_with(shape.faces.len(), ||{0 as u8});
        
        //----Vertex Processing
        let mut submerged_vertices = Vec::<bool>::with_capacity(self.shape.vertices.len());
        for vertex in self.shape.vertices.iter(){
            if plane.distance_from_plane(vertex.position) < 0.0{
                submerged_vertices.push(true);

                for face_index in vertex.face_indices.iter(){
                    //tracks how many vertices of a face are submerged, used during face processing
                    *face_submerged_vertices_count.get_mut(*face_index).unwrap() += 1;

                }
            }else{
                submerged_vertices.push(false);
            }

        }

        // --- Edge Processing
        let mut processed_edges: Vec<ProcessedEdge> = Vec::with_capacity(self.shape.edges.len());
        for edge in self.shape.edges.iter(){
            let [vertex_1_index,vertex_2_index] = edge.vertex_indexes;
            let vertex_1 = &shape.vertices[vertex_1_index];
            let vertex_2 = &shape.vertices[vertex_2_index];

            if submerged_vertices[vertex_1_index] ^ submerged_vertices[vertex_2_index]{
                //This edge is split between the plane
                
                let edge_line = Line::from_two_points(&vertex_1.position, &vertex_2.position);
                let intersection = plane.intersection_from_line(edge_line);
                if let Ok(point) = intersection{
                    let patch_index = result.patch_vertices.len();
                    result.patch_vertices.push(point.clone());
                    if submerged_vertices[vertex_1_index]{
                        processed_edges.push(ProcessedEdge::PartialClip { original_vertex_index: vertex_1_index, patch_vertex_index: patch_index });
                    }else{
                        processed_edges.push(ProcessedEdge::PartialClip { original_vertex_index: vertex_2_index, patch_vertex_index: patch_index });
                    }
                }else{
                    //Split unsuccessful, assume edge runs parallel and inside plane
                    processed_edges.push(ProcessedEdge::FullySubmerged);
                }


            }else if submerged_vertices[vertex_1_index] & submerged_vertices[vertex_2_index]{
                processed_edges.push(ProcessedEdge::FullySubmerged);
            }else{
                processed_edges.push(ProcessedEdge::FullyClipped);
            }
        }

        // --- Face Processing
        let mut current_face_poly_line_indices = Vec::<ClippedIndex>::with_capacity(4);
        
        for (face_index, face) in shape.faces.iter().enumerate(){
            let submerged_vertices_count = *face_submerged_vertices_count.get(face_index).unwrap();
            
            if submerged_vertices_count == 1 || submerged_vertices_count == 2 {
                current_face_poly_line_indices.clear();
                //generate the poly line
                
                for i in 0..3{
                    let current_vertex_index = face.vertex_indices[i];
                    let current_edge_index = face.edge_indexes[i];
                    let previous_edge_index = face.edge_indexes[(i+2)%3];
                    let next_edge_index = face.edge_indexes[(i+1)%3];

                    let current_processed_edge = &processed_edges[current_edge_index];

                    match current_processed_edge{
                        ProcessedEdge::FullySubmerged=>{
                            current_face_poly_line_indices.push(ClippedIndex::OriginalIndex(current_vertex_index));
                        },
                        ProcessedEdge::PartialClip { original_vertex_index:_, patch_vertex_index } =>{
                            if submerged_vertices[current_vertex_index]{
                                current_face_poly_line_indices.push(ClippedIndex::OriginalIndex(current_vertex_index));
                                if let ProcessedEdge::PartialClip { original_vertex_index:_, patch_vertex_index:_ } = processed_edges[next_edge_index]{
                                    current_face_poly_line_indices.push(ClippedIndex::PatchIndex(*patch_vertex_index));
                                }
                            }else{
                                current_face_poly_line_indices.push(ClippedIndex::PatchIndex(*patch_vertex_index));
                                
                            }
                        },
                        ProcessedEdge::FullyClipped =>{
                            if let ProcessedEdge::PartialClip { original_vertex_index:_, patch_vertex_index } = processed_edges[previous_edge_index]{
                                current_face_poly_line_indices.push(ClippedIndex::PatchIndex(patch_vertex_index));
                            }
                        }
                    }
                }

            }

            

            match submerged_vertices_count{
                1 => {
                    assert_eq!(current_face_poly_line_indices.len(),3);
                    for new_index in current_face_poly_line_indices.iter(){
                        result.indices.push(*new_index);
                    }
                },
                2 => {
                    //four side face with two patch vertices
                    result.indices.push(current_face_poly_line_indices[0]);
                    result.indices.push(current_face_poly_line_indices[1]);
                    result.indices.push(current_face_poly_line_indices[2]);

                    result.indices.push(current_face_poly_line_indices[0]);
                    result.indices.push(current_face_poly_line_indices[2]);
                    result.indices.push(current_face_poly_line_indices[3]);
                },
                3 => {
                    //whole face is submerged
                    for vertices_index in face.vertex_indices{
                        result.indices.push(ClippedIndex::OriginalIndex(vertices_index));
                    }
                },
                _ => () //default case, probably completely clipped
            }
        }
        return result;
    }
}