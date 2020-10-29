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

        let adj = origin_center.dot(&ray.direction); 
        let d2 = origin_center.dot(&origin_center) - (adj*adj); 
        let radius2 = self.radius * self.radius; 

        if d2 > radius2{
            return (false, -1.0)
        }

        let thc = (radius2 - d2).sqrt(); 
        let t0 = adj - thc; 
        let t1 = adj + thc; 

        if t0 < 0.0 && t1 < 0.0{
            return (false, -1.0)
        }

        let distance = if t0 < t1 {t0} else {t1}; 
        return (true, distance)
    }
}