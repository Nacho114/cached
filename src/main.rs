use std::{fs::File, io::Write};

use zellij_tile::prelude::*;

//TODO: Change path | Should come from config
const TAB_UPDATE_CACHE_PATH: &str = "/tmp/tab_update.json";
const PANE_MANIFEST_CACHE_PATH: &str = "/tmp/pane_manifest.json";

struct State {}

impl Default for State {
    fn default() -> Self {
        Self {}
    }
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        subscribe(&[EventType::TabUpdate, EventType::PaneUpdate]);
        hide_self();
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::TabUpdate(tab_info) => {
                let serialized = serde_json::to_string(&tab_info).unwrap();
                let mut file = File::create(TAB_UPDATE_CACHE_PATH).unwrap();
                file.write_all(serialized.as_bytes()).unwrap();
            }

            Event::PaneUpdate(pane_manifest) => {
                let serialized = serde_json::to_string(&pane_manifest).unwrap();
                let mut file = File::create(PANE_MANIFEST_CACHE_PATH).unwrap();
                file.write_all(serialized.as_bytes()).unwrap();
            }

            _ => (),
        };

        false
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        hide_self();
    }
}
