
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;

use std::fmt;

use super::utilities::{ random_double
                      , random_double_interval };

#[derive( Clone, Copy )]
pub struct Vec3f( pub f64, pub f64, pub f64 );

pub type Point3 = Vec3f;
#[allow( non_snake_case )]
pub fn Point3( x: f64, y: f64, z: f64 ) -> Point3 {
    Vec3f( x, y, z )
}

impl Vec3f {

    pub fn x( &self ) -> f64 { self.0 }
    pub fn y( &self ) -> f64 { self.1 }
    pub fn z( &self ) -> f64 { self.2 }

    pub const fn norm_sq( &self ) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn near_zero( &self ) -> bool {

        let s = 1e-8;

        f64::abs( self.0 ) < s &&
        f64::abs( self.1 ) < s &&
        f64::abs( self.2 ) < s
    }

    pub fn norm( &self ) -> f64 {
        f64::sqrt( self.norm_sq() )
    }

    pub fn normalize( &self ) -> Vec3f {
        self.div( self.norm() )
    }

    const fn dot( &self, v: &Vec3f ) -> f64 {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }

    const fn cross( &self, v: &Vec3f ) -> Vec3f {
        Vec3f( self.1 * v.2 - self.2 * v.1
             , self.2 * v.0 - self.0 * v.2
             , self.0 * v.1 - self.1 * v.0 )
    }

    pub const fn random() -> Vec3f {
        Vec3f( random_double(), random_double(), random_double() )
    }

    pub const fn random_vec_interval( min: f64, max: f64 ) -> Vec3f {
        Vec3f( random_double_interval( min, max )
             , random_double_interval( min, max )
             , random_double_interval( min, max ))
    }

    pub fn random_in_unit_disk() -> Vec3f {

        let mut p = Vec3f( random_double_interval( -1.0, 1.0 )
                         , random_double_interval( -1.0, 1.0 ), 0.0 );
        let mut normq = p.norm_sq();

        while true {
            if normq < 1.0 {
                break;
            } else {
                p = Vec3f( random_double_interval( -1.0, 1.0 )
                         , random_double_interval( -1.0, 1.0 ), 0.0 );
                normq = p.norm_sq();
            }
        }

        p
    }

    pub fn random_unit_vector() -> Vec3f {

        let mut p     = Vec3f::random_vec_interval( -1.0, 1.0 );
        let mut normq = p.norm_sq();

        while true {
            if 1e-160 <= normq && normq <= 1.0 {
                p = p / f64::sqrt( normq );
                break;
            } else {
                p     = Vec3f::random_vec_interval( -1.0, 1.0 );
                normq = p.norm_sq();
            }
        }

        p
    }

    pub fn random_on_hemisphere( normal: &Vec3f ) -> Vec3f {

        let on_unit_sphere = Vec3f::random_unit_vector();

        if dot( on_unit_sphere, *normal ) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn reflect( v: Vec3f, n: Vec3f ) -> Vec3f {
        v - 2.0 * dot(v, n) * n
    }

    pub fn refract( uv: Vec3f, n: Vec3f, etai_over_etat: f64 ) -> Vec3f {
        let cos_theta      = f64::min( dot( -uv, n ), 1.0 );
        let r_out_perp     = etai_over_etat * ( uv + cos_theta * n );
        let r_out_parallel = -f64::sqrt( f64::abs( 1.0 - r_out_perp.norm_sq() )) * n;

        r_out_perp + r_out_parallel
    }

}

impl Add for Vec3f {
    type Output = Vec3f;
    fn add( self, rhs: Vec3f ) -> Vec3f {
        Vec3f( self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2 )
    }
}

impl AddAssign for Vec3f {
    fn add_assign( &mut self, rhs: Self ) {
        *self = Vec3f( self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2 )
    }
}

impl Mul<f64> for Vec3f {
    type Output = Vec3f;
    fn mul( self, rhs: f64 ) -> Vec3f {
        Vec3f( self.0 * rhs, self.1 * rhs, self.2 * rhs )
    }
}

impl MulAssign<f64> for Vec3f {
    fn mul_assign( &mut self, rhs: f64 ) {
        *self = Vec3f( rhs * self.0, rhs * self.1, rhs * self.2 )
    }
}

impl Mul<Vec3f> for f64 {
    type Output = Vec3f;
    fn mul( self, rhs: Vec3f ) -> Vec3f {
        Vec3f( self * rhs.0, self * rhs.1, self * rhs.2 )
    }
}

impl Mul<Vec3f> for Vec3f {
    type Output = Vec3f;
    fn mul(self, rhs: Vec3f) -> Self::Output {
        Vec3f( self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2 )
    }
}

impl Neg for Vec3f {
    type Output = Vec3f;
    fn neg( self ) -> Self::Output {
        -1.0 * self
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;
    fn sub( self, rhs: Vec3f ) -> Vec3f {
        self + (-1.0 * rhs)
    }
}

impl SubAssign for Vec3f {
    fn sub_assign( &mut self, rhs: Self ) {
        *self += -1.0 * rhs
    }
}

impl Div<f64> for Vec3f {
    type Output = Vec3f;
    fn div( self, rhs: f64 ) -> Vec3f {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Vec3f {
    fn div_assign( &mut self, rhs: f64 ) {
        *self *= 1.0 / rhs
    }
}

impl fmt::Display for Vec3f {
    fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
        write!( f, "({}, {}, {})", self.0, self.1, self.2 )
    }
}

pub const fn dot( v: Vec3f, u: Vec3f ) -> f64 {
    v.dot( &u )
}

pub const fn cross( v: Vec3f, u: Vec3f ) -> Vec3f {
    v.cross( &u )
}
