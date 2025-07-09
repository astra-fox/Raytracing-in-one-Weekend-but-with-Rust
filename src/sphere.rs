
use crate::raytracer::material;
use crate::raytracer::Rc;

use std::sync::Arc;

use crate::raytracer::math::interval::*;
use crate::raytracer::math::vec3f::*;
use crate::raytracer::ray::*;
use crate::raytracer::hittable::*;
use crate::raytracer::material::*;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material + Send + Sync>,
}

impl Sphere {

    pub fn new( center: Point3, radius: f64, mat: Arc<dyn Material + Send + Sync> ) -> Sphere {
        Sphere { center, radius, mat }
    }
}

impl Hittable for Sphere {

    fn hit( &self, r: &Ray, ray_t: &Interval ) -> ( bool, Hit_record ) {
        
        let oc = self.center - r.origin();
        let a  = r.direction().norm_sq();
        let h  = dot( r.direction(), oc );
        let c  = oc.norm_sq() - self.radius * self.radius;

        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return ( false, Hit_record::new() )
        }

        let sqrtd = f64::sqrt( discriminant );
        
        let root = {
            let mut root = ( h - sqrtd ) / a;
            if !ray_t.surrounds( root ) {
                root = ( h + sqrtd ) / a;
                if !ray_t.surrounds( root ) {
                    return ( false, Hit_record::new() )
                } else {
                    root
                }
            } else {
                root
            }
        };

        let mut rec = Hit_record::new();

        rec.t = root;
        rec.p = r.at( rec.t );

        let outward_normal = ( rec.p - self.center ) / self.radius;
        rec.set_face_normal( r, &outward_normal );
        
        rec.mat = self.mat.clone();

        ( true, rec )
    }

    fn hittable( &self ) { (); }
}
