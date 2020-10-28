use crate::point::Point;

pub struct Vector3 {
    pub x: f64, 
    pub y: f64, 
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3{
        Vector3{
            x: x,
            y: y,
            z: z, 
        }
    }

    pub fn to_point(&self) -> Point{
        Point::new(self.x, self.y, self.z) 
    }

    pub fn add(&self, vector: &Vector3) -> Vector3{
        Vector3{
            x: self.x+vector.x,
            y: self.y+vector.y,
            z: self.z+vector.z, 
        }
    }

    pub fn subtract(&self, vector: &Vector3) -> Vector3{
        Vector3{
            x: self.x-vector.x,
            y: self.y-vector.y,
            z: self.z-vector.z, 
        }
    }

    pub fn multiply(&self, scalar: f64) -> Vector3 {
        Vector3{
            x: self.x*scalar,
            y: self.y*scalar,
            z: self.z*scalar, 
        }
    }

    pub fn normalize(&self) -> Vector3{
        let length = self.length(); 
        Vector3 {
            x: self.x/length,
            y: self.y/length,
            z: self.z/length,
        }
    }

    pub fn dot(&self, vector: &Vector3) -> f64 {
        self.x*vector.x+self.y*vector.y+self.z*vector.z
    }

    pub fn neg(&self) -> Vector3{
        self.multiply(-1.0)
    }

    pub fn length(&self) -> f64{
        self.dot(&self).sqrt()
    }

    pub fn copy(&self) -> Vector3{
        Vector3{
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}