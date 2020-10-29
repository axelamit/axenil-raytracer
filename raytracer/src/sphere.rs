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

        let projection_length2 = origin_center.dot(&ray.direction); 
        let perpendicular2 = origin_center.dot(&origin_center) - (projection_length2*projection_length2); 
        let radius2 = self.radius*self.radius; 

        if perpendicular2 > radius2{
            return (false, -1.0)
        }

        let add = (radius2 - perpendicular2).sqrt(); 
        let intersection0 = projection_length2 - add; 
        let intersection1 = projection_length2 + add; 

        if intersection0 < 0.0 && intersection1 < 0.0{
            return (false, -1.0)
        }

        let distance = if intersection0 < intersection1 {intersection0} else {intersection1}; 
        return (true, distance)
    }

    pub fn get_material(&self) -> &Material{
        &self.material
    }

    pub fn get_albedo(&self) -> f64{
        self.albedo
    }

    pub fn get_color(&self) -> &Color{
        &self.color
    }
}