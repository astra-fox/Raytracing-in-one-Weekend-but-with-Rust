
use super::color::Color;
use super::hittable::Hit_record;
use super::math::utilities::random_double;
use super::math::vec3f::{ Vec3f, dot };
use super::ray::Ray;

pub trait Material {
    fn scatter( &self, r_in: &Ray, rec: &Hit_record ) -> ( bool, Color, Ray );
}

pub struct Empty_mat {}
impl Material for Empty_mat {
    fn scatter( &self, r_in: &Ray, rec: &Hit_record ) -> (bool, Color, Ray) {
        ( true, Color( 0.0, 0.0, 0.0 ), Ray( Vec3f( 0.0, 0.0, 0.0 )
                                           , Vec3f( 0.0, 0.0, 0.0 )))
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new( albedo: &Color ) -> Self {
        Self { albedo: *albedo }
    }
}

impl Material for Lambertian {

    fn scatter( &self, r_in: &Ray, rec: &Hit_record ) -> ( bool, Color, Ray ) {
        
        let mut scatter_direction = rec.normal - Vec3f::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered   = Ray( rec.p, scatter_direction );
        let attenuation = self.albedo;

        ( true, attenuation, scattered )
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new( albedo: &Color, fuzz: f64 ) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo: *albedo, fuzz }
    }
}

impl Material for Metal {

    fn scatter( &self, r_in: &Ray, rec: &Hit_record ) -> (bool, Color, Ray) {

        let reflected   = Vec3f::reflect( r_in.direction(), rec.normal );
        let reflected   = reflected.normalize() + self.fuzz * Vec3f::random_unit_vector();
        let scattered   = Ray( rec.p, reflected );
        let attenuation = self.albedo;
        let bool_ret    = dot( scattered.direction(), rec.normal ) > 0.0;

        ( bool_ret, attenuation, scattered )
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new( refraction_index: f64 ) -> Self {
        Self { refraction_index }
    }

    fn reflectance( cosine: f64, refraction_index: f64 ) -> f64 {
        let r0 = ( 1.0 - refraction_index ) / ( 1.0 + refraction_index );
        let r0 = r0 * r0;

        r0 + ( 1.0 - r0 ) * f64::powi( 1.0 - cosine, 5 )
    }
}

impl Material for Dielectric {
    
    fn scatter( &self, r_in: &Ray, rec: &Hit_record ) -> (bool, Color, Ray) {

        let attenuation = Color( 1.0, 1.0, 1.0 );
        let ri =
            if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = r_in.direction().normalize();
        let cos_theta      = f64::min( dot( -unit_direction, rec.normal ), 1.0 );
        let sin_theta      = f64::sqrt( 1.0 - cos_theta * cos_theta );
        let cannot_refract = ri * sin_theta > 1.0;

        let direction =
            if cannot_refract || Self::reflectance( cos_theta, ri ) > random_double() {
                Vec3f::reflect( unit_direction, rec.normal )
            } else {
                Vec3f::refract( unit_direction, rec.normal, ri )
            };

        let scattered = Ray( rec.p, direction );

        ( true, attenuation, scattered )
    }
}
