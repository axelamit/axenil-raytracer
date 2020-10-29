use crate::color::*;
use crate::element::Element;
use crate::light::*;
use crate::material::Material;
use crate::plane::Plane;
use crate::point::Point;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector::Vector3;

use image;
use image::{DynamicImage, GenericImage, ImageFormat, Pixel, Rgba};

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub elements: Vec<Element>,
    pub directional_lights: Vec<DirectionalLight>,
    pub spherical_lights: Vec<SphericalLight>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            width: 1000,
            height: 1000,
            elements: Vec::new(),
            directional_lights: Vec::new(),
            spherical_lights: Vec::new(),
        }
    }

    pub fn test_scene(&mut self) {
        let sphere1 = Sphere::new(
            Point::new(2.0, 0.0, 4.0),
            1.0,
            Color::new_from_color(Colors::Red),
            0.1,
            Material::Diffuse,
        );
        self.elements.push(Element::Sphere(sphere1));

        let sphere2 = Sphere::new(
            Point::new(-1.0, 0.0, 4.0),
            1.0,
            Color::new_from_color(Colors::Green),
            0.1,
            Material::Reflective(0.9),
        );
        self.elements.push(Element::Sphere(sphere2));

        let sphere3 = Sphere::new(
            Point::new(-1.0, -0.5, 1.5),
            0.5,
            Color::new_from_color(Colors::Blue),
            0.1,
            Material::Reflective(0.9),
        );
        self.elements.push(Element::Sphere(sphere3));

        let plane1 = Plane::new(
            Point::new(0.0, -1.0, 0.0),
            Vector3::new(0.0, -1.0, 0.0),
            Color::new_from_rgba(55., 57., 67., 0.),
            0.1,
            Material::Diffuse,
        );
        self.elements.push(Element::Plane(plane1));

        /*
        let dir_light1 = DirectionalLight::new(Vector3::new(100.0, -3.0, 3.0).normalize(), Color::new_from_color(Colors::White), 200.0);
        self.directional_lights.push(dir_light1);*/

        let sph_light1 = SphericalLight::new(
            Point::new(2.0, 2.0, 0.0),
            2.0,
            Color::new_from_color(Colors::White),
            2000.0,
        );
        self.spherical_lights.push(sph_light1);
    }

    pub fn get_closest_element(&self, ray: &Ray) -> f64 {
        let mut distance = 999999999.0;
        let mut closest_element = -1.0;
        for i in 0..self.elements.len() {
            let intersect = self.elements[i].intersect(ray);
            if intersect.0 {
                if intersect.1 < distance {
                    distance = intersect.1;
                    closest_element = i as f64;
                }
            }
        }
        closest_element
    }

    pub fn normal_shading(&self, ray: &Ray, element: &Element, intersection: Point) -> Color {
        let mut color = Color::new_from_color(Colors::Black);

        for light in self.spherical_lights.iter() {
            let light_intersect = light.intersect(ray);

            let direction_to_light = light
                .center
                .subtract(&intersection)
                .to_vector3()
                .normalize();
            let shadow_ray = Ray::create_ray(
                intersection.add(&element.normal(&intersection).multiply(0.00001).to_point()),
                direction_to_light.copy(),
                1,
            );
            let shadow_element = self.get_closest_element(&shadow_ray);
            let shadow_element_intersect =
                self.elements[shadow_element as usize].intersect(&shadow_ray);

            let light_blocked =
                if shadow_element == -1.0 || (light_intersect.1 > shadow_element_intersect.1) {
                    false
                } else {
                    true
                };

            let distance_to_light = light.center.subtract(&ray.origin).to_vector3().length();
            let light_intensity = if light_blocked {
                0.0
            } else {
                light.intensity / (4.0 * ::std::f64::consts::PI * distance_to_light)
            };
            let light_power =
                (element.normal(&intersection).dot(&direction_to_light)).max(0.0) * light_intensity;
            let light_reflected = element.get_albedo() / ::std::f64::consts::PI;

            let light_color = light
                .color
                .normalize()
                .multiply_scalar(light_power)
                .multiply_scalar(light_reflected);
            color = color.add_color(&element.get_color().normalize().multiply_color(&light_color));
        }
        for light in self.directional_lights.iter() {
            let direction_to_light = light.direction.neg();

            let shadow_ray = Ray::create_ray(
                intersection.add(&direction_to_light.multiply(0.00001).to_point()),
                direction_to_light.copy(),
                1,
            );
            let shadow_element = self.get_closest_element(&shadow_ray);

            let light_blocked = if shadow_element == -1.0 { false } else { true };

            let light_intensity = if light_blocked { 0.0 } else { light.intensity };
            let light_power =
                (element.normal(&intersection).dot(&direction_to_light)).max(0.0) * light_intensity;
            let light_reflected = element.get_albedo() / ::std::f64::consts::PI;

            let light_color = light
                .color
                .normalize()
                .multiply_scalar(light_power)
                .multiply_scalar(light_reflected);
            color = color.add_color(&element.get_color().normalize().multiply_color(&light_color));
        }
        color.to_normal_range().min_max()
    }

    pub fn get_color(&self, ray: &Ray) -> Color {
        if ray.recursion_limit == 0 {
            return Color::new_from_rgba(53., 81., 98., 0.);
        }

        let closest_element = self.get_closest_element(ray);
        if closest_element == -1.0 {
            return Color::new_from_rgba(53., 81., 98., 0.);
        }

        let element_intersect = self.elements[closest_element as usize].intersect(ray);
        let element_intersection = ray
            .origin
            .add(&ray.direction.multiply(element_intersect.1).to_point());
        let normal = self.elements[closest_element as usize].normal(&element_intersection);

        match self.elements[closest_element as usize].get_material() {
            Material::Diffuse => self.normal_shading(
                ray,
                &self.elements[closest_element as usize],
                element_intersection,
            ),
            Material::Reflective(reflectivity) => {
                let mut color = self.normal_shading(
                    ray,
                    &self.elements[closest_element as usize],
                    element_intersection.copy(),
                );
                let reflection_ray = ray.reflection(&normal, &element_intersection);
                color = color.multiply_scalar(1.0 - reflectivity);
                color = color.add_color(
                    &self
                        .get_color(&reflection_ray)
                        .multiply_scalar(*reflectivity),
                );
                color
            }
        }
    }

    pub fn render(&self) {
        let mut image = DynamicImage::new_rgb8(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let trans_x = (x as f64 / self.width as f64) * 2.0 - 1.0;
                let trans_y = 1.0 - (y as f64 / self.height as f64) * 2.0;

                let ray_origin = Point::new(0.0, 0.0, 0.0);
                let ray_vector = Vector3::new(trans_x, trans_y, 1.0).normalize();
                let ray = Ray::create_ray(ray_origin, ray_vector, 4);

                let color = self.get_color(&ray);
                if color.a != -1.0 {
                    image.put_pixel(
                        x,
                        y,
                        Rgba::from_channels(
                            color.r as u8,
                            color.g as u8,
                            color.b as u8,
                            color.a as u8,
                        ),
                    );
                }
            }
        }
        image
            .save_with_format("images/test.png", ImageFormat::Png)
            .ok();
    }
}