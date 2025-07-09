
#![allow( warnings )]

mod raytracer;
mod sphere;
mod camera;

use raytracer::hittable_list::Hittable_list;

use raytracer::material::{ Empty_mat
                         , Dielectric
                         , Lambertian
                         , Metal };

use raytracer::color::Color;

use raytracer::Rc;
use std::sync::Arc;

use raytracer::math::vec3f::{ Vec3f
                            , Point3 };
use raytracer::math::utilities::{ random_double
                                , random_double_interval };
use raytracer::math::PI;

use sphere::Sphere;
use camera::Camera;
use camera::Multithread_camera;

fn main() {

    use std::time::Instant;

    let now = Instant::now();
    one_weekend_final_render();
    let elapse = now.elapsed();

    eprintln!( "Elapsed: {:.2?}                     ", elapse );
}

fn one_weekend_final_render() {

    //let mut world = Hittable_list::new();
    let mut world = Box::leak( Box::new( Hittable_list::new() ));

    let ground_material = Arc::new( Lambertian::new( &Color( 0.5, 0.5, 0.5 )));
    world.add( Arc::new( Sphere::new( Point3( 0.0, -1000.0, 0.0 )
                                    , 1000.0, ground_material )));

    for a in -11..11 {
        for b in -11..11 {

            let choose_mat = random_double();
            let center     = Point3( a as f64 + 0.9 * random_double(), 0.2
                                   , b as f64 + 0.9 * random_double() );

            if ( center - Point3( 4.0, 0.2, 0.0 )).norm() > 0.9 {
                
                if choose_mat < 0.8 {
                    
                    let albedo          = Color::random() * Color::random();
                    let sphere_material = Arc::new( Lambertian::new( &albedo ));
                    world.add( Arc::new( Sphere::new( center, 0.2, sphere_material )));
                }
                else if choose_mat < 0.95 {

                    let albedo          = Color::random_vec_interval( 0.5, 1.0 );
                    let fuzz            = random_double_interval( 0.0, 0.5 );
                    let sphere_material = Arc::new( Metal::new( &albedo, fuzz ));
                    world.add( Arc::new( Sphere::new( center, 0.2, sphere_material )));
                }
                else {

                    let sphere_material = Arc::new( Dielectric::new( 1.5 ));
                    world.add( Arc::new( Sphere::new( center, 0.2, sphere_material )));
                }
            }
        }
    }

    let material_1 = Arc::new( Dielectric::new( 1.5 ));
    world.add( Arc::new( Sphere::new( Point3( 0.0, 1.0, 0.0), 1.0, material_1 )));

    let material_2 = Arc::new( Lambertian::new( &Color( 0.4, 0.2, 0.1 )));
    world.add( Arc::new( Sphere::new( Point3( -4.0, 1.0, 0.0), 1.0, material_2 )));

    let material_3 = Arc::new( Metal::new( &Color( 0.4, 0.2, 0.1 ), 0.0 ));
    world.add( Arc::new( Sphere::new( Point3( 4.0, 1.0, 0.0), 1.0, material_3 )));


    let mut cam: Camera = Default::default();

    // Going to take a lottttt with these settings!!!
    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth         = 50;

    cam.vfov     = 20.0;
    cam.lookfrom = Point3( 13.0, 2.0, 3.0 );
    cam.lookat   = Point3( 0.0, 0.0, 0.0 );
    cam.vup      = Vec3f( 0.0, 1.0, 0.0 );

    cam.defocus_angle = 0.6;
    cam.focus_dist    = 10.0;

    cam.initializer();

    //cam.render( world );
    let mut multithread_camera = Multithread_camera::new( cam );
    multithread_camera.render_multithread( world );
}

fn one_weekend_last_render() {

    let world = Box::leak( Box::new(Hittable_list::new() ));

    let material_ground = Arc::new( Lambertian::new( &Color( 0.8, 0.8, 0.0 )));
    let material_center = Arc::new( Lambertian::new( &Color( 0.1, 0.2, 0.5 )));

    let material_left   = Arc::new( Dielectric::new( 1.5));
    let material_bubble = Arc::new( Dielectric::new( 1.0 / 1.5));
    let material_right  = Arc::new( Metal::new( &Color( 0.8, 0.6, 0.2 ), 1.0 ));

    world.add( Arc::new( Sphere::new( Point3( 0.0, -100.5, -1.0)
                                    , 100.0, material_ground )));
    world.add( Arc::new( Sphere::new( Point3( 0.0, 0.0, -1.2)
                                    , 0.5, material_center )));
    world.add( Arc::new( Sphere::new( Point3( -1.0, 0.0, -1.0)
                                    , 0.5, material_left )));
    world.add( Arc::new( Sphere::new( Point3( -1.0, 0.0, -1.0)
                                    , 0.4, material_bubble )));
    world.add( Arc::new( Sphere::new( Point3( 1.0, 0.0, -1.0)
                                    , 0.5, material_right )));

    let mut cam: Camera = Default::default();

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 50;

    cam.vfov     = 20.0;
    cam.lookfrom = Point3( -2.0, 2.0, 1.0 );
    cam.lookat   = Point3( 0.0, 0.0, -1.0 );
    cam.vup      = Vec3f( 0.0, 1.0, 0.0 );

    cam.defocus_angle = 0.6;
    cam.focus_dist    = 10.0;

    cam.initializer();

    // cam.render_with_flatten_for( world );
    let mut multithread_camera = Multithread_camera::new( cam );
    multithread_camera.render_multithread( world );
}
