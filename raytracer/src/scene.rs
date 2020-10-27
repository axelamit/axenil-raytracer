use crate::vector::Vector3;
use crate::point::Point;  
use crate::ray::Ray; 
use crate::sphere::Sphere; 

use image; 
use image::{DynamicImage, GenericImage, Pixel, Rgba, ImageFormat};

pub struct Scene{
    width: u32,
    height: u32,
    spheres: Vec<Sphere>,
}

impl Scene{
    pub fn new() -> Scene {
        Scene{
            width: 1000,
            height: 1000,
            spheres: Vec::new(),
        }
    }

    pub fn test_scene(&mut self) {
        let sphere1 = Sphere::new(Point::new(0.0, 0.0, -5.0), 2.0); 
        self.add_sphere(sphere1); 
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere); 
    }

    pub fn render(&self) {
        let mut image = DynamicImage::new_rgb8(self.width, self.height); 
        for y in 0..self.height{
            for x in 0..self.width{
                let trans_x = (x as f64/self.width as f64)*2.0 - 1.0; 
                let trans_y = (y as f64/self.height as f64)*2.0 - 1.0;

                let ray_origin = Point::new(0.0, 0.0, 0.0); 
                let ray_vector = Vector3::new(trans_x, trans_y, -1.0).normalize(); 
                let ray = Ray::create_ray(ray_origin, ray_vector); 

                for sphere in self.spheres.iter() {
                    if sphere.intersect(ray.copy()) {
                        image.put_pixel(x, y, Rgba::from_channels(0, 0, 0, 0)); 
                    }
                    else{
                        image.put_pixel(x, y, Rgba::from_channels(255, 255, 255, 0)); 
                    }
                }
            }
        }
        image.save_with_format("images/test.png", ImageFormat::Png).ok(); 
    }
}