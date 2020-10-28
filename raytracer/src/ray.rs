use crate::vector::Vector3;
use crate::point::Point;  

pub struct Ray{
    pub origin: Point,
    pub direction: Vector3, 
    pub recursion_limit: u64,
}

impl Ray{
    pub fn create_ray(origin: Point, direction: Vector3, recursion_limit: u64) -> Ray{
        Ray{
            origin: origin, 
            direction: direction, 
            recursion_limit: recursion_limit, 
        }
    }

    pub fn reflection(&self, normal: &Vector3, intersection: &Point) -> Ray{
        Ray{
            origin: intersection.add(&normal.multiply(0.00001).to_point()),
            direction: self.direction.subtract(&normal.multiply(self.direction.dot(&normal)*2.0)),
            recursion_limit: self.recursion_limit-1,
        }
    }

    pub fn copy(&self) -> Ray{
        Ray{
            origin: Point::new(self.origin.x, self.origin.y, self.origin.z),
            direction: Vector3::new(self.direction.x, self.direction.y, self.direction.z),
            recursion_limit: self.recursion_limit, 
        }
    }
}