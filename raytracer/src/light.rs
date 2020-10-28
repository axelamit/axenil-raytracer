use crate::vector::Vector3; 
use crate::color::Color; 
use crate::point::Point; 
use crate::ray::Ray; 

pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color, 
    pub intensity: f64, 
}

impl DirectionalLight{
    pub fn new(direction: Vector3, color: Color, intensity: f64) -> DirectionalLight{
        DirectionalLight{
            direction: direction,
            color: color,
            intensity: intensity,
        }
    }
}

pub struct SphericalLight{
    pub center: Point, 
    pub radius: f64, 
    pub color: Color,
    pub intensity: f64, 
}

impl SphericalLight{
    pub fn new(center: Point, radius: f64, color: Color, intensity: f64) -> SphericalLight{
        SphericalLight{
            center: center,
            radius: radius,
            color: color,
            intensity: intensity,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> (bool, f64){
        let origin_center = self.center.subtract(&ray.origin).to_vector3();

        let a = ray.direction.dot(&ray.direction); 
        let b = 2.0*origin_center.dot(&ray.direction); 
        let c = origin_center.dot(&origin_center) - (self.radius*self.radius); 
        
        let discriminant = b*b - 4.0*a*c; 

        if discriminant < 0.0{
            (false, 0.0)
        }
        else{
            let intersect_distance = (-b - (discriminant.sqrt()))/(2.0*a);
            if intersect_distance < 0.0{
                return (true, intersect_distance.abs())
            }
            (false, intersect_distance)
        }
    }
}