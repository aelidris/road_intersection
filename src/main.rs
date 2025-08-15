use macroquad::prelude::*;

mod vehicles;
mod traffic_lights;
mod tools;

use vehicles::*;
use traffic_lights::*;
use tools::*;

fn conf() -> Conf {
    Conf {
        window_title: "road_intersection".to_string(),
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
    let mut lights = TrafficLights::new();

    loop {
        clear_background(BLACK);

        draw_roads();
        draw_traffic_lights(&lights);

        let (up, down, left, right) = vehicle_direction(&mut vehicles);
        let mut count = 0;
        [up, down, left, right].iter().for_each(|v| {
            if !v.is_empty() {
                count += 1;
            }
        });
        if count == 1 {
            let last_vehicle: Vehicle = vehicles[vehicles.len() - 1].clone();
            lights.on_light(last_vehicle.dir);
        } else {
            lights.prioritize_vehicle(&vehicles);
        }

        // Update vehicles
        let copy = &vehicles.clone();
        for vehicle in &mut vehicles {
            if !vehicle.at_traffic_light(&mut lights) && !vehicle.next(copy) {
                vehicle.update();
            }
            if !vehicle.turned {
                vehicle.on_change();
            }
        }

        draw_vehicles(&vehicles);

        // Remove vehicles that are off-screen
        vehicles.retain(
            |vehicle| vehicle.y <= 660 && vehicle.y >= -60 && vehicle.x <= 860 && vehicle.x >= -60
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
