use crate::vector::Vector3;
use crate::point::Point;  

pub struct Ray{
    pub origin: Point,
    pub direction: Vector3, 
}

impl Ray{
    pub fn create_ray(origin: Point, direction: Vector3) -> Ray{
        Ray{
            origin: origin, 
            direction: direction, 
        }
    }

    pub fn copy(&self) -> Ray{
        Ray{
            origin: Point::new(self.origin.x, self.origin.y, self.origin.z),
            direction: Vector3::new(self.direction.x, self.direction.y, self.direction.z),
        }
    }
}