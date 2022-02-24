use sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Point;

use std::time::Duration;

fn create_pallatte() -> Vec<Color> {
    let mut colors = vec![Color::BLACK; 50];

    let mut lightness = 1;
    for mut color in &mut colors {
        color.r += lightness;
        color.g += lightness;
        color.b += lightness;
        lightness += 5;
    }

    return colors;
}

fn mandlebrot_buf(width: usize, height: usize, max_iterations: u16) -> Vec<Vec<Color>> {
    let mut buf = vec![vec![Color::BLACK; width]; height];

    let colors = create_pallatte();

    let mut y_index = 0;
    for y in buf.clone() {
        let mut x_index = 0;
        for _x in y {
            if y_index > 0 && x_index > 0 {
                let x0 = ((x_index as f32 / height as f32) * 2.47) - 2.25;
                let y0 = ((y_index as f32 / width as f32) * 2.47) - 1.0;
            
                let mut x_t: f32 = 0.0;
                let mut y_t: f32 = 0.0;

                let mut iteration = 0;
                while x_t*x_t + y_t*y_t < 4.0 && iteration < max_iterations - 1 {
                    let xtemp = (x_t*x_t) as f32 - (y_t*y_t) as f32 + x0;
                    y_t = 2.0*x_t*y_t + y0;
                    x_t = xtemp;
                    iteration += 1;
                }

                let color = colors[iteration as usize];
                println!("Pixel {}, {} finished with {} iterations and color {:?}", x_index, y_index, iteration, color);
                buf[y_index - 1][x_index - 1] = color;  
            }
            x_index += 1;

            if x_index == width {
                break;
            }
        }
        y_index += 1;
    }

    buf
} 

fn main() {
    let ctx = sdl2::init().unwrap();
    let subsys = ctx.video().unwrap();

    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    let window = subsys.window("Mandlebrot Set", WIDTH.try_into().unwrap(), HEIGHT.try_into().unwrap()).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = ctx.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                _ => (),
            }
        }

        let buf = mandlebrot_buf(WIDTH, HEIGHT, 50);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                    canvas.set_draw_color(buf[y][x]);
                    canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();      
                }
            }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
