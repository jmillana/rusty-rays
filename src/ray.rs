use crate::vec3;

pub struct Ray {
    pub origin: vec3::Point3,
    pub direction: vec3::Vec3,
}

impl Ray {
    pub fn new() -> Self {
        return Ray {
            origin: vec3::Point3::new(),
            direction: vec3::Vec3::new(),
        };
    }

    pub fn build(origin: &vec3::Point3, direction: &vec3::Vec3) -> Self {
        return Ray {
            origin: vec3::Point3::build(origin.x(), origin.y(), origin.z()),
            direction: vec3::Vec3::build(direction.x(), direction.y(), direction.z()),
        };
    }

    pub fn origin(self) -> vec3::Point3 {
        return self.origin;
    }

    pub fn direction(self) -> vec3::Vec3 {
        return self.direction;
    }

    pub fn at(self, t: &f64) -> vec3::Point3 {
        return &self.origin + &(&self.direction * t);
    }
}
