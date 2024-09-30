extern crate nalgebra as na;
extern crate image;

use na::{Point3, Vector3};

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

pub struct Cube {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}

impl Cube {
    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let inv_dir = Vector3::new(1.0 / ray.direction.x, 1.0 / ray.direction.y, 1.0 / ray.direction.z);
        let mut t_min = (self.min - ray.origin).component_mul(&inv_dir);
        let mut t_max = (self.max - ray.origin).component_mul(&inv_dir);

        for i in 0..3 {
            if inv_dir[i] < 0.0 {
                std::mem::swap(&mut t_min[i], &mut t_max[i]);
            }
        }

        let t_enter = t_min.max();
        let t_exit = t_max.min();

        // Si hay intersección válida
        if t_enter < t_exit && t_exit > 0.0 {
            Some(t_enter)
        } else {
            None
        }
    }
}
