use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use std::time::{ Duration, Instant };
use std::collections::VecDeque;
use rand::Rng;

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 800;
const ROAD_WIDTH: i32 = 60;
const LANE_WIDTH: i32 = 30;
const VEHICLE_SIZE: i32 = 30;
const SAFETY_GAP: i32 = 15;
const VEHICLE_SPEED: i32 = 2;
const SPAWN_COOLDOWN: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Route {
    Straight,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Vehicle {
    x: f32,
    y: f32,
    direction: Direction,
    route: Route,
    color: Color,
    has_turned: bool,
}

struct Lane {
    vehicles: VecDeque<Vehicle>,
    direction: Direction,
    capacity: usize,
    last_spawn: Instant,
}

impl Lane {
    fn new(direction: Direction) -> Self {
        let lane_length = match direction {
            Direction::North | Direction::South => ((WINDOW_HEIGHT as i32) - ROAD_WIDTH) / 2,
            Direction::East | Direction::West => ((WINDOW_WIDTH as i32) - ROAD_WIDTH) / 2,
        };
        let capacity = (lane_length / (VEHICLE_SIZE + SAFETY_GAP)) as usize;
        Self {
            vehicles: VecDeque::new(),
            direction,
            capacity: capacity.max(1),
            last_spawn: Instant::now(),
        }
    }
    fn can_spawn(&self) -> bool {
        self.last_spawn.elapsed() >= SPAWN_COOLDOWN && self.vehicles.len() < self.capacity
    }
    fn spawn_vehicle(&mut self) {
        if !self.can_spawn() {
            return;
        }
        let mut rng = rand::thread_rng();
        let route = match rng.gen_range(0..3) {
            0 => Route::Straight,
            1 => Route::Left,
            _ => Route::Right,
        };
        let color = get_route_color(route);
        let (x, y) = self.get_spawn_position();
        let vehicle = Vehicle {
            x,
            y,
            direction: self.direction,
            route,
            color,
            has_turned: false,
        };
        self.vehicles.push_back(vehicle);
        self.last_spawn = Instant::now();
    }
    fn get_spawn_position(&self) -> (f32, f32) {
        let center_x = (WINDOW_WIDTH as f32) / 2.0;
        let center_y = (WINDOW_HEIGHT as f32) / 2.0;
        match self.direction {
            Direction::North =>
                (center_x + (LANE_WIDTH as f32) / 2.0, (WINDOW_HEIGHT as f32) - 30.0),
            Direction::South => (center_x - (LANE_WIDTH as f32) / 2.0, 30.0),
            Direction::East => (30.0, center_y + (LANE_WIDTH as f32) / 2.0),
            Direction::West => ((WINDOW_WIDTH as f32) - 30.0, center_y - (LANE_WIDTH as f32) / 2.0),
        }
    }

    fn update(&mut self) {
        let mut to_remove = Vec::new();
        let mut movements = Vec::new();
        for (i, vehicle) in self.vehicles.iter().enumerate() {
            let mut can_move = true;
            if i > 0 {
                let front_vehicle = &self.vehicles[i - 1];
                let distance = calculate_distance(*vehicle, *front_vehicle);
                if distance < (SAFETY_GAP as f32) + (VEHICLE_SIZE as f32) {
                    can_move = false;
                }
            }
            movements.push(can_move);
        }
        for (i, vehicle) in self.vehicles.iter_mut().enumerate() {
            if movements[i] {
                move_vehicle(vehicle);

                if vehicle_off_screen(*vehicle) {
                    to_remove.push(i);
                }
            }
        }
        for &i in to_remove.iter().rev() {
            self.vehicles.remove(i);
        }
    }
}

fn calculate_distance(v1: Vehicle, v2: Vehicle) -> f32 {
    ((v1.x - v2.x).powi(2) + (v1.y - v2.y).powi(2)).sqrt()
}

fn move_vehicle(vehicle: &mut Vehicle) {
    match vehicle.direction {
        Direction::North => {
            vehicle.y -= VEHICLE_SPEED as f32;
        }
        Direction::South => {
            vehicle.y += VEHICLE_SPEED as f32;
        }
        Direction::East => {
            vehicle.x += VEHICLE_SPEED as f32;
        }
        Direction::West => {
            vehicle.x -= VEHICLE_SPEED as f32;
        }
    }

    handle_route_change(vehicle);
}

fn handle_route_change(vehicle: &mut Vehicle) {
    let center_x = (WINDOW_WIDTH as f32) / 2.0;
    let center_y = (WINDOW_HEIGHT as f32) / 2.0;

    if vehicle.route != Route::Straight && !vehicle.has_turned {
        let should_turn = match (vehicle.direction, vehicle.route) {
            (Direction::North, Route::Left) => vehicle.y <= center_y,
            (Direction::South, Route::Left) => vehicle.y >= center_y,
            (Direction::East, Route::Left) => vehicle.x >= center_x,
            (Direction::West, Route::Left) => vehicle.x <= center_x,

            (Direction::North, Route::Right) => vehicle.y <= center_y + (VEHICLE_SIZE as f32),
            (Direction::South, Route::Right) => vehicle.y >= center_y - (VEHICLE_SIZE as f32),
            (Direction::East, Route::Right) => vehicle.x >= center_x - (VEHICLE_SIZE as f32),
            (Direction::West, Route::Right) => vehicle.x <= center_x + (VEHICLE_SIZE as f32),

            _ => false,
        };

        if should_turn {
            match vehicle.route {
                Route::Left => {
                    vehicle.direction = match vehicle.direction {
                        Direction::North => Direction::West,
                        Direction::South => Direction::East,
                        Direction::East => Direction::North,
                        Direction::West => Direction::South,
                    };
                }
                Route::Right => {
                    vehicle.direction = match vehicle.direction {
                        Direction::North => Direction::East,
                        Direction::South => Direction::West,
                        Direction::East => Direction::South,
                        Direction::West => Direction::North,
                    };
                }
                _ => {}
            }
            vehicle.has_turned = true;
        }
    }
}

fn vehicle_off_screen(vehicle: Vehicle) -> bool {
    vehicle.x < -(VEHICLE_SIZE as f32) ||
        vehicle.x > (WINDOW_WIDTH as f32) + (VEHICLE_SIZE as f32) ||
        vehicle.y < -(VEHICLE_SIZE as f32) ||
        vehicle.y > (WINDOW_HEIGHT as f32) + (VEHICLE_SIZE as f32)
}

fn get_route_color(route: Route) -> Color {
    match route {
        Route::Straight => Color::RGB(0, 255, 0),
        Route::Left => Color::RGB(255, 255, 0),
        Route::Right => Color::RGB(255, 165, 0),
    }
}

struct TrafficSimulation {
    lanes: [Lane; 4],
}

impl TrafficSimulation {
    fn new() -> Self {
        Self {
            lanes: [
                Lane::new(Direction::North),
                Lane::new(Direction::South),
                Lane::new(Direction::East),
                Lane::new(Direction::West),
            ],
        }
    }
    fn update(&mut self) {
        for lane in &mut self.lanes {
            lane.update();
        }
    }
    fn spawn_vehicle(&mut self, direction: Direction) {
        let lane_index = match direction {
            Direction::North => 0,
            Direction::South => 1,
            Direction::East => 2,
            Direction::West => 3,
        };
        self.lanes[lane_index].spawn_vehicle();
    }

    fn spawn_random_vehicle(&mut self) {
        let mut rng = rand::thread_rng();
        let direction = match rng.gen_range(0..4) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            _ => Direction::West,
        };
        self.spawn_vehicle(direction);
    }

    fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.clear();
        self.draw_roads(canvas)?;
        self.draw_vehicles(canvas)?;
        canvas.present();
        Ok(())
    }

    fn draw_roads(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        let center_x = (WINDOW_WIDTH as i32) / 2;
        let center_y = (WINDOW_HEIGHT as i32) / 2;
        let h_road = Rect::new(0, center_y - ROAD_WIDTH / 2, WINDOW_WIDTH, ROAD_WIDTH as u32);
        canvas.fill_rect(h_road)?;
        let v_road = Rect::new(center_x - ROAD_WIDTH / 2, 0, ROAD_WIDTH as u32, WINDOW_HEIGHT);
        canvas.fill_rect(v_road)?;
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let intersection_half_size = ROAD_WIDTH / 2 + 10;
        for x in (0..WINDOW_WIDTH as i32).step_by(20) {
            if !(x > center_x - intersection_half_size && x < center_x + intersection_half_size) {
                let rect = Rect::new(x, center_y - 1, 10, 2);
                canvas.fill_rect(rect)?;
            }
        }
        for y in (0..WINDOW_HEIGHT as i32).step_by(20) {
            if !(y > center_y - intersection_half_size && y < center_y + intersection_half_size) {
                let rect = Rect::new(center_x - 1, y, 2, 10);
                canvas.fill_rect(rect)?;
            }
        }

        Ok(())
    }

    fn draw_vehicles(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let center_x = (WINDOW_WIDTH as i32) / 2;
        let center_y = (WINDOW_HEIGHT as i32) / 2;
        let vehicle_half = VEHICLE_SIZE / 2;

        for lane in &self.lanes {
            for vehicle in &lane.vehicles {
                canvas.set_draw_color(vehicle.color);

                let (x, y) = match vehicle.direction {
                    Direction::North => (center_x + vehicle_half, vehicle.y as i32),
                    Direction::South => (center_x - vehicle_half, vehicle.y as i32),
                    Direction::East => (vehicle.x as i32, center_y + vehicle_half),
                    Direction::West => (vehicle.x as i32, center_y - vehicle_half),
                };

                let rect = Rect::new(
                    x - vehicle_half,
                    y - vehicle_half,
                    VEHICLE_SIZE as u32,
                    VEHICLE_SIZE as u32
                );
                canvas.fill_rect(rect)?;
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Traffic Intersection Simulation", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build().expect("could not make a rendering context");
    let mut event_pump = sdl_context.event_pump()?;
    let mut simulation = TrafficSimulation::new();
    println!("Traffic Intersection Simulation");
    println!("Controls:");
    println!("↑ - Spawn vehicle from South");
    println!("↓ - Spawn vehicle from North");
    println!("→ - Spawn vehicle from West");
    println!("← - Spawn vehicle from East");
    println!("R - Spawn random vehicle");
    println!("ESC - Exit simulation");
    println!("\nVehicle Colors:");
    println!("Green - Going Straight");
    println!("Yellow - Turning Left");
    println!("Orange - Turning Right");
    let mut last_spawn_time = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(keycode), repeat, .. } => {
                    if !repeat && last_spawn_time.elapsed() >= Duration::from_millis(700) {
                        match keycode {
                            Keycode::Up => simulation.spawn_vehicle(Direction::North),
                            Keycode::Down => simulation.spawn_vehicle(Direction::South),
                            Keycode::Right => simulation.spawn_vehicle(Direction::East),
                            Keycode::Left => simulation.spawn_vehicle(Direction::West),
                            Keycode::R => simulation.spawn_random_vehicle(),
                            _ => {}
                        }
                        last_spawn_time = Instant::now();
                    }
                }
                _ => {}
            }
        }
        simulation.update();
        simulation.render(&mut canvas)?;
        std::thread::sleep(Duration::from_millis(30));
    }
    Ok(())
}
