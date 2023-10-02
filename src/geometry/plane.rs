use bevy::prelude::*;
use bevy::utils::thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum PlaneIntersectionError{
    #[error("Infinite intersections.")]
    Enclosed,
    #[error("No intersections.")]
    Parallel
}

// impl Error for PlaneIntersectionError {
    
// }

impl super::Plane{
    pub fn distance_from_plane<T : Into<Vec3>>(&self, point:T)->f32{
        let n = self.normal.normalize();
        return (point.into() - self.zero_point).dot(n)
    }

    pub fn intersection_from_line(&self, line: super::Line) -> Result<Vec3, PlaneIntersectionError>{
        let divider = line.direction.dot(self.normal);
        let numerator = (self.zero_point - line.zero_point).dot(self.normal);
        if divider == 0.0{
            //line is parallel to plane, no intersection possible
            if numerator == 0.0 && line.direction.dot(self.normal) == 0.0{
                //line is fully inside the pane, infinite intersections
                return Err(PlaneIntersectionError::Enclosed);
            }else{
                //line is parallel outside the plane, no intersections possible
                return Err(PlaneIntersectionError::Parallel);
            }
        }else{
            //intersection guaranteed, calculate and return
            let t = numerator / divider;
            return Ok(line.zero_point + line.direction * t);
        }
        
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_function(){
        let plane = crate::geometry::Plane{
            normal: Vec3::Y,
            zero_point: Vec3::ZERO
        };

        assert_eq!(plane.distance_from_plane(Vec3::Y), 1.0);
        assert_eq!(plane.distance_from_plane(Vec3::NEG_Y), -1.0);
    }

    #[test]
    fn test_intersection(){
        let plane = crate::geometry::Plane{
            normal: Vec3::Y,
            zero_point: Vec3::ZERO
        };

        let valid_line1 = crate::geometry::Line{zero_point: Vec3{x:-1.5, y:23.0, z:7.0}, direction: Vec3::Y};
        assert_eq!(plane.intersection_from_line(valid_line1).unwrap(), Vec3{x:-1.5, y:0.0, z:7.0});

        let valid_line2 = crate::geometry::Line{zero_point: Vec3::NEG_X, direction: Vec3{x:3.0,y:-11.4,z:2.3}};
        assert_eq!(plane.intersection_from_line(valid_line2).unwrap(), Vec3::NEG_X);

        let enclosed_line = crate::geometry::Line{zero_point: Vec3::ZERO, direction: Vec3::Z};
        assert_eq!(plane.intersection_from_line(enclosed_line), Err(PlaneIntersectionError::Enclosed));

        let parallel_line = crate::geometry::Line{zero_point: Vec3::Y, direction: Vec3::X};
        assert_eq!(plane.intersection_from_line(parallel_line), Err(PlaneIntersectionError::Parallel));
    }
}