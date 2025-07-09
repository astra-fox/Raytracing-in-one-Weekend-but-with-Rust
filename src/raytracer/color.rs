
use super::math::interval::*;
use super::math::vec3f::*;

pub type Color = Vec3f;
#[allow( non_snake_case )]
pub fn Color( x: f64, y: f64, z: f64 ) -> Color {
    Vec3f( x, y, z )
}

pub fn linear_to_gamma( linear_component: f64 ) -> f64 {

    if linear_component > 0.0 {
        return f64::sqrt( linear_component )
    }

    0.0
}

pub fn write_color( output: &mut impl std::io::Write
                  , pixel_color: &Color ) {

    let r = linear_to_gamma( pixel_color.x() );
    let g = linear_to_gamma( pixel_color.y() );
    let b = linear_to_gamma( pixel_color.z() );

    let rbyte = ( 256.0 * INTENSITY.clamp(r) ) as i32;
    let gbyte = ( 256.0 * INTENSITY.clamp(g) ) as i32;
    let bbyte = ( 256.0 * INTENSITY.clamp(b) ) as i32;

    writeln!( output, "{} {} {}", rbyte, gbyte, bbyte );
}

const INTENSITY: Interval = Interval { min: 0.0, max: 0.999 };