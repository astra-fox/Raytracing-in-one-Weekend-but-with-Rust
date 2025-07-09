
use super::math::vec3f::*;

pub struct Ray {
    orig: Point3,
    dir: Vec3f,
}
#[allow( non_snake_case )]
pub fn Ray( orig: Point3, dir: Vec3f ) -> Ray {
    Ray { orig, dir }
}

impl Ray {

    pub fn    origin( &self ) -> Point3 { self.orig }
    pub fn direction( &self ) ->  Vec3f { self.dir }

    pub fn at( &self, t: f64 ) -> Point3 {
        self.orig + t * self.dir
    }
}
