use eframe::egui;
use std::path::Path;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 200.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Guilty Gear Strive Mod Manager",
        options,
        Box::new(|_cc| Ok(Box::new(SteamCheckerApp::default()))),
    )
}


struct SteamCheckerApp {
    show_popup: bool,
    steam_found: bool,
    steam_path: String,
    checked: bool,
    custom_path: String,
    valid_path: bool,
    items: Vec<Item>,
    selected_item: Option<usize>,
}

struct Item {
    id: usize,
    title: String,
    label: String,
}

impl Default for SteamCheckerApp {
    fn default() -> Self {
        let items = vec![
            Item {
                id: 0,
                title: "Enable Mods".to_string(),
                label: "Toggle mod loading for GGS".to_string(),
            },
            Item {
                id: 1,
                title: "Backup Save Data".to_string(),
                label: "Create a backup of your save file".to_string(),
            },
            Item {
                id: 2,
                title: "Restore Save Data".to_string(),
                label: "Restore a previous save backup".to_string(),
            },
            Item {
                id: 3,
                title: "Manage Mods".to_string(),
                label: "View and organize installed mods".to_string(),
            },
        ];
        let mut app = SteamCheckerApp {
            show_popup: false,
            steam_found: false,
            steam_path: String::new(),
            checked: false,
            custom_path: String::new(),
            valid_path: true,
            items,
            selected_item: None,
        };
        app.check_steam_installation(None);
        app
    }
}

impl SteamCheckerApp {
    fn check_steam_installation(&mut self, custom_path: Option<String>) {
        if self.checked {
            return;
        }
        if custom_path.is_some() {
            let path = custom_path.unwrap();
            if Path::new(&path).exists() && Path::new(&path).is_dir() {
                self.steam_found = true;
                self.steam_path = path;
                self.show_popup = true;
                self.checked = true;
                return;
            } else {
                self.steam_found = false;
                self.steam_path = String::from("Not found");
                self.show_popup = true;
                self.checked = true;
                return;
            }
        }
        let paths: Vec<String> = if cfg!(target_os = "windows") {
            vec![
            String::from(r"C:\\Program Files (x86)\\Steam\\steamapps\\common\\GUILTY GEAR STRIVE\\RED\\Content\\Paks\\"),
            String::from(r"C:\\Program Files\\Steam\\steamapps\\common\\GUILTY GEAR STRIVE\\RED\\Content\\Paks\\"),
            ]
        } else if cfg!(target_os = "macos") {
            vec![
            String::from("/Applications/Steam.app/steamapps/common/GUILTY GEAR STRIVE/RED/Content/Paks/"),
            format!("{}/Library/Application Support/Steam/steamapps/common/GUILTY GEAR STRIVE/RED/Content/Paks/",
                std::env::var("HOME").unwrap_or_default()),
            ]
        } else if cfg!(target_os = "linux") {
            vec![
            format!("{}/.local/share/Steam/steamapps/common/GUILTY GEAR STRIVE/RED/Content/Paks/",
                std::env::var("HOME").unwrap_or_default()),
            format!("{}/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/GUILTY GEAR STRIVE/RED/Content/Paks/",
                std::env::var("HOME").unwrap_or_default()),
            ]
        } else {
            vec![]
        };

        for path in paths {
            if Path::new(&path).exists() && Path::new(&path).is_dir() {
                self.steam_found = true;
                self.steam_path = path.to_string();
                self.show_popup = true;
                self.checked = true;
                return;
            }
        }

        self.steam_found = false;
        self.steam_path = String::from("Not found");
        self.show_popup = true;
        self.checked = true;
    }

    fn create_mods_directory(&self) {
        if self.steam_found {
            let mods_path = format!("{}/~mods/", self.steam_path);
            if !Path::new(&mods_path).exists() {
                if let Err(e) = std::fs::create_dir_all(&mods_path) {
                    eprintln!("Failed to create Mods directory: {}", e);
                }
            }
        }
    }
}

impl eframe::App for SteamCheckerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Main window
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("GUILTY GEAR STRIVE Mod Manager");
            ui.add_space(20.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                let available_width = ui.available_width();
                for item in &mut self.items {
                    ui.add_space(4.0);
                    ui.group(|ui| {
                    ui.set_width(available_width);
                    ui.horizontal_wrapped(|ui| {
                        ui.heading(&item.title);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add(egui::Button::new(egui::RichText::new("⚙").size(18.0))).clicked() {
                            self.selected_item = Some(item.id);
                        }
                        });
                    });
                    ui.label(&item.label);
                    }).response;
                    ui.add_space(8.0);
                }
            });

            egui::SidePanel::right("right_bar")
            .resizable(false)
            .default_width(120.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    if let Ok(image) = image::open("imgs/brisket.jpg") {
                        let image_buffer = image.to_rgba8();
                        let (width, height) = image_buffer.dimensions();
                        let pixels = image_buffer.into_vec();
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(
                            [width as usize, height as usize],
                            &pixels,
                        );
                        let texture = ui.ctx().load_texture(
                            "brisket_logo",
                            color_image,
                            egui::TextureOptions::default(),
                        );
                        ui.image(&texture);
                    } else {
                        ui.label("Image not found");
                    }
                    ui.add_space(10.0);
                    ui.label("Character Name:".to_string());
                });
            });
        });

        // Modal popup window
        if self.show_popup {
            egui::Window::new("Result")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    if self.steam_found {
                        ui.colored_label(egui::Color32::GREEN, "Steam and GUILTY GEAR STRIVE found");
                        ui.add_space(10.0);
                        ui.label(format!("Found at: {}", self.steam_path));
                        ui.add_space(10.0);
                        ui.colored_label(egui::Color32::WHITE, "Is this the correct installation?");
                        ui.horizontal(|ui| {
                            if ui.button("Yes").clicked() {
                                self.create_mods_directory();
                                self.show_popup = false;
                            }
                            if ui.button("No").clicked() {
                                self.steam_found = false;
                            }
                        });
                    } else {
                        ui.colored_label(egui::Color32::RED, "Steam not found");
                        ui.add_space(10.0);
                        ui.label("A Steam installation with GUILTY GEAR STRIVE was not found in the default directories.");
                        ui.add_space(10.0);
                        ui.label("Enter custom directory for GUILTY GEAR STRIVE (up to the game folder):");
                        ui.text_edit_singleline(&mut self.custom_path);
                        if !self.valid_path {
                            ui.colored_label(egui::Color32::RED, "No GUILTY GEAR STRIVE installation folder found at the specified path");
                        }
                        ui.horizontal(|ui| {
                            if ui.button("Check").clicked() {
                                // Compose the expected Paks path from the user input
                                let mut custom_paks_path = self.custom_path.trim().to_string();
                                if !custom_paks_path.ends_with("/RED/Content/Paks/") {
                                    if !custom_paks_path.ends_with('/') {
                                        custom_paks_path.push('/');
                                    }
                                    custom_paks_path.push_str("RED/Content/Paks/");
                                }
                                // Call check_steam_installation with the composed path
                                self.checked = false; // allow re-check
                                self.check_steam_installation(Some(custom_paks_path.clone()));
                                // Set valid_path for UI feedback
                                self.valid_path = self.steam_found && self.steam_path == custom_paks_path;
                            }
                            if ui.button("Exit").clicked() {
                                std::process::exit(0);
                            }
                        });
                    }
                });
        }
    }
}
