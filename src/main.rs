mod camera;
mod common_math;
mod state;
mod aircraft;
mod displays;
mod wing;

use std::{fs::OpenOptions, io::Write};
use chrono::Utc;

use displays::{Dial, Gauge};
use eframe::egui;

fn main() {
    let _ = std::fs::remove_file("log.json");
    let mut file = OpenOptions::new().append(true).create(true).open("log.json").unwrap();
    let _ = file.write("{\"data\": [".as_bytes());
	let options = eframe::NativeOptions::default();
	eframe::run_native(
		"My egui App",
		options,
		Box::new(|_cc| Ok(Box::new(App::with_file(file)))),
    ).unwrap();
    println!("done");
}

struct App{
    camera: camera::Camera,
    aircraft: aircraft::Aircraft,
    logger: std::fs::File,
    velocity_dial: displays::Dial,
    altitude_dial: displays::Dial,
    climb_rate_dial: displays::Dial,
    throttle_gauge: displays::Gauge,
}

impl Default for App {
    fn default() -> Self {
        Self{
            camera: camera::Camera::new(),
            aircraft: aircraft::Aircraft::new(),
            logger: std::fs::File::open("thing").unwrap(),
            velocity_dial: displays::Dial::test(),
            altitude_dial: displays::Dial::test(),
            climb_rate_dial: displays::Dial::test(),
            throttle_gauge: displays::Gauge::test(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let time_start = Utc::now();
        let dt = 1.0/50.0; // time to render in seconds
        self.run_physics(dt);
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = ctx.input(|state|{
                for key_code in state.keys_down.clone() {
                    match key_code {
                        egui::Key::W => self.aircraft.pitch_by(-20.0 * dt),
                        egui::Key::S => self.aircraft.pitch_by(20.0 * dt),
                        egui::Key::K => self.aircraft.pitch_by(1.0 * dt),
                        egui::Key::I => self.aircraft.pitch_by(-1.0 * dt),
                        egui::Key::Q => self.aircraft.state.pointing_global.roll += 1.0,
                        egui::Key::E => self.aircraft.state.pointing_global.roll += -1.0,
                        egui::Key::A => self.aircraft.yaw_by(10.0 * dt),
                        egui::Key::D => self.aircraft.yaw_by(-10.0 * dt),
                        egui::Key::Z => self.aircraft.throttle_by(2.5 * dt),
                        egui::Key::X => self.aircraft.throttle_by(-2.5 * dt),
                        _ => (),
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.vertical( |ui| {
                    ui.add(egui::Image::from_texture(&ctx.load_texture("siulator",self.camera.render(self.aircraft.state.velocity.angle_with_horizon(), 0.0), Default::default())));
                    ui.horizontal(|ui| {
                        self.velocity_dial.draw(ui, self.aircraft.state.velocity.magnitude());
                        self.altitude_dial.draw(ui, self.aircraft.state.position.z);
                        self.climb_rate_dial.draw(ui, self.aircraft.state.velocity.z);
                        self.throttle_gauge.draw(ui, self.aircraft.throttle_percent);
                    });
                });
                ui.label(format!("roll: {}", self.aircraft.state.pointing_global.roll));
                /*ui.vertical(|ui|{
                    ui.label("velocity:    ");
                    ui.label("altitude:    ");
                    ui.label("throttle:    ");
                    ui.label("pitch:       ");
                    ui.label("roll:        ");
                    ui.label("alpha:       ");
                    ui.label("climb rate:  ");
                });
                ui.vertical(|ui| {
                    ui.label(format!("{:.2}", self.aircraft.state.velocity.magnitude()));
                    ui.label(format!("{:.2}", self.aircraft.state.position.z));
                    ui.label(format!("{:.1}", self.aircraft.throttle_percent));
                    ui.label(format!("{:.2}", self.aircraft.state.pointing_global.altitude));
                    ui.label(format!("{:.2}", self.aircraft.state.pointing_global.roll));
                    ui.label(format!("{:.1}", self.aircraft.state.pointing_global.as_vec3().angle_with(&self.aircraft.state.velocity) * 57.3));
                    ui.label(format!("{:.3}", self.aircraft.state.velocity.z));
                }); */
            });
        });
        self.logger.write(self.aircraft.state.log().as_bytes()).unwrap();
        self.logger.write(b",\n").unwrap();
        ctx.request_repaint();
        let time_end = Utc::now();
        let dt_actual = time_end - time_start;
        //println!("{:?}", (time_end - time_start).num_milliseconds());
        if let Some(time) = ((dt * 1e9) as u64).checked_sub(dt_actual.num_nanoseconds().unwrap() as u64) {
            std::thread::sleep(std::time::Duration::from_nanos(time));
        } else {
            println!("frame took too long to render!");
        }
    }
}

impl App {
    fn run_physics(&mut self, dt: f64) {
        self.aircraft.do_step(dt);
        self.camera.euler = self.aircraft.state.pointing_global;
        self.camera.position = self.aircraft.state.position;
    }
    
    fn with_file(file: std::fs::File) -> App {
        let velocity_dial = Dial::new("vel".to_string(), "m/s".to_string(), 80.0, 0.0);
        let altitude_dial = Dial::new("alt".to_string(), "m".to_string(), 400.0, 0.0);
        let climb_rate_dial = Dial::new("v_z".to_string(), "m/s".to_string(), -10.0, 10.0);
        let throttle_gauge = Gauge::new("throttle".to_string(), "%".to_string() , 1.0, 0.0);
        App{
            camera: camera::Camera::new(),
            aircraft: aircraft::Aircraft::flying_high(),
            logger: file,
            velocity_dial,
            altitude_dial,
            climb_rate_dial,
            throttle_gauge,
        }
    }
}