use raylib::prelude::*;

const WIDTH: i32 = 100;
const HEIGHT: i32 = 100;

const DEAD: Color = Color { r: 0, g: 255, b: 255, a: 255 };  
const ALIVE: Color = Color { r: 255, g: 105, b: 180, a: 255 }; 


static mut INITIALIZED: bool = false;

pub fn point(image: &mut Image, x: i32, y: i32, color: Color) {
    if x >= 0 && x < image.width() && y >= 0 && y < image.height() {
        image.draw_pixel(x, y, color);
    }
}

fn is_alive(color: Color) -> bool {
    color.r > 127 
}

fn get_color(image: &Image, x: i32, y: i32) -> Color {
    let index = (y * image.width + x) * 4;

    unsafe {
        let data = image.data as *mut u8;

        Color {
            r: *data.add(index as usize),
            g: *data.add(index as usize + 1),
            b: *data.add(index as usize + 2),
            a: *data.add(index as usize + 3),
        }
    }
}



fn count_alive_neighbors(image: &Image, x: i32, y: i32) -> i32 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if is_alive(get_color(image, x + dx, y + dy)) {
                count += 1;
            }
        }
    }
    count
}

fn draw_block(image: &mut Image, x: i32, y: i32) {
    let pattern = [(0,0), (0,1), (1,0), (1,1)];
    for (dx, dy) in pattern {
        point(image, x + dx, y + dy, ALIVE);
    }
}

fn draw_beehive(image: &mut Image, x: i32, y: i32) {
    let pattern = [(1,0), (2,0), (0,1), (3,1), (1,2), (2,2)];
    for (dx, dy) in pattern {
        point(image, x + dx, y + dy, ALIVE);
    }
}

fn draw_loaf(image: &mut Image, x: i32, y: i32) {
    let pattern = [(1,0), (2,0), (0,1), (3,1), (1,2), (3,2), (2,3)];
    for (dx, dy) in pattern {
        point(image, x + dx, y + dy, ALIVE);
    }
}

fn draw_boat(image: &mut Image, x: i32, y: i32) {
    let pattern = [(0,0), (1,0), (0,1), (2,1), (1,2)];
    for (dx, dy) in pattern {
        point(image, x + dx, y + dy, ALIVE);
    }
}

fn draw_tub(image: &mut Image, x: i32, y: i32) {
    let pattern = [(1,0), (0,1), (2,1), (1,2)];
    for (dx, dy) in pattern {
        point(image, x + dx, y + dy, ALIVE);
    }
}

fn draw_blinker(image: &mut Image, x: i32, y: i32) {
    let pattern = [(0,1), (1,1), (2,1)];
    for (dx, dy) in pattern {
        point(image, x + dx, y + dy, ALIVE);
    }
}

fn draw_toad(image: &mut Image, x: i32, y: i32) {
    let pattern = [(1,0), (2,0), (3,0), (0,1), (1,1), (2,1)];
    for (dx, dy) in pattern {
        point(image, x + dx, y + dy, ALIVE);
    }
}

fn draw_beacon(image: &mut Image, x: i32, y: i32) {
    let pattern = [(0,0), (1,0), (0,1), (1,1), (2,2), (3,2), (2,3), (3,3)];
    for (dx, dy) in pattern {
        point(image, x + dx, y + dy, ALIVE);
    }
}

fn draw_pulsar(image: &mut Image, x: i32, y: i32) {
    let coords = [
        (2,0), (3,0), (4,0), (8,0), (9,0), (10,0),
        (0,2), (5,2), (7,2), (12,2),
        (0,3), (5,3), (7,3), (12,3),
        (0,4), (5,4), (7,4), (12,4),
        (2,5), (3,5), (4,5), (8,5), (9,5), (10,5),
        (2,7), (3,7), (4,7), (8,7), (9,7), (10,7),
        (0,8), (5,8), (7,8), (12,8),
        (0,9), (5,9), (7,9), (12,9),
        (0,10), (5,10), (7,10), (12,10),
        (2,12), (3,12), (4,12), (8,12), (9,12), (10,12),
    ];
    for (dx, dy) in coords {
        point(image, x + dx, y + dy, ALIVE);
    }
}

fn draw_glider(image: &mut Image, x: i32, y: i32) {
    let pattern = [(1,0), (2,1), (0,2), (1,2), (2,2)];
    for (dx, dy) in pattern {
        point(image, x + dx, y + dy, ALIVE);
    }
}

fn draw_lwss(image: &mut Image, x: i32, y: i32) {
    let pattern = [(1,0), (4,0), (0,1), (0,2), (4,2), (0,3), (1,3), (2,3), (3,3)];
    for (dx, dy) in pattern {
        point(image, x + dx, y + dy, ALIVE);
    }
}

fn draw_mwss(image: &mut Image, x: i32, y: i32) {
    let pattern = [(1,0), (4,0), (5,1), (0,2), (0,3), (5,3), (0,4), (1,4), (2,4), (3,4), (4,4)];
    for (dx, dy) in pattern {
        point(image, x + dx, y + dy, ALIVE);
    }
}


pub fn render(image: &mut Image) {
    unsafe {
        if !INITIALIZED {

            draw_glider(image, 5, 5);             
            draw_glider(image, 5, 85);          
            draw_glider(image, 85, 85);           
            draw_lwss(image, 60, 60);            
            draw_mwss(image, 30, 10);           
            draw_beacon(image, 10, 40);        
            draw_blinker(image, 50, 80);        
            draw_block(image, 80, 80);            
            draw_beehive(image, 15, 75);     
            draw_boat(image, 60, 40);            
            draw_loaf(image, 30, 70);            
            draw_tub(image, 75, 15);    
            draw_toad(image, 40, 60);
            draw_pulsar(image, 20, 20);
            draw_pulsar(image, 60, 20);
            draw_pulsar(image, 20, 60);
            draw_pulsar(image, 60, 60);
            



            INITIALIZED = true;
           
        }
    }

    let mut next_frame = vec![vec![false; WIDTH as usize]; HEIGHT as usize];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let alive = is_alive(get_color(image, x, y));
            let neighbors = count_alive_neighbors(image, x, y);
            next_frame[y as usize][x as usize] = match (alive, neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = if next_frame[y as usize][x as usize] {
                ALIVE
            } else {
                DEAD
            };
            point(image, x, y, color);
        }
    }

    std::thread::sleep(std::time::Duration::from_millis(50));
}



fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("Conway's Game of Life")
        .build();

    rl.set_target_fps(60);

    let mut image = Image::gen_image_color(WIDTH, HEIGHT, DEAD);

    while !rl.window_should_close() {
        render(&mut image);

        let texture = rl.load_texture_from_image(&thread, &image).expect("Failed to load texture");

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture_ex(
            &texture,
            Vector2::new(0.0, 0.0),
            0.0,
            8.0,
            Color::WHITE,
        );
    }
}
