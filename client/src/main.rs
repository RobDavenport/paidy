use eframe::egui;
use shared::{Menu, SERVICE_URL};

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
    pending_table: Option<i64>,
    pending_order: Vec<i64>,
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
            pending_table: None,
            pending_order: Vec::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // TODO: Split this into panels

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
                                self.pending_table = None
                            };

                            if ui.button("Submit Order").clicked() {
                                println!("TODO: Submit order");
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
                        ui.label("TODO: ");
                        // TODO: Add Table Selector
                        // TODO: Show Table contents
                        // TODO: Add buttons to remove items from the table
                    })
                })
            });
        });
    }
}
// TODO:
// Client: add one or more items with a table number,
// The application MUST, upon creation request, store the item, the table number, and how long the item will take to cook.

// TODO:
// Client: remove an item for a table,
// The application MUST, upon deletion request, remove a specified item for a specified table number.

// TODO:
// Client: query the items still remaining for a table.
// The application MUST, upon query request, show all items for a specified table number.

// TODO:
// Client: query a specific item remaining for a table
// The application MUST, upon query request, show a specified item for a specified table number.
