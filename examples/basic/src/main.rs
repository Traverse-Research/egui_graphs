use std::collections::HashMap;

use eframe::{run_native, App, CreationContext};
use egui::Context;
use egui_graphs::{Changes, Edge, Elements, GraphView, Node, Settings};

pub struct ExampleApp {
    elements: Elements,
    settings: Settings,
}

impl ExampleApp {
    fn new(_: &CreationContext<'_>) -> Self {
        let settings = Settings::default();
        let elements = generate_graph();
        Self { settings, elements }
    }

    fn apply_changes(&mut self, changes: Changes) {
        if !changes.is_some() {
            return;
        }

        // Here we track only location changes. You can track other changes as well.

        changes.nodes.iter().for_each(|(idx, change)| {
            if let Some(location_change) = change.location {
                let el_node = self.elements.get_node_mut(idx).unwrap();
                el_node.location = location_change;
            }
        });

        changes.edges.iter().for_each(|(idx, change)| {
            if let Some(width_change) = change.width {
                let edge = self.elements.get_edge_mut(idx).unwrap();
                edge.width = width_change;
            }
        });
    }
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        let widget = &GraphView::new(&self.elements, &self.settings);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(widget);
        });

        self.apply_changes(widget.last_changes());
    }
}

fn generate_graph() -> Elements {
    let mut nodes = HashMap::new();
    nodes.insert(0, Node::new(egui::Vec2::new(0., 30.)));
    nodes.insert(1, Node::new(egui::Vec2::new(-30., 0.)));
    nodes.insert(2, Node::new(egui::Vec2::new(30., 0.)));

    let mut edges = HashMap::new();
    edges.insert((0, 1), vec![Edge::new(0, 1, 0)]);
    edges.insert((1, 2), vec![Edge::new(1, 2, 0)]);
    edges.insert((2, 0), vec![Edge::new(2, 0, 0)]);

    Elements::new(nodes, edges)
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    run_native(
        "egui_graphs_basic_demo",
        native_options,
        Box::new(|cc| Box::new(ExampleApp::new(cc))),
    )
    .unwrap();
}