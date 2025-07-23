use eframe::egui;
use std::collections::HashMap;
enum Menu {
    Calculator,
    NewItems,
}
impl Default for Menu {
    fn default() -> Menu {
        return Menu::Calculator;
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _result = eframe::run_native("Billing Calculator", native_options, Box::new(|cc| Ok(Box::new(MainApplication::new(cc)))));
}

#[derive(Default)]
struct MainApplication {
    menu: Menu,
    bill: Bill,
    prices:  HashMap<String, f32>,
    item_to_add: String,
    item_price_to_add: String,
    current_displayed_price: f32,
}

impl MainApplication {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            menu: Menu::Calculator,
            bill: Bill {total_value: 0.0},
            prices: HashMap::new(),
            item_to_add: "".to_owned(),
            item_price_to_add: "".to_owned(),
            current_displayed_price: 0.0,
        };
        let mut thing = Self::default();
        initialise_prices(&mut thing.prices);
        return thing;

    }
    fn render_calculator(&mut self, ctx: &egui::Context) {
        let button_width = (ctx.screen_rect().max.x-40.)/5.;
        egui::TopBottomPanel::bottom("bottom_navbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(20.);
                if ui.add_sized([button_width/2.,button_width/6.], egui::Button::new(format!("get Price"))).clicked() {
                    self.current_displayed_price = self.bill.get_value();
                }
                if ui.add_sized([button_width/2.,button_width/6.], egui::Button::new(format!("go to add items"))).clicked() {
                    self.menu = Menu::NewItems;
                }
                if self.current_displayed_price != 0. {
                    ui.heading(format!("Price: {}", self.current_displayed_price));
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("pricesGrid").show(ui, |ui|{
                    let mut count = 0.;
                    for (name, price) in &self.prices {
                        if count >=  5. {
                            ui.end_row();
                            count = 0.;
                        }
                        if ui.add_sized([button_width, button_width/3.], egui::Button::new(format!("{}\n€{}", name.clone(), price.clone()))).clicked() {
                            self.bill.add_price(*self.prices.get(name).expect("fuck..."));
                        }
                        count += 1.;
                    }
                });
            });
        });
    }
    fn render_new_items(&mut self, ctx: &egui::Context) {
        let button_width = (ctx.screen_rect().max.x-40.)/5.;
        egui::CentralPanel::default().show(ctx, |ui| {
           egui::ScrollArea::vertical().show(ui, |ui| {
               ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                   ui.vertical(|ui| {
                       for (name, price) in &mut self.prices.clone() {
                           ui.horizontal(|ui|{
                               ui.label(format!("{} €{}", name, price));
                               ui.add_space(ui.available_width()-80.);
                               if ui.button("delete").clicked() {
                                   self.prices.remove(name);
                               };
                           });
                       }
                   });
               });
           });
        });
        egui::TopBottomPanel::bottom("bottom_navbar_items").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let _item_name_text = ui.text_edit_singleline(&mut self.item_to_add).on_hover_text("item name");
                let price_text = ui.text_edit_singleline(&mut self.item_price_to_add).on_hover_text("item price");
                if price_text.lost_focus() && price_text.ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                    add_new_item(self.item_to_add.clone(), self.item_price_to_add.clone(), &mut self.prices);
                    self.item_to_add = "".to_owned();
                    self.item_price_to_add = "".to_owned();
                }
                ui.add_space(20.);
                if ui.add_sized([button_width/2.,button_width/6.], egui::Button::new(format!("Add item"))).clicked() {
                    add_new_item(self.item_to_add.clone(), self.item_price_to_add.clone(), &mut self.prices);
                }
                if ui.add_sized([button_width/2.,button_width/6.], egui::Button::new(format!("go to calculator"))).clicked() {
                    self.menu = Menu::Calculator;
                }
            });
        });

    }
}

impl eframe::App for MainApplication {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.menu {
            Menu::Calculator => self.render_calculator(ctx),
            Menu::NewItems => self.render_new_items(ctx)
        }
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
        self.total_value += amount;
    }
    fn get_value(&mut self) -> f32 {
        let value = self.total_value.clone();
        self.total_value = 0.0;
        return value;
    }
}

impl Default for Bill {
    fn default() -> Bill {
       return Bill{total_value: 0.0};
    }
}

fn add_new_item(item_name: String, item_cost: String, prices: &mut HashMap<String, f32>)
{
    let item_price: f32 = item_cost.trim().parse().expect("please insert as a float");
    prices.insert(item_name, item_price);
}
fn initialise_prices(prices: &mut HashMap<String, f32>) {
    add_new_item("Beer".to_string(), 1.50.to_string(), prices);
    add_new_item("Soda".to_string(), 1.50.to_string(), prices);
    add_new_item("Wine".to_string(), 1.50.to_string(), prices);
    add_new_item("Weihen".to_string(), 4.to_string(), prices);
    add_new_item("Kasteel Rouge".to_string(), 3.to_string(), prices);
    add_new_item("Guinness".to_string(), 4.to_string(), prices);
    add_new_item("Shot Binnenlands".to_string(), 2.to_string(), prices);
    add_new_item("Shot Buitenlands".to_string(), 3.to_string(), prices);
    add_new_item("Mix Binnenlands".to_string(), 2.5.to_string(), prices);
    add_new_item("Mix Buitenlands".to_string(), 3.5.to_string(), prices);
}
