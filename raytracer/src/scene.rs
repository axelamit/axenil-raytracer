use crate::vector::Vector3;
use crate::point::Point;  
use crate::ray::Ray; 
use crate::sphere::Sphere; 
use crate::color::*; 
use crate::light::*; 
use crate::material::Material; 

use image; 
use image::{DynamicImage, GenericImage, Pixel, Rgba, ImageFormat};

pub struct Scene{
    pub width: u32,
    pub height: u32,
    pub spheres: Vec<Sphere>,
    pub directional_lights: Vec<DirectionalLight>, 
    pub spherical_lights: Vec<SphericalLight>,
}

impl Scene{
    pub fn new() -> Scene {
        Scene{
            width: 1000,
            height: 1000,
            spheres: Vec::new(),
            directional_lights: Vec::new(),
            spherical_lights: Vec::new(), 
        }
    }

    pub fn test_scene(&mut self) {
        let sphere1 = Sphere::new(Point::new(3.0, 0.0, -20.0), 2.0, Color::new_from_color(Colors::Red), 0.1, Material::Diffuse); 
        self.add_sphere(sphere1);

        let sphere2 = Sphere::new(Point::new(-4.0, 0.0, -10.0), 4.0, Color::new_from_color(Colors::Red), 0.1, Material::Reflective(0.0)); 
        self.add_sphere(sphere2);

        /*
        let sphere2 = Sphere::new(Point::new(2.0, 0.0, -10.0), 4.0, Color::new_from_color(Colors::Green), 1.); 
        self.add_sphere(sphere2);*/
        
        /*
        let sphere2 = Sphere::new(Point::new(1.0, -4.0, -10.0), 2.0, Color::new_from_color(Colors::Red), 0.1); 
        self.add_sphere(sphere2);*/

        /* 
        let dir_light1 = DirectionalLight::new(Vector3::new(400.0, 1.0, -1.0), Color::new_from_color(Colors::White), 100.0); 
        self.directional_lights.push(dir_light1); */

        let sph_light1 = SphericalLight::new(Point::new(-10.0, 0.0, 0.0), 2.0, Color::new_from_color(Colors::White), 6000.0); 
        self.spherical_lights.push(sph_light1); 
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere); 
    }

    pub fn get_closest_sphere(&self, ray: &Ray) -> f64{
        let mut distance = 999999999.0; 
        let mut closest_sphere = -1.0; 
        for i in 0..self.spheres.len() {
            let intersect = self.spheres[i].intersect(ray);
            //println!("Intersect: {}, Distance: {}", intersect.1, distance);  
            if intersect.0{
                if intersect.1 < distance{
                    distance = intersect.1; 
                    closest_sphere = i as f64; 
                }
            }
        }
        closest_sphere
    }

    pub fn normal_shading(&self, ray: &Ray, sphere: &Sphere, intersection: Point) -> Color{
        let mut color = Color::new_from_color(Colors::Black); 

        //Spherical lights
        for light in self.spherical_lights.iter(){
            let light_intersect = light.intersect(ray); 

            let direction_to_light = light.center.subtract(&intersection).to_vector3().normalize().neg(); 
            let shadow_ray = Ray::create_ray(intersection.add(&sphere.normal(&intersection).multiply(0.00001).to_point()), direction_to_light.copy(), 1); 
            let shadow_sphere = self.get_closest_sphere(&shadow_ray);
            let shadow_sphere_intersect = self.spheres[shadow_sphere as usize].intersect(&shadow_ray);  

            let light_blocked = if shadow_sphere == -1.0 || (light_intersect.1 > shadow_sphere_intersect.1) {false} else {true}; 

            let distance_to_light = light.center.subtract(&ray.origin).to_vector3().length(); 
            let light_intensity = if light_blocked {0.0} else {light.intensity / (4.0 * ::std::f64::consts::PI * distance_to_light)}; 
            let light_power = (sphere.normal(&intersection).dot(&direction_to_light)).max(0.0) * light_intensity; 
            let light_reflected = sphere.albedo / ::std::f64::consts::PI; 

            let light_color = light.color.normalize().multiply_scalar(light_power).multiply_scalar(light_reflected); 
            color = color.add_color(&sphere.color.normalize().multiply_color(&light_color)); 
        }
        color.to_normal_range().min_max()
    }

    //TODO - 1. Check if light source is closer than the intersection 
    //       2. Move the for loop for lights and check for any sphere intersection inside of it 
    //       3. Change light intensity based on the inverse square law and mabye add some check of the angle from the normal like 
    //       in the directional light function 
    //       4. Add the directional light
    pub fn get_color(&self, ray: &Ray) -> Color{
        if ray.recursion_limit == 0{
            return Color::new_from_rgba(53., 81., 98., 0.)
            //return Color::new_from_color(Colors::Black)
        }

        let closest_sphere = self.get_closest_sphere(ray);  
        if closest_sphere == -1.0{
            return Color::new_from_rgba(53., 81., 98., 0.)
        }

        let sphere_intersect = self.spheres[closest_sphere as usize].intersect(ray); 
        let sphere_intersection = ray.origin.add(&ray.direction.multiply(sphere_intersect.1).to_point()); 
        let normal = self.spheres[closest_sphere as usize].normal(&sphere_intersection); 

        match self.spheres[closest_sphere as usize].material{
            Material::Diffuse => self.normal_shading(ray, &self.spheres[closest_sphere as usize], sphere_intersection),
            Material::Reflective(reflectivity) => {
                let mut color = self.normal_shading(ray, &self.spheres[closest_sphere as usize], sphere_intersection.copy()); 
                let reflection_ray = ray.reflection(&normal, &sphere_intersection); 
                color = color.multiply_scalar(1.0 - reflectivity); 
                color = color.add_color(&self.get_color(&reflection_ray).multiply_scalar(reflectivity)); 
                color
            }
        }
    }

    pub fn render(&self) {
        let mut image = DynamicImage::new_rgb8(self.width, self.height); 
        for y in 0..self.height{
            for x in 0..self.width{
                let trans_x = (x as f64/self.width as f64)*2.0 - 1.0; 
                let trans_y = (y as f64/self.height as f64)*2.0 - 1.0;

                let ray_origin = Point::new(0.0, 0.0, 0.0); 
                let ray_vector = Vector3::new(trans_x, trans_y, -1.0).normalize(); 
                let ray = Ray::create_ray(ray_origin, ray_vector, 3); 

                let color = self.get_color(&ray); 
                if color.a != -1.0{
                    image.put_pixel(x, y, Rgba::from_channels(color.r as u8, color.g as u8, color.b as u8, color.a as u8)); 
                }
            }
        }
        image.save_with_format("images/test.png", ImageFormat::Png).ok(); 
    }
}