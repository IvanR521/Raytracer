#![allow(clippy::float_cmp)]
pub use image::Rgb;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::sync::Arc;
// use std;
mod vec3;
pub use vec3::*;
mod ray;
pub use ray::Ray;
mod hit;
use hit::*;
mod camera;
use camera::Camera;
mod material;
use material::*;
pub const INF: f64 = std::f64::MAX;
pub const PI: f64 = std::f64::consts::PI;
mod random;
use random::*;
mod texture;
// use texture::*;
mod aabb;
mod onb;
mod pdf;
use pdf::*;

// fn get_color(this_ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
//     if depth <= 0 {
//         return Vec3::zero();
//     }
//     if let Option::Some(rec) = world.hit(this_ray, 0.001, INF) {
//         // let target = rec.p + random_in_hemisphere(rec.nor);
//         if let Option::Some((atten_col, scattered)) = rec.mat_ptr.scatter(this_ray, &rec) {
//             return get_color(&scattered, world, depth - 1).change(atten_col);
//         }
//         return Vec3::zero();
//     }
//     let unit_dir = this_ray.dir.unit();
//     let k: f64 = (unit_dir.y + 1.0) / 2.0;
//     (Vec3::new(1.0, 1.0, 1.0) * (1.0 - k)) + (Vec3::new(0.5, 0.7, 1.0) * k)
// }

fn get_color(
    this_ray: &Ray,
    background: Vec3,
    world: &HittableList,
    lights: Arc<dyn Hittable>,
    depth: i32,
) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }
    if let Option::Some(rec) = world.hit(this_ray, 0.001, INF) {
        let emitted = rec.mat_ptr.emitted(this_ray, &rec, rec.u, rec.v, rec.p);
        if let Option::Some(srec) = rec.mat_ptr.scatter(this_ray, &rec) {
            if srec.is_specular {
                return srec.atten_col.change(get_color(
                    &srec.specular_ray,
                    background,
                    world,
                    lights.clone(),
                    depth - 1,
                ));
            }
            let light_ptr = Arc::new(HittablePDF::new(lights.clone(), rec.p));
            let p = MixturePDF::new(light_ptr, srec.pdf_ptr);
            // let on_light = Vec3::new(get_rand(213.0, 343.0), 554.0, get_rand(227.0, 332.0));
            // let to_light = on_light - rec.p;
            // let _dis_squared = to_light.squared_length();
            // let to_light = to_light.unit();
            // if to_light * rec.nor < 0.0 {
            //     return emitted;
            // }
            // let _light_area = (343.0 - 213.0) * (332.0 - 227.0);
            // let light_cos = to_light.y.abs();
            // if light_cos < 0.000001 {
            //     return emitted;
            // }
            // let light_shape = Arc::new(XzRect::new(
            //     213.0,
            //     343.0,
            //     227.0,
            //     332.0,
            //     554.0,
            //     Arc::new(DiffuseLight::new(Vec3::new(15.0, 15.0, 15.0))),
            // ));
            // let p0 = Arc::new(HittablePDF::new(light_shape, rec.p));
            // let p1 = Arc::new(CosPDF::new(rec.nor));
            // let p = MixturePDF::new(p0, p1);

            let scattered = Ray::new(rec.p, p.generate(), this_ray.tm);
            let pdf = p.value(scattered.dir);
            return emitted
                + get_color(&scattered, background, world, lights.clone(), depth - 1)
                    .change(srec.atten_col)
                    * rec.mat_ptr.scattering_pdf(this_ray, &rec, &scattered)
                    / pdf;
        }
        emitted
    } else {
        background
    }
}
/*
pub fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::newarc(checker)),
    )));

    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = get_rand01();
            let center = Vec3::new(
                i as f64 + 0.9 * get_rand01(),
                0.2,
                j as f64 + 0.9 * get_rand01(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random01().change(Vec3::random01());
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, get_rand(0.0, 0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = get_rand(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

pub fn two_spheres() -> HittableList {
    let mut objects = HittableList::default();
    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::newarc(checker.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::newarc(checker.clone())),
    )));
    objects
}

pub fn simple_light() -> HittableList {
    let mut objects = HittableList::default();
    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::newarc(checker.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::newarc(checker.clone())),
    )));
    let difflight = Arc::new(DiffuseLight::new(Vec3::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));
    objects
}
*/
pub fn cornellbox() -> HittableList {
    let mut objects = HittableList::default();

    let red = Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Vec3::new(15.0, 15.0, 15.0)));

    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(FlipFace::new(Arc::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )))));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    // let ns = 1000;
    // let mut boxes0 = HittableList::default();
    // for i in 0..ns {
    //     boxes0.add(Arc::new(Sphere::new(
    //         Vec3::random(165.0, 330.0),
    //         10.0,
    //         white.clone(),
    //     )));
    // }
    // objects.add(Arc::new(BvhNode::new(boxes0.objects, ns, 0.0, 1.0)));
    let _aluminum = Arc::new(Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.0));
    let box1 = Arc::new(Bbox::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        // aluminum.clone(),
        white,
    ));
    let box1 = Arc::new(Rotatey::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1);

    let glass_sphere = Arc::new(Sphere::new(
        Vec3::new(190.0, 90.0, 190.0),
        90.0,
        Arc::new(Dielectric::new(1.5)),
    ));

    objects.add(glass_sphere);
    /*let box2 = Arc::new(Bbox::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white,
    ));
    let box2 = Arc::new(Rotatey::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    objects.add(box2);*/

    objects
}

fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let aspect_ratio: f64 = 1.0;
    let image_height: u32 = 600;
    let sam_num: i32 = 1000;
    let max_dep: i32 = 50;

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    let world = cornellbox();
    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vfov = 40.0;
    let aperture = 0.0;
    let background = Vec3::zero();

    // {
    //     //Case 1:
    //     world = random_scene();
    //     background = Vec3 :: new(0.70, 0.80, 1.00);
    //     lookfrom = Vec3::new(13.0, 2.0, 3.0);
    //     lookat = Vec3::new(0.0, 0.0, 0.0);
    //     vfov = 20.0;
    //     aperture = 0.1;
    // }
    // {
    //     //Case 2:
    //     world = two_spheres();
    //     background = Vec3 :: new(0.70, 0.80, 1.00);
    //     lookfrom = Vec3::new(13.0, 2.0, 3.0);
    //     lookat = Vec3::new(0.0, 0.0, 0.0);
    //     vfov = 20.0;
    // }
    // {
    //     //Case 5:
    //     world = simple_light();
    //     sam_num = 400;
    //     background = Vec3::zero();
    //     lookfrom = Vec3::new(26.0, 3.0, 6.0);
    //     lookat = Vec3::new(0.0, 2.0, 0.0);
    //     vfov = 20.0;
    // }
    {
        //Case 6:
        // world = cornellbox();
        // aspect_ratio = 1.0;
        // image_height = 600;
        // sam_num = 200;
        // background = Vec3::zero();
        // lookfrom = Vec3::new(278.0, 278.0, -800.0);
        // lookat = Vec3::new(278.0, 278.0, 0.0);
        // vfov = 40.0;
    }
    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let image_width: u32 = ((image_height as f64) * aspect_ratio) as u32;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(image_width as u64);
    // reflect(Vec3::ones(), Vec3::ones());

    let mut lights = HittableList::default();

    let light_shape = Arc::new(XzRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::new(NOMaterial {}),
    ));
    lights.add(light_shape);

    let glass_sphere = Arc::new(Sphere::new(
        Vec3::new(190.0, 90.0, 190.0),
        90.0,
        Arc::new(NOMaterial {}),
    ));
    lights.add(glass_sphere);

    let lights = Arc::new(lights);

    for x in 0..image_width {
        for y in 0..image_height {
            let pixel = img.get_pixel_mut(x, image_height - 1 - y);
            let mut color: Vec3 = Vec3::zero();
            for _i in 0..sam_num {
                let dx = (x as f64 + get_rand01()) / (image_width as f64);
                let dy = (y as f64 + get_rand01()) / (image_height as f64);
                let this_ray = cam.get_ray(dx, dy);
                color += get_color(&this_ray, background, &world, lights.clone(), max_dep);
            }
            *pixel = Rgb([
                ((color.x / sam_num as f64).sqrt() * 255.0) as u8,
                ((color.y / sam_num as f64).sqrt() * 255.0) as u8,
                ((color.z / sam_num as f64).sqrt() * 255.0) as u8,
            ]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
