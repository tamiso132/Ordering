#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{robot, Queue, Thing};
use eframe::egui;
use egui::Button;

pub fn init() {}

pub(crate) fn run(
    all: Arc<Mutex<Thing>>,
    stream: Arc<Mutex<TcpStream>>,
) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 450.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Multiple viewports",
        options,
        Box::new(|_cc| {
            Box::new(MyApp {
                stream,
                all,
                toggle_history: false,
                toggle_current: false,
                toggle_inprocess: false,
                red: String::new(),
                yellow: String::new(),
                green: String::new(),
                blue: String::new(),
            })
        }),
    )
}

struct MyApp {
    all: Arc<Mutex<Thing>>,
    toggle_history: bool,
    toggle_current: bool,
    toggle_inprocess: bool,
    stream: Arc<Mutex<TcpStream>>, // current_order: Arc<Mutex<Option<([u16; 4], u16)>>>,
    // history_orders: Arc<Mutex<Vec<([u16; 4], u16)>>>,
    // orders_to_process: Arc<Mutex<Queue<([u16; 4], u16)>>>,
    // robot_stream: Arc<Mutex<TcpStream>>,
    red: String,
    yellow: String,
    green: String,
    blue: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let thing = self.all.lock().unwrap();
            thread::sleep(Duration::from_millis(500));
            ui.horizontal(|ui| {
                let name_label = ui.label("Red");
                ui.text_edit_singleline(&mut self.red)
                    .labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                let name_label = ui.label("Yellow");
                ui.text_edit_singleline(&mut self.yellow)
                    .labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                let name_label = ui.label("Green");
                ui.text_edit_singleline(&mut self.green)
                    .labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                let name_label = ui.label("Blue");
                ui.text_edit_singleline(&mut self.blue)
                    .labelled_by(name_label.id);
            });
            thread::sleep(Duration::from_millis(500));
            ui.horizontal(|ui| {
                if ui.button("Submit").clicked() {
                    if !self.red.is_empty()
                        || self.red.parse::<u16>().is_ok()
                        || !self.yellow.is_empty()
                        || self.red.parse::<u16>().is_ok()
                        || !self.blue.is_empty()
                        || self.blue.parse::<u16>().is_ok()
                        || !self.green.is_empty()
                        || self.green.parse::<u16>().is_ok()
                    {
                        let tuple = thing.grid.get_free();
                        let red = self.red.parse::<u16>().unwrap();
                        let yellow = self.yellow.parse::<u16>().unwrap();
                        let green = self.green.parse::<u16>().unwrap();
                        let blue = self.blue.parse::<u16>().unwrap();

                        if tuple.0 >= self.red.parse::<u16>().unwrap() {
                            if tuple.1 >= self.yellow.parse::<u16>().unwrap() {
                                if tuple.2 >= self.green.parse::<u16>().unwrap() {
                                    if tuple.3 >= self.blue.parse::<u16>().unwrap() {
                                        let v = thing
                                            .grid
                                            .get_positions_for_order([red, yellow, green, blue]);
                                        robot::send_order(10, v, self.stream.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            });
            thread::sleep(Duration::from_millis(500));
            ui.vertical_centered(|ui| {
                ui.button("Start").clicked();
                if ui.button("Start").clicked() {
                    robot::send_start(self.stream.clone());
                }
                if ui.button("Stop").clicked() {
                    robot::send_stop(self.stream.clone());
                }
                if ui.button("Current Order").clicked() {
                    self.toggle_current = !self.toggle_current;
                };
                if self.toggle_current {
                    if thing.current_order.is_some() {
                        let y = thing.current_order.unwrap();
                        ui.label(format!("Order-id: {}", y.1));
                        ui.label(format!("Red: {}", y.0[0]));
                        ui.label(format!("Yellow: {}", y.0[1]));
                        ui.label(format!("Green: {}", y.0[2]));
                        ui.label(format!("Blue: {}", y.0[3]));
                    }
                }
                if ui.button("Order History").clicked() {
                    self.toggle_history = !self.toggle_history;
                }
                thread::sleep(Duration::from_millis(500));
                if self.toggle_history {
                    egui::ScrollArea::vertical()
                        .max_height(100.0)
                        .show(ui, |ui| {
                            // Dynamically add labels
                            if thing.history_orders.len() > 0 {
                                let history = thing.history_orders.clone();
                                for i in 0..history.len() {
                                    ui.label(format!("Order-id: {}", history[i].1));
                                    ui.label(format!("Red: {}", history[i].0[0]));
                                    ui.label(format!("Yellow: {}", history[i].0[1]));
                                    ui.label(format!("Green: {}", history[i].0[2]));
                                    ui.label(format!("Blue: {}\n", history[i].0[3]));
                                }
                            }
                        });
                }

                if ui.button("Order incoming").clicked() {
                    self.toggle_inprocess = !self.toggle_inprocess;
                }
                if self.toggle_inprocess {
                    egui::ScrollArea::vertical()
                        .max_height(100.0)
                        .show(ui, |ui| {
                            // TODO uncomment
                            if thing.orders_to_process.list_of_queue.len() > 0 {
                                let in_process_orders =
                                    thing.orders_to_process.list_of_queue.clone();
                                for order in in_process_orders {
                                    ui.label(format!("Order-id: {}", order.1));
                                    ui.label(format!("Red: {}", order.0[0]));
                                    ui.label(format!("Yellow: {}", order.0[1]));
                                    ui.label(format!("Green: {}", order.0[2]));
                                    ui.label(format!("Blue: {}\n", order.0[3]));
                                }
                            }
                        });
                }
            });
            thread::sleep(Duration::from_millis(500));

            if ui.button("Toggle ScrollArea").clicked() {
                self.toggle_history = !self.toggle_history;
            }
        });
    }
}
