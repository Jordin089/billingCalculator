use eframe::egui;
use std::collections::HashMap;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Billing Calculator", native_options, Box::new(|cc| Ok(Box::new(MainApplication::new(cc)))));
}

#[derive(Default)]
struct MainApplication {
    bill: Bill,
    prices:  HashMap<String, f32>,
    item_to_add: String,
    item_price_to_add: String,
    current_displayed_price: f32,
}

impl MainApplication {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            bill: Bill {total_value: 0.0},
            prices: HashMap::new(),
            item_to_add: "".to_owned(),
            item_price_to_add: "".to_owned(),
            current_displayed_price: 0.0,
        };
        Self::default()
    }
}

impl eframe::App for MainApplication {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("pricesGrid").show(ui, |ui|{
                    for (name, _price) in &self.prices {
                        if ui.add_sized([200.,80.], egui::Button::new(format!("{}", name.clone()))).clicked() {
                            self.bill.add_price(*self.prices.get(name).expect("fuck..."));
                        }
                    }
                });
            });
        });
        egui::TopBottomPanel::bottom("bottom_navbar").show(ctx, |ui| {
            let available_size = ui.available_size();
            ui.horizontal(|ui| {

                ui.text_edit_singleline(&mut self.item_to_add).on_hover_text("item name");
                ui.text_edit_singleline(&mut self.item_price_to_add).on_hover_text("item price");
                if ui.add_sized([120.,80.], egui::Button::new(format!("Add item"))).clicked() {
                    add_new_item(self.item_to_add.clone(), self.item_price_to_add.clone(), &mut self.prices);
                }
                ui.add_space(20.);
                if ui.add_sized([120.,80.], egui::Button::new(format!("get Price"))).clicked() {
                    self.current_displayed_price = self.bill.get_value();
                }
                if self.current_displayed_price != 0. {
                    ui.label(format!("Price: {}", self.current_displayed_price));
                }

            });
        });
    }
}

// below here is anything to do with functionality

struct Bill
{
    total_value: f32,
}
impl Bill {
    fn add_price(&mut self, amount: f32)
    {
        println!("before: {}", self.total_value);
        self.total_value += amount;
        println!("after : {}", self.total_value);
    }
    fn get_value(&mut self) -> f32 {
        self.total_value.clone()
    }
    fn print_value(&mut self)
    {
        println!("Total Costs: {}", self.total_value);
    }
}

impl Default for Bill {
    fn default() -> Bill {
       Bill{total_value: 0.0}
    }
}

fn add_new_item(item_name: String, item_cost: String, prices: &mut HashMap<String, f32>)
{
    let item_price: f32 = item_cost.trim().parse().expect("please insert as a float");
    prices.insert(item_name, item_price);
}
