use macroquad::{ prelude::*, rand::gen_range };
use crate::{ TrafficLights, on_light, last_vehicle_direction };

// Constants
const MAX: usize = 7;
const GAP: i32 = 65;

// Enums
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Vehicle {
    pub x: i32,
    pub y: i32,
    pub dir: Direction,
    pub color: Color,
    pub turned: bool,
    pub moving: bool,
}

impl Vehicle {
    pub fn new(x: i32, y: i32, dir: Direction, color: Color) -> Vehicle {
        Vehicle {
            x,
            y,
            dir,
            color,
            turned: false,
            moving: false,
        }
    }

    pub fn update(&mut self) {
        match self.dir {
            Direction::Up => {
                self.y += 5;
            }
            Direction::Down => {
                self.y -= 5;
            }
            Direction::Left => {
                self.x -= 5;
            }
            Direction::Right => {
                self.x += 5;
            }
        }
    }

    pub fn next(&self, vehicles: &Vec<Vehicle>) -> bool {
        for vehicle in vehicles {
            if vehicle.dir == self.dir {
                if self.on_close(vehicle) {
                    return true;
                }
            }
        }
        false
    }

    pub fn random() -> Color {
        match gen_range(0, 4) {
            0 => BLUE,
            1 => YELLOW,
            _ => ORANGE,
        }
    }

    pub fn on_change(&mut self) {
        match self.dir {
            Direction::Up => self.on_up(),
            Direction::Down => self.on_down(),
            Direction::Right => self.on_right(),
            Direction::Left => self.on_left(),
        }
    }

    pub fn road_light(&self, lights: &mut TrafficLights) {
        on_light(self.dir, lights)
    }

    // movement helpers
    fn on_up(&mut self) {
        if self.x == 360 && self.y == 260 && self.color == ORANGE {
            self.dir = Direction::Left;
            self.turned = true;
        } else if self.x == 360 && self.y == 310 && self.color == YELLOW {
            self.dir = Direction::Right;
            self.turned = true;
        }
    }

    fn on_down(&mut self) {
        if self.y == 310 && self.x == 410 && self.color == ORANGE {
            self.dir = Direction::Right;
            self.turned = true;
        } else if self.y == 260 && self.x == 410 && self.color == YELLOW {
            self.dir = Direction::Left;
            self.turned = true;
        }
    }

    fn on_right(&mut self) {
        if self.x == 360 && self.y == 310 && self.color == ORANGE {
            self.dir = Direction::Up;
            self.turned = true;
        } else if self.x == 410 && self.y == 310 && self.color == YELLOW {
            self.dir = Direction::Down;
            self.turned = true;
        }
    }

    fn on_left(&mut self) {
        if self.x == 360 && self.y == 260 && self.color == YELLOW {
            self.dir = Direction::Up;
            self.turned = true;
        } else if self.x == 410 && self.y == 260 && self.color == ORANGE {
            self.dir = Direction::Down;
            self.turned = true;
        }
    }

    fn on_close(&self, vehicle: &Vehicle) -> bool {
        match self.dir {
            Direction::Up => vehicle.y > self.y && vehicle.y - self.y <= GAP,
            Direction::Down => vehicle.y < self.y && self.y - vehicle.y <= GAP,
            Direction::Left => vehicle.x < self.x && self.x - vehicle.x <= GAP,
            Direction::Right => vehicle.x > self.x && vehicle.x - self.x <= GAP,
        }
    }

    pub fn push(
        self,
        vehicles: &mut Vec<Vehicle>,
        check_y: Option<i32>,
        check_x: Option<i32>,
        dir: Direction
    ) {
        let can_push = if vehicles.is_empty() {
            true
        } else {
            match last_vehicle_direction(vehicles, dir) {
                Some(last) =>
                    match (check_y, check_x) {
                        (Some(y_limit), None) => {
                            if self.dir == Direction::Down {
                                last.y < y_limit
                            } else {
                                last.y > y_limit
                            }
                        }
                        (None, Some(x_limit)) => {
                            if self.dir == Direction::Right {
                                last.x > x_limit
                            } else {
                                last.x < x_limit
                            }
                        }
                        _ => false,
                    }
                None => true,
            }
        };

        if can_push && vehicles.len() < MAX {
            vehicles.push(self);
        }
    }
}
