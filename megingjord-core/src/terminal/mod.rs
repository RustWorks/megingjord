pub mod config;
pub mod geolocation;
pub mod local_osm_tiles;
pub mod mappainter;

use core::cell::Cell;
use egui::Align2;
use egui::Area;
use egui::CentralPanel;
use egui::ComboBox;
use egui::Context;
use egui::Frame;
use egui::Image;
use egui::RichText;
use egui::Ui;
use egui::Window;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use walkers::sources::Attribution;
use walkers::HttpOptions;
use walkers::Map;
use walkers::MapMemory;
use walkers::Position;
use walkers::Tiles;
use walkers::TilesManager;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[derive(Debug, Clone, Copy)]
pub struct GeoLocation {
    position: Position,
    accuracy: f32,
}

fn http_options() -> HttpOptions {
    HttpOptions {
        cache: if cfg!(target_arch = "wasm32") || std::env::var("NO_HTTP_CACHE").is_ok() {
            None
        } else {
            Some(".cache".into())
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Source {
    OpenStreetMap,
    LocalOSMTiles,
}

fn sources(egui_ctx: Context) -> (HashMap<Source, Box<dyn TilesManager + Send>>, Source) {
    let mut sources: HashMap<Source, Box<dyn TilesManager + Send>> = HashMap::default();
    let mut default_selected = Source::OpenStreetMap;

    sources.insert(
        Source::OpenStreetMap,
        Box::new(Tiles::with_options(
            walkers::sources::OpenStreetMap,
            http_options(),
            egui_ctx.to_owned(),
        )),
    );

    if let Some(localosm) = local_osm_tiles::LocalOSMTiles::new(egui_ctx.to_owned()) {
        sources.insert(Source::LocalOSMTiles, Box::new(localosm));
        default_selected = Source::LocalOSMTiles;
    }

    (sources, default_selected)
}

pub struct MyApp {
    sources: HashMap<Source, Box<dyn TilesManager + Send>>,
    selected_source: Source,
    map_memory: MapMemory,
    config_ctx: config::ConfigContext,
    plugin_painter: mappainter::MapPainterPlugin,
    geo: Arc<Mutex<Cell<Option<GeoLocation>>>>,
}

impl MyApp {
    pub fn new(egui_ctx: Context) -> Self {
        egui_extras::install_image_loaders(&egui_ctx);
        let (sources, default_source) = sources(egui_ctx.to_owned());

        let mut instance = Self {
            sources,
            selected_source: default_source,
            map_memory: MapMemory::default(),
            config_ctx: config::ConfigContext::new("terminal.ini".to_string()),
            plugin_painter: mappainter::MapPainterPlugin::new(),
            geo: Arc::new(Mutex::new(Cell::new(None))),
        };

        instance.config_load();

        #[cfg(target_arch = "wasm32")]
        instance.watch_geolocation();

        instance
    }

    #[cfg(target_arch = "wasm32")]
    fn watch_geolocation(&mut self) {
        let geolocation = web_sys::window().unwrap().navigator().geolocation().unwrap();
        let geo_store_mutexed = Arc::clone(&self.geo);

        let geo_callback = wasm_bindgen::prelude::Closure::<dyn FnMut(_)>::new(move |e: web_sys::Position| {
            let coords = e.coords();

            let geo = GeoLocation {
                position: Position::from_lat_lon(coords.latitude(), coords.longitude()),
                accuracy: coords.accuracy() as f32,
            };
            geo_store_mutexed.lock().unwrap().set(Some(geo));
        });

        let _ = geolocation.watch_position(geo_callback.as_ref().unchecked_ref());
        geo_callback.forget();
    }

    fn probe_geolocation(&self) -> Option<GeoLocation> {
        self.geo.lock().unwrap().get()
    }

    fn config_load(&mut self) {
        let config = self.config_ctx.config_load();

        if let Some(zoom_value) = config.zoom {
            while self.map_memory.zoom_get() < zoom_value {
                if self.map_memory.zoom_in().is_err() {
                    break;
                }
            }

            while self.map_memory.zoom_get() > zoom_value {
                if self.map_memory.zoom_out().is_err() {
                    break;
                }
            }
        }

        if let Some(lat_lon) = config.lat_lon {
            self.map_memory.center_at(lat_lon);
        }
    }
}

pub fn acknowledge(ui: &Ui, attribution: Attribution) {
    Area::new("Acknowledge")
        .anchor(Align2::CENTER_BOTTOM, [0., -10.])
        .show(ui.ctx(), |ui| {
            ui.horizontal(|ui| {
                if let Some(logo) = attribution.logo_light {
                    ui.add(Image::new(logo).max_height(30.0).max_width(80.0));
                }
                ui.hyperlink_to(attribution.text, attribution.url);
            });
        });
}

/// Simple GUI to zoom in and out.
pub fn zoom(ui: &Ui, map_memory: &mut MapMemory) {
    Window::new("Map")
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .anchor(Align2::LEFT_BOTTOM, [10., -10.])
        .show(ui.ctx(), |ui| {
            ui.horizontal(|ui| {
                if ui.button(RichText::new("➕").heading()).clicked() {
                    let _ = map_memory.zoom_in();
                }

                ui.label(format!("{:^3}", map_memory.zoom_get()));

                if ui.button(RichText::new("➖").heading()).clicked() {
                    let _ = map_memory.zoom_out();
                }
            });
        });
}

pub fn controls(ui: &Ui, selected_source: &mut Source, possible_sources: &mut dyn Iterator<Item = &Source>) {
    Window::new("Satellite")
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .anchor(Align2::RIGHT_TOP, [-10., 10.])
        .show(ui.ctx(), |ui| {
            ui.collapsing("Map", |ui| {
                ComboBox::from_label("")
                    .selected_text(format!("{:?}", selected_source))
                    .show_ui(ui, |ui| {
                        ui.set_min_width(100.);
                        for p in possible_sources {
                            ui.selectable_value(selected_source, *p, format!("{:?}", p));
                        }
                    });
            });
        });
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let rimless = Frame {
            fill: ctx.style().visuals.panel_fill,
            ..Default::default()
        };

        let geolocation = self.probe_geolocation();
        let myposition = if let Some(geolocation) = geolocation {
            geolocation.position
        } else {
            Position::from_lat_lon(0.0, 0.0)
        };

        CentralPanel::default().frame(rimless).show(ctx, |ui| {
            let tiles = self.sources.get_mut(&self.selected_source).unwrap().as_mut();
            let attribution = tiles.attribution();

            /* ctx.set_pixels_per_point(1.2); */

            // In egui, widgets are constructed and consumed in each frame.
            let map = Map::new(Some(tiles), &mut self.map_memory, myposition)
                .drag_gesture(!self.plugin_painter.painting_in_progress())
                .with_plugin(&self.plugin_painter)
                .with_plugin(geolocation::GeoLocationPlugin::new(geolocation));

            ui.add(map);

            // Draw utility windows.
            if !self.plugin_painter.painting_in_progress() {
                zoom(ui, &mut self.map_memory);
                if self.sources.len() > 1 {
                    controls(ui, &mut self.selected_source, &mut self.sources.keys());
                }
                acknowledge(ui, attribution);
                geolocation::GeoLocationPlugin::show_ui(ui, &mut self.map_memory, geolocation);
            }
            self.plugin_painter.show_ui(ui);
        });

        self.config_ctx
            .config_update(self.map_memory.zoom_get(), self.map_memory.detached());
    }
}
