use std::thread::JoinHandle;

use eframe::egui;
use shared::{Menu, OrderItemsRequest, TableResponse, SERVICE_URL};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Paidy Client",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
    .unwrap();
}

struct App {
    menu: Vec<MenuListItem>,
    pending_order: Vec<i64>,
    table_selector: String,
    table_response: TableResponse,
    debug_order_id: String,
}

struct MenuListItem {
    id: i64,
    name: String,
    prep_time: String,
}

impl App {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        let menu: Menu = reqwest::blocking::get(format!("http://{SERVICE_URL}/menu"))
            .unwrap()
            .json()
            .unwrap();

        let menu = menu
            .items
            .iter()
            .map(|item| {
                let prep_time = format!("~{}mins", (item.prep_min_m + item.prep_max_m) / 2.0);

                MenuListItem {
                    id: item.id,
                    name: item.name.clone(),
                    prep_time,
                }
            })
            .collect();

        Self {
            menu,
            pending_order: Vec::new(),
            table_selector: String::default(),
            table_response: TableResponse {
                table_id: 0,
                ordered_items: Vec::new(),
            },
            debug_order_id: String::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let mut new_response = None;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Menu Left Panel
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Menu");
                        self.menu.iter().for_each(|item| {
                            ui.horizontal(|ui| {
                                if ui.button("+").clicked() {
                                    self.pending_order.push(item.id);
                                }
                                ui.label(format!("id: {}:", item.id));
                                ui.label(&item.name);
                                ui.label(&item.prep_time);
                            });
                        });
                    });
                });

                // Order Center Panel
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Order Status");

                        ui.horizontal(|ui| {
                            if ui.button("Clear Order").clicked() {
                                self.pending_order.clear();
                            };

                            if ui.button("Submit Order").clicked() {
                                if let Ok(table_id) = self.table_selector.parse() {
                                    new_response = order_items(table_id, &self.pending_order);
                                    self.pending_order.clear();
                                } else {
                                    println!("Failed to parse i64 from table_selector");
                                }
                            }
                        });

                        ui.heading("Pending Order");
                        self.pending_order.iter().enumerate().for_each(|(i, id)| {
                            ui.label(format!(
                                "{i}: {}",
                                self.menu.iter().find(|item| item.id == *id).unwrap().name
                            ));
                        });
                    });
                });

                // Table Status Panel
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Table Status");
                        ui.text_edit_singleline(&mut self.table_selector);
                        if ui.button("Fetch Table Items").clicked() {
                            if let Ok(table_id) = self.table_selector.parse() {
                                new_response = fetch_table_items(table_id);
                            } else {
                                println!("Failed to parse i64 from table_selector");
                            }
                        }

                        self.table_response.ordered_items.iter().for_each(|item| {
                            ui.horizontal(|ui| {
                                if ui.button("-").clicked() {
                                    if let Ok(table_id) = self.table_selector.parse() {
                                        new_response = remove_item(table_id, item.order_id);
                                    } else {
                                        println!("Failed to parse i64 from table_selector");
                                    }
                                }
                                ui.label(format!(
                                    "oid: {}, {}, rdy @ {}",
                                    item.order_id,
                                    self.menu.get(item.item_id as usize).unwrap().name, // This is not ideal...
                                    item.ready_at,
                                ));
                            });
                        });
                    });
                });

                // "Debug" Menu
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Debug Menu");
                        if ui.button("Random 10 Table Orders").clicked() {
                            order_random_multiple(10);
                        }

                        if ui.button("Random 100 Table Orders").clicked() {
                            order_random_multiple(100);
                        }

                        if ui.button("Random 1000 Table Orders").clicked() {
                            order_random_multiple(1000);
                        }

                        ui.label("oid:");
                        ui.text_edit_singleline(&mut self.debug_order_id);

                        if ui.button("Get Table Item").clicked() {
                            if let (Ok(table_id), Ok(order_id)) =
                                (self.table_selector.parse(), self.debug_order_id.parse())
                            {
                                println!("{:?}", get_table_item(table_id, order_id));
                            } else {
                                println!(
                                    "Failed to parse i64 from table_selector or debug_order_id."
                                );
                            }
                        }
                    })
                })
            });
        });

        if let Some(new_response) = new_response {
            self.table_response = new_response
        }
    }
}

fn order_random_multiple(count: usize) {
    println!("Ordering random items to {count} tables...");
    let threads: Vec<JoinHandle<()>> = (0..count)
        .map(|table_id| {
            std::thread::spawn(move || {
                let item_count = fastrand::usize(5..15);
                let items: Vec<i64> = (0..item_count).map(|_| fastrand::i64(0..15)).collect(); // Hard coded to 15 items
                let _ = order_items(table_id as i64, &items);
            })
        })
        .collect();

    for thread in threads {
        thread.join().unwrap()
    }
    println!("Done!")
}

fn order_items(table_id: i64, items: &[i64]) -> Option<TableResponse> {
    let client = reqwest::blocking::Client::new();
    match client
        .post(format!("http://{SERVICE_URL}/tables/{table_id}"))
        .json(&OrderItemsRequest {
            items: items.iter().cloned().collect(),
        })
        .send()
        .unwrap()
        .json()
    {
        Ok(response) => Some(response),
        Err(e) => {
            println!("{e}");
            None
        }
    }
}

fn fetch_table_items(table_id: i64) -> Option<TableResponse> {
    match reqwest::blocking::get(format!("http://{SERVICE_URL}/tables/{table_id}"))
        .unwrap()
        .json()
    {
        Ok(response) => Some(response),
        Err(e) => {
            println!("{e}");
            None
        }
    }
}

fn remove_item(table_id: i64, order_id: i64) -> Option<TableResponse> {
    let client = reqwest::blocking::Client::new();
    match client
        .delete(format!("http://{SERVICE_URL}/tables/{table_id}/{order_id}"))
        .send()
        .unwrap()
        .json()
    {
        Ok(response) => Some(response),
        Err(e) => {
            println!("{e}");
            None
        }
    }
}

fn get_table_item(table_id: i64, order_id: i64) -> Option<String> {
    let client = reqwest::blocking::Client::new();
    match client
        .get(format!("http://{SERVICE_URL}/tables/{table_id}/{order_id}"))
        .send()
        .unwrap()
        .text()
    {
        Ok(response) => Some(response),
        Err(e) => {
            println!("{e}");
            None
        }
    }
}
