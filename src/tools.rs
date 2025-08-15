use macroquad::{ prelude::*, rand::gen_range };
use crate::vehicles::{ Vehicle, Direction };
use crate::traffic_lights::TrafficLights;
use std::time::Duration;

pub const WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 600.0;

pub const ON_DURATION: Duration = Duration::from_secs(1);
pub const OFF_DURATION: Duration = Duration::from_millis(500);
pub const CAPACITY: usize = 5;

pub fn draw_vehicles(vehicles: &[Vehicle]) {
    for vehicle in vehicles {
        draw_rectangle(vehicle.x as f32, vehicle.y as f32, 30.0, 30.0, vehicle.color);
    }
}

pub fn draw_roads() {
    // Road sizes
    let road_width = 100.0;
    let (left, right, top, bottom) = (350.0, 450.0, 250.0, 350.0);
    let (dash_length, dash_thickness) = (15.0, 2.0);

    // Draw horizontal and vertical roads
    draw_rectangle(0.0, top, WIDTH, road_width, GRAY); // Horizontal
    draw_rectangle(left, 0.0, road_width, HEIGHT, GRAY); // Vertical

    // Helper function to draw dashed lines
    fn draw_dashed_line_vertical(
        x: f32,
        start_y: f32,
        end_y: f32,
        dash: f32,
        thickness: f32,
        color: Color
    ) {
        let mut y = start_y;
        while y < end_y {
            let next_y = (y + dash).min(end_y);
            draw_line(x, y, x, next_y, thickness, color);
            y += dash * 2.0;
        }
    }

    fn draw_dashed_line_horizontal(
        y: f32,
        start_x: f32,
        end_x: f32,
        dash: f32,
        thickness: f32,
        color: Color
    ) {
        let mut x = start_x;
        while x < end_x {
            let next_x = (x + dash).min(end_x);
            draw_line(x, y, next_x, y, thickness, color);
            x += dash * 2.0;
        }
    }

    // Vertical dashed lines (center of lanes)
    draw_dashed_line_vertical(WIDTH / 2.0, 0.0, top, dash_length, dash_thickness, YELLOW);
    draw_dashed_line_vertical(WIDTH / 2.0, bottom, HEIGHT, dash_length, dash_thickness, YELLOW);

    // Horizontal dashed lines (center of lanes)
    draw_dashed_line_horizontal(HEIGHT / 2.0, 0.0, left, dash_length, dash_thickness, YELLOW);
    draw_dashed_line_horizontal(HEIGHT / 2.0, right, WIDTH, dash_length, dash_thickness, YELLOW);

    // Draw intersection boundaries (solid lines)
    draw_line(left, 0.0, left, HEIGHT, dash_thickness, WHITE);
    draw_line(right, 0.0, right, HEIGHT, dash_thickness, WHITE);
    draw_line(0.0, top, WIDTH, top, dash_thickness, WHITE);
    draw_line(0.0, bottom, WIDTH, bottom, dash_thickness, WHITE);
}

pub fn draw_traffic_lights(lights: &TrafficLights) {
    let mut color = if lights.top_light { GREEN } else { RED };
    draw_rectangle_lines(318.0, 218.0, 30.0, 30.0, 2.0, color);

    color = if lights.right_light { GREEN } else { RED };
    draw_rectangle_lines(318.0, 352.0, 30.0, 30.0, 2.0, color);

    color = if lights.down_light { GREEN } else { RED };
    draw_rectangle_lines(452.0, 352.0, 30.0, 30.0, 2.0, color);

    color = if lights.left_light { GREEN } else { RED };
    draw_rectangle_lines(452.0, 218.0, 30.0, 30.0, 2.0, color);
}

//return 4 Vecs contains vehicule filtred by direction
pub fn vehicle_direction(
    vehicles: &mut Vec<Vehicle>
) -> (Vec<Vehicle>, Vec<Vehicle>, Vec<Vehicle>, Vec<Vehicle>) {
    let mut up = Vec::new();
    let mut down = Vec::new();
    let mut left = Vec::new();
    let mut right = Vec::new();

    for vehicle in vehicles.iter() {
        match vehicle.dir {
            Direction::Up if vehicle.x == 360 && vehicle.y <= 260 => up.push(vehicle.clone()),
            Direction::Down if vehicle.x == 410 && vehicle.y >= 410 => down.push(vehicle.clone()),
            Direction::Left if vehicle.y == 260 && vehicle.x >= 410 => left.push(vehicle.clone()),
            Direction::Right if vehicle.y == 310 && vehicle.x <= 360 => right.push(vehicle.clone()),
            _ => {}
        };
    }
    (up, down, left, right)
}

// get last vehicle by direction
pub fn last_vehicle_direction(vehicles: &mut Vec<Vehicle>, dir: Direction) -> Option<Vehicle> {
    vehicles
        .iter()
        .rev()
        .find(|v| v.dir == dir)
        .cloned()
}

// count single direction vehicles 
pub fn vehicles_count_by_dir(vehicles: &Vec<Vehicle>, direction: Direction) -> usize {
    let vehicles_vec = &mut vehicles.clone(); // Convert to Vec<vehicle>
    let (up, down, left, right) = vehicle_direction(vehicles_vec);

    let count = match direction {
        Direction::Up => up.len(),
        Direction::Down => down.len(),
        Direction::Left => left.len(),
        Direction::Right => right.len(),
    };

    println!("Lane {:?}: {} vehicles waiting", direction, count);
    count
}
// Key Event Handlers
pub fn key_up(vehicles: &mut Vec<Vehicle>) {
    let vehicle = Vehicle::new(410, 600, Direction::Down, Vehicle::random());
    vehicle.push(vehicles, Some(570), None, Direction::Down);
}

pub fn key_down(vehicles: &mut Vec<Vehicle>) {
    let vehicle = Vehicle::new(360, 0, Direction::Up, Vehicle::random());
    vehicle.push(vehicles, Some(60), None, Direction::Up);
}

pub fn key_right(vehicles: &mut Vec<Vehicle>) {
    let vehicle = Vehicle::new(-30, 310, Direction::Right, Vehicle::random());
    vehicle.push(vehicles, None, Some(30), Direction::Right);
}

pub fn key_left(vehicles: &mut Vec<Vehicle>) {
    let vehicle = Vehicle::new(800, 260, Direction::Left, Vehicle::random());
    vehicle.push(vehicles, None, Some(770), Direction::Left);
}

pub fn key_r(vehicles: &mut Vec<Vehicle>) {
    let vec: Vec<fn(&mut Vec<Vehicle>)> = vec![key_up, key_down, key_left, key_right];
    (vec[gen_range(0, 4)])(vehicles);
}

pub fn calc_duration(vehicle_count: usize) -> Duration {
    let count = match vehicle_count {
        0..=2 => Duration::from_millis(10),
        3..=4 => Duration::from_millis(30),
        5..=7 => Duration::from_millis(60),
        8..=10 => Duration::from_millis(90),
        _ => Duration::from_millis(100),
    };
    println!("{:?}", count);
    count
}
