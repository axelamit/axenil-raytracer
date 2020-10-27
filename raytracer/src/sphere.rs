//use crate::vector::Vector3;
use crate::point::Point;  
use crate::ray::Ray; 

pub struct Sphere{
    center: Point,
    radius: f64, 
} 

impl Sphere{
    pub fn new(center: Point, radius: f64) -> Sphere{
        Sphere{
            center: center,
            radius: radius,
        }
    }

    pub fn intersect(&self, ray: Ray) -> bool{
        let origin_center = self.center.subtract(ray.origin).to_vector3();

        let a = ray.direction.dot(&ray.direction); 
        let b = 2.0*origin_center.dot(&ray.direction); 
        let c = origin_center.dot(&origin_center) - (self.radius*self.radius); 
        
        let discriminant = b*b - 4.0*a*c; 

        if discriminant < 0.0{
            false
        }
        else{
            let _intersection = (-b - (discriminant.sqrt()))/(2.0*a); 
            true
        }
    }
}