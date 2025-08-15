use macroquad::prelude::*;
use crate::*;
use std::time::{ Instant, Duration };

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

impl TrafficLights {
    pub fn new() -> Self {
        Self {
            top_light: false,
            down_light: false,
            left_light: false,
            right_light: false,
            timestamp: Instant::now(),
            direction: Direction::Right,
            state: false,
            duration: None,
        }
    }
    pub fn on_light(&mut self, direction: Direction) {
        match direction {
            Direction::Right => {
                self.right_light = true;
                self.top_light = false;
                self.left_light = false;
                self.down_light = false;
            }
            Direction::Up => {
                self.top_light = true;
                self.right_light = false;
                self.left_light = false;
                self.down_light = false;
            }
            Direction::Left => {
                self.left_light = true;
                self.top_light = false;
                self.right_light = false;
                self.down_light = false;
            }
            Direction::Down => {
                self.down_light = true;
                self.left_light = false;
                self.top_light = false;
                self.right_light = false;
            }
        }
        self.state = true;
        self.timestamp = Instant::now();
    }
    pub fn prioritize_vehicle(&mut self, vehicles: &Vec<Vehicle>) {
        
        let current_on_duration = if let Some(delai) = self.duration {
            ON_DURATION + delai
        } else {
            ON_DURATION
        };
        
        let elapsed: Duration = self.timestamp.elapsed();
        if self.state {
            if elapsed >= current_on_duration {
                self.state = false;
                self.timestamp = Instant::now();
                self.duration = None; // Reset
            }
        } else {
            if elapsed >= OFF_DURATION {
                self.direction = match self.direction {
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Right,
                };

                let vehicle_count = vehicles_count_by_dir(vehicles, self.direction);
                if vehicle_count >= CAPACITY {
                    self.duration = Some(calc_duration(vehicle_count));
                } else if vehicle_count == 0 {
                    self.duration = None;
                }

                self.on_light(self.direction);
                self.state = true;
            }
        }
    }
}
