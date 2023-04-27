mod color;
mod ray;
mod vec3;

const RAY_COLOR_1: vec3::Color = vec3::Color { e: [1.0, 1.0, 1.0] };

const RAY_COLOR_2: vec3::Color = vec3::Color { e: [0.5, 0.7, 1.0] };

fn ray_color(r: ray::Ray) -> vec3::Color {
    let unit_direction = r.direction().unit_vector();
    let t = &0.5 * &(unit_direction.y() + 1.0);
    return &(&(1.0 - t) * &RAY_COLOR_1) + &(&t * &RAY_COLOR_2);
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Point3::build(0.0, 0.0, 0.0);
    let horizontal = vec3::Vec3::build(viewport_width, 0.0, 0.0);
    let vertical = vec3::Vec3::build(0.0, viewport_height, 0.0);

    let horizontal_offset = &origin - &(&horizontal / &2.0);
    let vertical_offset = &horizontal_offset - &(&vertical / &2.0);
    let lower_left_corner = &vertical_offset - &vec3::Vec3::build(0.0, 0.0, focal_length);

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..(IMAGE_HEIGHT - 1)).rev() {
        // eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let horizontal_mod = &horizontal * &u;
            let vertical_mod = &vertical * &v;
            let r = ray::Ray::build(
                &origin,
                &(&(&lower_left_corner + &horizontal_mod) + &vertical_mod),
            );
            let pixel_color = ray_color(r);
            color::write_color(pixel_color);
        }
    }
    eprintln!("Done.");
}
