use crate::point::Point; 
use crate::vector::Vector3; 
use crate::ray::Ray; 
use crate::material::Material; 
use crate::color::Color; 

use crate::sphere::Sphere; 
use crate::plane::Plane; 

pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element{
    pub fn normal(&self, point: &Point) -> Vector3{
        match self{
            Element::Sphere(s) => s.normal(point),
            Element::Plane(p) => p.normal(point),  
        }
    }

    pub fn intersect(&self, ray: &Ray) -> (bool, f64){
        match self{
            Element::Sphere(s) => s.intersect(ray),
            Element::Plane(p) => p.intersect(ray),
        }
    }

    pub fn get_material(&self) -> &Material{
        match self{
            Element::Sphere(s) => s.get_material(),
            Element::Plane(p) => p.get_material(),
        }
    }

    pub fn get_albedo(&self) -> f64{
        match self{
            Element::Sphere(s) => s.get_albedo(),
            Element::Plane(p) => p.get_albedo(),
        }
    }

    pub fn get_color(&self) -> &Color{
        match self{
            Element::Sphere(s) => s.get_color(),
            Element::Plane(p) => p.get_color(),
        }
    }
}