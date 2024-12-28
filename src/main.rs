mod camera;
mod common_math;
mod state;
mod aircraft;
mod displays;

use std::{fs::OpenOptions, io::Write};

use displays::Dial;
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
    test_dial: displays::Dial,
}

impl Default for App {
    fn default() -> Self {
        Self{
            camera: camera::Camera::new(),
            aircraft: aircraft::Aircraft::new(),
            logger: std::fs::File::open("thing").unwrap(),
            test_dial: displays::Dial::test(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let dt = 0.05;

        self.run_physics(dt);
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = ctx.input(|state|{
                for key_code in state.keys_down.clone() {
                    match key_code {
                        egui::Key::W => self.aircraft.state.pointing.altitude += -1.0,
                        egui::Key::S => self.aircraft.state.pointing.altitude += 1.0,
                        egui::Key::E => self.aircraft.state.pointing.roll += 1.0,
                        egui::Key::Q => self.aircraft.state.pointing.roll += -1.0,
                        egui::Key::Z => self.aircraft.increase_throttle(),
                        egui::Key::X => self.aircraft.decrease_throttle(),
                        _ => (),
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.add(egui::Image::from_texture(&ctx.load_texture("siulator",self.camera.render(), Default::default())));
                ui.vertical(|ui|{
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
                    ui.label(format!("{:.2}", self.aircraft.state.pointing.altitude));
                    ui.label(format!("{:.2}", self.aircraft.state.pointing.roll));
                    ui.label(format!("{:.1}", self.aircraft.state.pointing.as_vec3().angle_with(&self.aircraft.state.velocity) * 57.3));
                    ui.label(format!("{:.3}", self.aircraft.state.velocity.z));
                });
                ui.vertical(|ui| {
                    self.test_dial.draw(ui, self.aircraft.state.velocity.magnitude());
                });
            });
        });
        self.logger.write(self.aircraft.state.log().as_bytes()).unwrap();
        self.logger.write(b",\n");
        std::thread::sleep(std::time::Duration::from_millis((dt*1000.0) as u64));
        ctx.request_repaint();
    }
}

impl App {
    fn run_physics(&mut self, dt: f64) {
        self.aircraft.do_step(dt);
        self.camera.euler = self.aircraft.state.pointing;
        self.camera.position = self.aircraft.state.position;
    }
    
    fn with_file(file: std::fs::File) -> App {
        let test_dial = Dial::new("vel".to_string(), String::new(), 50.0, 0.0);
        App{
            camera: camera::Camera::new(),
            aircraft: aircraft::Aircraft::new(),
            logger: file,
            test_dial,
        }
    }
}