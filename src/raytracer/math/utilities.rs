
use super::PI;

use super::random::{ Rand_generator
                   , random_f64 };

pub const fn degrees_to_radians( degrees: f64 ) -> f64 {
    degrees * PI / 180.0
}

pub const fn random_double() -> f64 {
    static mut GENERATOR: Rand_generator = Rand_generator::new( 7892365412305211419 );
    
    unsafe {
        random_f64( &mut GENERATOR )
    }
}

pub const fn random_double_interval( min: f64, max: f64 ) -> f64 {
    min + ( max - min ) * random_double()
}