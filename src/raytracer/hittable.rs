
use super::math::interval::Interval;
use super::math::vec3f::*;
use super::ray::*;

use std::sync::Arc;

use super::Rc;
use super::material;

pub struct Hit_record {
    pub p: Point3,
    pub normal: Vec3f,
    pub t: f64,
    pub front_face: bool,
    pub mat: Arc<dyn material::Material + Send + Sync>,
}

impl Hit_record {

    pub fn new() -> Hit_record {
        Self {
            p: Point3( 0.0, 0.0, 0.0 ),
            normal: Vec3f( 0.0, 0.0, 0.0 ),
            t: 0.0,
            front_face: false,
            mat: Arc::new( material::Empty_mat {} ),
        }
    }

    pub fn set_face_normal( &mut self, r: &Ray, outward_normal: &Vec3f ) {
        self.front_face = dot( r.direction(), *outward_normal ) < 0.0;
        self.normal     = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

pub trait Hittable: 'static + Send + Sync {

    fn hittable( &self );

    //fn hit( &self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut Hit_record ) -> bool;
    //fn hit( &self, r: &Ray, ray_tmin: f64, ray_tmax: f64, ) -> (bool, Hit_record);
    fn hit( &self, r: &Ray, ray_t: &Interval, ) -> ( bool, Hit_record );
}
