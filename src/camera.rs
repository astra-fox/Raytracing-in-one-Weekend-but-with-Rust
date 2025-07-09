
use std::io::Write;
use std::io::stdout;

use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

use math::utilities::degrees_to_radians;
use math::utilities::random_double;

use crate::raytracer::*;
use crate::raytracer::ray::*;
use crate::raytracer::color::*;
use crate::raytracer::hittable::*;

use crate::raytracer::math::vec3f::*;
use crate::raytracer::math::interval::*;

#[derive( Clone, Copy )]
pub struct Camera {
    
    pub aspect_ratio: f64,
    pub vfov: f64,

    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,

    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3f,

    pub defocus_angle: f64,
    pub focus_dist: f64,

    pixel_samples_scale: f64,
    image_height: i32,

    center: Point3,
    pixel00_loc: Point3,

    pixel_delta_u: Vec3f,
    pixel_delta_v: Vec3f,

    u: Vec3f,
    v: Vec3f,
    w: Vec3f,

    defocus_disk_u: Vec3f,
    defocus_disk_v: Vec3f,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {

            aspect_ratio: 1.0,
            vfov: 90.0,

            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,

            lookfrom: Point3( 0.0, 0.0, 0.0 ),
            lookat: Point3( 0.0, 0.0, -1.0 ),
            vup: Vec3f( 0.0, 1.0, 0.0 ),

            defocus_angle: 0.0,
            focus_dist: 10.0,

            pixel_samples_scale: 0.5,
            image_height: 100,

            center: Point3( 0.0, 0.0, 0.0 ),
            pixel00_loc: Point3( 0.0, 0.0, 0.0 ),

            pixel_delta_u: Vec3f( 0.0, 0.0, 0.0 ),
            pixel_delta_v: Vec3f( 0.0, 0.0, 0.0 ),

            u: Vec3f( 0.0, 0.0, 0.0 ),
            v: Vec3f( 0.0, 0.0, 0.0 ),
            w: Vec3f( 0.0, 0.0, 0.0 ),

            defocus_disk_u: Vec3f( 0.0, 0.0, 0.0 ),
            defocus_disk_v: Vec3f( 0.0, 0.0, 0.0 ),
        }
    }
}

impl Camera {

    pub fn render( &mut self, world: &impl Hittable ) {
        
        //self.initializer();
        
        println!( "P3\n{} {}\n255", self.image_width, self.image_height );

        for j in 0..self.image_height {
    
            eprint!( "Scanlines remaining: {}   \r", self.image_height - j );
            stdout().flush().unwrap();
    
            for i in 0..self.image_width {

                let mut pixel_color = Color( 0.0, 0.0, 0.0 );
                
                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray( i, j );
                    pixel_color += Camera::ray_color( &r, self.max_depth, world );
                }

                let display_color = self.pixel_samples_scale * pixel_color;
                write_color( &mut stdout(), &display_color );
            }
        }

        eprint!( "\r                                      \r" ); stdout().flush().unwrap();
        eprint!( "Done!\n" );
    }

    pub fn render_with_flatten_for( &mut self, world: &impl Hittable ) {

        //self.initializer();
        
        println!( "P3\n{} {}\n255", self.image_width, self.image_height );

        let total = self.image_height * self.image_width;

        let ( mut i, mut j ): ( i32, i32 ) = ( 0, -1 );
        for t in 0..total {

            eprint!( "Pixels remaining: {}             \r", total - t );
            stdout().flush().unwrap();

            i = t % self.image_width;
            if i == 0 { j += 1; }

            let pixel_color = {

                let mut pixel_color= Color( 0.0, 0.0, 0.0 );

                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray( i, j );
                    pixel_color += Camera::ray_color( &r, self.max_depth, world );
                }

                pixel_color
            };

            let display_color = self.pixel_samples_scale * pixel_color;
            write_color( &mut stdout(), &display_color );
        }

        eprint!( "\r                                      \r" ); stdout().flush().unwrap();
        eprint!( "Done!\n" );
    }

    pub fn initializer( &mut self ) {

        self.image_height = ( self.image_width as f64 / self.aspect_ratio ) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        let theta           = degrees_to_radians( self.vfov );
        let h               = f64::tan( theta / 2.0 );
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width  = viewport_height * ( self.image_width as f64 / self.image_height as f64 );

        let w = ( self.lookfrom - self.lookat ).normalize();
        let u = cross( self.vup, w ).normalize();
        let v = cross( w, u );

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center - self.focus_dist * w
                                - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left
                         + 0.5 * ( self.pixel_delta_u + self.pixel_delta_v );

        let defocus_radius = self.focus_dist
                           * f64::tan( degrees_to_radians( self.defocus_angle / 2.0 ));

        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius;
    }

    fn get_ray( &self, i: i32, j: i32 ) -> Ray {
        
        let offset = Camera::sample_square();

        let pixel_sample = self.pixel00_loc
                         + (( i as f64 + offset.x() ) * self.pixel_delta_u )
                         + (( j as f64 + offset.y() ) * self.pixel_delta_v );
        
        let ray_origin =
            if self.defocus_angle <= 0.0 {
                self.center
            } else {
                self.defocus_disk_sample()
            };
        let ray_direction = pixel_sample - ray_origin;

        Ray( ray_origin, ray_direction )
    }

    const fn sample_square() -> Vec3f {
        Vec3f( random_double() - 0.5, random_double() - 0.5, 0.0 )
    }

    fn defocus_disk_sample( &self ) -> Point3 {
        let p = Vec3f::random_in_unit_disk();
        self.center + p.0 * self.defocus_disk_u + p.1 * self.defocus_disk_v
    }

    fn ray_color( r: &Ray, depth: i32, world: &impl Hittable) -> Color {

        if depth <= 0 {
            return Color( 0.0, 0.0, 0.0 )
        }

        let ( bool_ret, rec ) = world.hit( r, &Interval::new( 0.001, math::INFINITY ));

        if bool_ret {

            let ( bool_scat, attenuation, scattered ) = rec.mat.scatter( r, &rec );

            if bool_scat {
                return attenuation * Camera::ray_color( &scattered, depth - 1, world )
            }

            return Color( 0.0, 0.0, 0.0 )
        }

        let unit_direction = r.direction().normalize();
        let a              = 0.5 * ( unit_direction.y() + 1.0 );

        ( 1.0 - a ) * Color( 1.0, 1.0, 1.0 ) + a * Color( 0.5, 0.7, 1.0 )
    }
}

pub struct Multithread_camera {
    camera: Arc<Camera>,
}

impl Multithread_camera {

    pub fn new( camera: Camera ) -> Self {
        Multithread_camera { camera: Arc::new( camera ) }
    }

    pub fn render_multithread( &self, world: &'static impl Hittable ) {

        let local_camera = self.camera.clone();

        //local_camera.lock().unwrap().initializer();
        let image_width         = local_camera.image_width;
        let image_height        = local_camera.image_height;
        let samples_per_pixel   = local_camera.samples_per_pixel;
        let max_depth           = local_camera.max_depth;
        let pixel_samples_scale = local_camera.pixel_samples_scale;

        let number_of_threads = 4;
        
        println!( "P3\n{} {}\n255", image_width, image_height );

        let total = image_height * image_width;

        let ( mut i, mut j ): ( i32, i32 ) = ( 0, -1 );
        'pixels: for t in ( 0..total ).step_by( number_of_threads as usize ) {

            eprint!( "Pixels remaining: {}             \r", total - t );
            stdout().flush().unwrap();

            let handles = {
                let mut handles = vec![];

                'threads: for l in 0..number_of_threads {

                    let ii = t + l;
                    if ii >= total { break 'threads; } /* Was continue before. lmao */
        
                    i = ii % image_width;
                    if i == 0 { j += 1; }

                    let local_camera = self.camera.clone();

                    let h = thread::spawn(
                        move || {
                            let mut pixel_color = Color( 0.0, 0.0, 0.0 );
                            for sample in 0..samples_per_pixel {
                                let r = local_camera.get_ray( i, j );
                                pixel_color += Camera::ray_color( &r, max_depth, world );
                            }

                            pixel_color
                        });

                    handles.push( h );
                }

                handles
            };

            for handle in handles {
                let pixel_color   = handle.join().unwrap();
                let display_color = pixel_samples_scale * pixel_color;
                write_color( &mut stdout(), &display_color );
            }
        }

        eprint!( "\r                                      \r" ); stdout().flush().unwrap();
        eprint!( "Done!\n" );
    }
}