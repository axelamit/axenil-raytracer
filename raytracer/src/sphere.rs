use crate::vector::Vector3;
use crate::point::Point;  
use crate::ray::Ray; 
use crate::color::*; 
use crate::material::Material; 

pub struct Sphere{
    pub center: Point,
    pub radius: f64, 
    pub color: Color, 
    pub albedo: f64, 
    pub material: Material,
} 

impl Sphere{
    pub fn new(center: Point, radius: f64, color: Color, albedo: f64, material: Material) -> Sphere{
        Sphere{
            center: center,
            radius: radius,
            color: color,
            albedo: albedo, 
            material: material, 
        }
    }

    pub fn normal(&self, point: &Point) -> Vector3{
        point.subtract(&self.center).to_vector3().normalize()
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