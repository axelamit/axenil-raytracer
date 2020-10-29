use crate::color::Color;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector3;

pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f64,
}

impl DirectionalLight {
    pub fn new(direction: Vector3, color: Color, intensity: f64) -> DirectionalLight {
        DirectionalLight {
            direction: direction,
            color: color,
            intensity: intensity,
        }
    }
}

pub struct SphericalLight {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
    pub intensity: f64,
}

impl SphericalLight {
    pub fn new(center: Point, radius: f64, color: Color, intensity: f64) -> SphericalLight {
        SphericalLight {
            center: center,
            radius: radius,
            color: color,
            intensity: intensity,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> (bool, f64) {
        let origin_center = self.center.subtract(&ray.origin).to_vector3();

        let projection_length2 = origin_center.dot(&ray.direction);
        let perpendicular2 =
            origin_center.dot(&origin_center) - (projection_length2 * projection_length2);
        let radius2 = self.radius * self.radius;

        if perpendicular2 > radius2 {
            return (false, -1.0);
        }

        let add = (radius2 - perpendicular2).sqrt();
        let intersection0 = projection_length2 - add;
        let intersection1 = projection_length2 + add;

        if intersection0 < 0.0 && intersection1 < 0.0 {
            return (false, -1.0);
        }

        let distance = if intersection0 < intersection1 {
            intersection0
        } else {
            intersection1
        };
        return (true, distance);
    }
}
