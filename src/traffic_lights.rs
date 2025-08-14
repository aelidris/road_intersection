use macroquad::prelude::*;
use crate::*;
use std::time::{Instant, Duration};

pub struct TrafficLights {
    pub top_light: bool,
    pub down_light: bool,
    pub left_light: bool,
    pub right_light: bool,
    pub direction: Direction,
    pub state: bool,
    pub timestamp: Instant,
    pub duration: Option<Duration>,
}


pub fn road_vehicles_count(vehicles: &Vec<Vehicle>, direction: Direction) -> usize {
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

pub fn traffic_lights(vehicle: &mut Vehicle, lights: &TrafficLights) {
    if
        (!lights.down_light && vehicle.dir == Direction::Down && vehicle.y == 360) ||
        (!lights.top_light && vehicle.dir == Direction::Up && vehicle.y == 210) ||
        (!lights.right_light && vehicle.dir == Direction::Right && vehicle.x == 310) ||
        (!lights.left_light && vehicle.dir == Direction::Left && vehicle.x == 460)
    {
        vehicle.moving = false;
    } else {
        vehicle.moving = true;
    }
}

pub fn prioritize_vehicle(lights: &mut TrafficLights, vehicles: &Vec<Vehicle>) {
    let elapsed = lights.timestamp.elapsed();

    let current_on_duration = if let Some(delai) = lights.duration {
        ON_DURATION + delai
    } else {
        ON_DURATION
    };

    if lights.state {
        if elapsed >= current_on_duration {
            lights.right_light = false;
            lights.top_light = false;
            lights.left_light = false;
            lights.down_light = false;

            lights.state = false;
            lights.timestamp = Instant::now();
            lights.duration = None; // Reset extension
        }
    } else {
        if elapsed >= OFF_DURATION {
            lights.direction = match lights.direction {
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
            };

            let vehicle_count = road_vehicles_count(vehicles, lights.direction);
            if vehicle_count >= CAPACITY {
                lights.duration = Some(calc_duration(vehicle_count));
            } else if vehicle_count == 0 {
                lights.duration = None;
            }

            match lights.direction {
                Direction::Right => {
                    lights.right_light = true;
                    lights.top_light = false;
                    lights.left_light = false;
                    lights.down_light = false;
                }
                Direction::Up => {
                    lights.right_light = false;
                    lights.top_light = true;
                    lights.left_light = false;
                    lights.down_light = false;
                }
                Direction::Left => {
                    lights.right_light = false;
                    lights.top_light = false;
                    lights.left_light = true;
                    lights.down_light = false;
                }
                Direction::Down => {
                    lights.right_light = false;
                    lights.top_light = false;
                    lights.left_light = false;
                    lights.down_light = true;
                }
            }

            lights.state = true;
            lights.timestamp = Instant::now();
        }
    }
}

pub fn on_light(direction: Direction, lights: &mut TrafficLights) {
    match direction {
        Direction::Right => {
            lights.right_light = true;
            lights.top_light = false;
            lights.left_light = false;
            lights.down_light = false;
        }
        Direction::Up => {
            lights.top_light = true;
            lights.right_light = false;
            lights.left_light = false;
            lights.down_light = false;
        }
        Direction::Left => {
            lights.left_light = true;
            lights.top_light = false;
            lights.right_light = false;
            lights.down_light = false;
        }
        Direction::Down => {
            lights.down_light = true;
            lights.left_light = false;
            lights.top_light = false;
            lights.right_light = false;
        }
    }
    lights.state = true;
    lights.timestamp = Instant::now();
}
