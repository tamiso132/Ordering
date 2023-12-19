#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::{robot, Queue};
use eframe::egui;

pub fn init() {}

pub(crate) fn run(
    robot_stream: Arc<Mutex<TcpStream>>,
    history_orders: Arc<Mutex<Vec<([u16; 4], u16)>>>,
    current_order: Arc<Mutex<Option<([u16; 4], u16)>>>,
    orders_to_process: Arc<Mutex<Queue<([u16; 4], u16)>>>,
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
                toggle_history: false,
                current_order,
                history_orders,
                toggle_current: false,
                toggle_inprocess: false,
                orders_to_process,
                robot_stream,
            })
        }),
    )
}

struct MyApp {
    toggle_history: bool,
    toggle_current: bool,
    toggle_inprocess: bool,

    current_order: Arc<Mutex<Option<([u16; 4], u16)>>>,
    history_orders: Arc<Mutex<Vec<([u16; 4], u16)>>>,
    orders_to_process: Arc<Mutex<Queue<([u16; 4], u16)>>>,
    robot_stream: Arc<Mutex<TcpStream>>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if ui.button("Start").clicked() {
                    robot::send_start(self.robot_stream.clone());
                }
                if ui.button("Stop").clicked() {
                    robot::send_stop(self.robot_stream.clone());
                }
                if ui.button("Current Order").clicked() {
                    self.toggle_current = !self.toggle_current;
                };
                if self.toggle_current {
                    if self.current_order.lock().unwrap().is_some() {
                        let y = self.current_order.lock().unwrap().clone().unwrap();
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
                if self.toggle_history {
                    egui::ScrollArea::vertical()
                        .max_height(100.0)
                        .show(ui, |ui| {
                            // Dynamically add labels
                            if self.history_orders.lock().unwrap().len() > 0 {
                                let history = self.history_orders.lock().unwrap();
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
                            if self.orders_to_process.lock().unwrap().list_of_queue.len() > 0 {
                                let in_process_orders =
                                    self.orders_to_process.lock().unwrap().list_of_queue.clone();
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

            if ui.button("Toggle ScrollArea").clicked() {
                self.toggle_history = !self.toggle_history;
            }
        });
    }
}
