use crate::point::Point; 
use crate::vector::Vector3; 
use crate::color::Color; 
use crate::material::Material; 
use crate::ray::Ray; 

pub struct Plane {
    pub point: Point,
    pub normal: Vector3,
    pub color: Color, 
    pub albedo: f64, 
    pub material: Material,
}

impl Plane{
    pub fn new(point: Point, normal: Vector3, color: Color, albedo: f64, material: Material) -> Plane{
        Plane{
            point: point,
            normal: normal, 
            color: color,
            albedo: albedo,
            material: material,
        }
    }

    pub fn normal(&self, _: &Point) -> Vector3{
        self.normal.neg()
    }

    pub fn intersect(&self, ray: &Ray) -> (bool, f64){
        let denominator = self.normal.dot(&ray.direction); 
        if denominator > 1e-6 {
            let v = self.point.subtract(&ray.origin).to_vector3(); 
            let distance = v.dot(&self.normal) / denominator; 
            if distance >= 0.0 {
                return (true, distance)
            }
        }
        (false, -1.0)
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