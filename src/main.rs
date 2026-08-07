use macroquad::prelude::*;

fn generate_texture(pixels: Vec<Color>, width: u32, height: u32) -> Texture2D
{

    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);

    for px in 0 .. width
    {
        for py in 0 .. height
        {
            let index = (px * height + py) as usize;
            image.set_pixel(px, py, pixels[index]);
        }
    }

    let texture = Texture2D::from_image(&image);
    texture

}

fn scale(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32
{

    let right_scale: f32 = out_min + (((value - in_min) / (in_max - in_min)) * (out_max - out_min));

    right_scale

}

fn mandelbrot(center_x: f32, center_y: f32, range_width: f32, range_height: f32, max_iteration: u32) -> Vec<Color>
{

    let mut pixels: Vec<Color> = Vec::new();

    let screen_width  = screen_width();
    let screen_height  = screen_height();

    let x_min = center_x - range_width / 2.0;
    let x_max = center_x + range_width / 2.0;
    let y_min = center_y - range_height / 2.0;
    let y_max = center_y + range_height / 2.0;

    for px in 0..screen_width as i32
    {
        for py in 0..screen_height as i32
        {
            let x0 = scale(px as f32, 0.00, screen_width as f32, x_min, x_max);
            let y0 = scale(py as f32, 0.00, screen_height as f32, y_min, y_max);

            let mut x: f32 = 0.00;
            let mut y: f32 = 0.00;

            let mut iteration: u32 = 0;

            while x * x + y * y <= 4.0 && iteration < max_iteration
            {
                let xtemp = x * x - y * y + x0;
                y = 2.0 * x *y + y0;
                x = xtemp;

                iteration += 1;

            }

            let fraction: f32;

            let mut r: f32;
            let mut g: f32;
            let mut b: f32;

            if iteration == max_iteration
            {
                r = 0.0;
                g = 0.0;
                b = 0.0;
            }

            else 
            {

                fraction = iteration as f32 / max_iteration as f32;
                let frequency: f32 = 6.0;

                let fase_r: f32 = 0.0;
                let fase_g: f32 = 2.0;
                let fase_b: f32 = 4.0;

                r = 0.5 + 0.5 * (frequency * fraction + fase_r).sin();
                g = 0.5 + 0.5 * (frequency * fraction + fase_g).sin();
                b = 0.5 + 0.5 * (frequency * fraction + fase_b).sin();

                let brightness = fraction.sqrt();

                r = r * brightness;
                g = g * brightness;
                b = b * brightness;

            }

            pixels.push(Color::new(r, g, b, 1.0));

        }
    }
    return pixels
}

#[macroquad::main("Mandelbrot")]
async fn main() 
{

    let center_x: f32 = -0.765;
    let center_y: f32 = 0.0;
    let mut range_width: f32 = 2.47;
    let mut range_height: f32 = 2.24;

    let mut was_pressed: bool = false;

    let mut calculated_pixels = mandelbrot(center_x, center_y, range_width, range_height, 1000);

    let screen_width  = screen_width();
    let screen_height  = screen_height();

    let mut texture = generate_texture(calculated_pixels, screen_width as u32, screen_height as u32);

    loop 
    {
        let last_frame = get_frame_time();
        let mut changed: bool = false;
        let is_pressed_now: bool = is_key_down(KeyCode::E) || is_key_down(KeyCode::Q);

        if is_key_down(KeyCode::Q)
        {
            range_width *= 0.85f32.powf(last_frame);
            range_height *= 0.85f32.powf(last_frame);
            changed = true;
        }

        if is_key_down(KeyCode::E)
        {
            range_width *= 1.5f32.powf(last_frame);
            range_height *= 1.5f32.powf(last_frame);
            changed = true;
        }

        if changed
        {
            calculated_pixels = mandelbrot(center_x, center_y, range_width, range_height, 100);
            texture = generate_texture(calculated_pixels, screen_width as u32, screen_height as u32);
        }

        else if was_pressed && !is_pressed_now
        {
            calculated_pixels = mandelbrot(center_x, center_y, range_width, range_height, 1000);
            texture = generate_texture(calculated_pixels, screen_width as u32, screen_height as u32);
        }

        was_pressed = is_pressed_now;

        draw_texture(&texture, 0.0, 0.0, WHITE);

        next_frame().await;
    }
}
