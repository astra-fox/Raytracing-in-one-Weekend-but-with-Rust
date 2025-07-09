
use std::rc::Rc;
use std::sync::Arc;

use super::math::vec3f::Point3;
use super::math::interval::Interval;
use super::ray::Ray;
use super::hittable::*;

pub struct Hittable_list {
    pub objects: Vec<Arc<dyn Hittable + 'static>>,
}

impl Hittable_list {

    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add( &mut self, object: Arc<dyn Hittable + 'static> ) {
        self.objects.push( object );
    }
}

impl Hittable for Hittable_list {

    fn hit( &self, r: &Ray, ray_t: &Interval ) -> ( bool, Hit_record ) {
        
        let mut ret_rec        = Hit_record::new();
        let mut hit_anything   = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {

            let temp_interval = Interval { min: ray_t.min, max: closest_so_far };
            
            let ( ret_bool, temp_rec ) = object.hit( r, &temp_interval );

            if ret_bool {
                hit_anything   = true;
                ret_rec        = temp_rec;
                closest_so_far = ret_rec.t;
            }
        }

        ( hit_anything, ret_rec )
    }

    fn hittable( &self ) {
        
    }
}
