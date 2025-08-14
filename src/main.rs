use macroquad::prelude::*;
use std::time::Instant;

mod vehicles;
mod traffic_lights;
mod tools;

use vehicles::*;
use traffic_lights::*;
use tools::*;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

fn conf() -> Conf {
    Conf {
        window_title: "test".to_string(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut vehicles: Vec<Vehicle> = Vec::new();

    let mut lights = TrafficLights {
        top_light: false,
        down_light: false,
        left_light: false,
        right_light: false,
        timestamp: Instant::now(),
        direction: Direction::Right,
        state: false,
        duration: None,
    };

    loop {
        clear_background(BLACK);

        draw_roads();

        let (up, down, left, right) = vehicle_direction(&mut vehicles);
        let mut count = 0;
        [up, down, left, right].iter().for_each(|v| {
            if !v.is_empty() {
                count += 1;
            }
        });

        if count == 1 {
            vehicles[vehicles.len() - 1].road_light(&mut lights);
        } else {
            prioritize_vehicle(&mut lights, &vehicles);
        }

        draw_traffic_lights(&lights);

        // Update vehicles
        let copy_vehicles = vehicles.clone();
        for vehicle in &mut vehicles {
            traffic_lights(vehicle, &mut lights);
            if vehicle.moving && !vehicle.next(&copy_vehicles) {
                vehicle.update();
            }
            if !vehicle.turned {
                vehicle.on_change();
            }
        }

        draw_vehicles(&vehicles);

        // Remove vehicles that are off-screen
        vehicles.retain(
            |vehicle| vehicle.y <= 630 && vehicle.y >= -30 && vehicle.x <= 830 && vehicle.x >= -30
        );

        // Handle input
        if is_key_pressed(KeyCode::Up) {
            key_up(&mut vehicles);
        }
        if is_key_pressed(KeyCode::Down) {
            key_down(&mut vehicles);
        }
        if is_key_pressed(KeyCode::Left) {
            key_left(&mut vehicles);
        }
        if is_key_pressed(KeyCode::Right) {
            key_right(&mut vehicles);
        }
        if is_key_pressed(KeyCode::R) {
            key_r(&mut vehicles);
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
