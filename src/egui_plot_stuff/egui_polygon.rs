use egui::{Color32, DragValue, Id, Slider, Stroke, Ui};
use egui_plot::{LineStyle, PlotPoints, PlotResponse, PlotUi, Polygon};

use crate::egui_plot_stuff::colors::{Rgb, COLOR_OPTIONS};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct EguiPolygon {
    pub draw: bool,
    pub name_in_legend: bool,
    pub name: String,
    pub highlighted: bool,
    pub stroke: Stroke,
    pub width: f32,
    pub fill_color: Color32,
    #[serde(skip)]
    pub style: Option<LineStyle>,
    pub style_length: f32,
    pub vertices: Vec<[f64; 2]>,
    pub vertices_radius: f32,
    // Use Rgb struct for custom RGB values
    pub color_rgb: Rgb,
    pub stroke_rgb: Rgb,

    pub interactive_clicking: bool,
    pub interactive_dragging: bool,

    #[serde(skip)]
    pub is_dragging: bool,
    #[serde(skip)]
    pub dragged_vertex_index: Option<usize>,
}

impl Default for EguiPolygon {
    fn default() -> Self {
        EguiPolygon {
            draw: true,
            name_in_legend: false,
            name: "Polygon".to_string(),
            highlighted: false,
            stroke: Stroke::new(1.0, Color32::RED),
            width: 2.0,
            fill_color: Color32::TRANSPARENT,
            style: Some(LineStyle::Solid),
            style_length: 15.0,
            vertices: vec![],
            vertices_radius: 5.0,
            color_rgb: Rgb::from_color32(Color32::RED),
            stroke_rgb: Rgb::from_color32(Color32::RED),

            interactive_clicking: false,
            interactive_dragging: true,
            is_dragging: false,
            dragged_vertex_index: None,
        }
    }
}

impl EguiPolygon {
    pub fn new(name: &str) -> Self {
        EguiPolygon {
            name: name.to_string(),
            interactive_clicking: true,
            ..Default::default()
        }
    }

    pub fn handle_interactions(&mut self, plot_response: &PlotResponse<()>) {
        let pointer_state = plot_response.response.ctx.input(|i| i.pointer.clone());
        if self.interactive_clicking && self.draw {
            if let Some(pointer_pos) = pointer_state.hover_pos() {
                if plot_response.response.clicked() {
                    self.add_vertex(pointer_pos.x.into(), pointer_pos.y.into());
                    // self.dragged_vertex_index = Some(self.vertices.len() - 1);
                }
            }
        }

        if self.interactive_dragging && self.draw {
            let pointer_state = plot_response.response.ctx.input(|i| i.pointer.clone());
            if let Some(pointer_pos) = pointer_state.hover_pos() {
                if let Some(hovered_id) = plot_response.hovered_plot_item {
                    if hovered_id == Id::new(self.name.clone()) {
                        self.highlighted = true;

                        // Find index of the closest vertex to the pointer
                        let closest_index = self
                            .vertices
                            .iter()
                            .enumerate()
                            .min_by(|(_, a), (_, b)| {
                                let dist_a = (a[0] - pointer_pos.x as f64).powi(2)
                                    + (a[1] - pointer_pos.y as f64).powi(2);
                                let dist_b = (b[0] - pointer_pos.x as f64).powi(2)
                                    + (b[1] - pointer_pos.y as f64).powi(2);
                                dist_a.partial_cmp(&dist_b).unwrap()
                            })
                            .map(|(index, _)| index);

                        self.dragged_vertex_index = closest_index;

                        if pointer_state.button_pressed(egui::PointerButton::Middle) {
                            self.is_dragging = true;
                        }
                    } else {
                        self.highlighted = false;
                    }
                } else {
                    self.highlighted = false;
                }

                if self.is_dragging {
                    if let Some(index) = self.dragged_vertex_index {
                        let plot_pos = plot_response.transform.value_from_position(pointer_pos);
                        self.vertices[index] = [plot_pos.x, plot_pos.y];
                    }

                    if pointer_state.button_released(egui::PointerButton::Middle) {
                        self.is_dragging = false;
                        self.dragged_vertex_index = None;
                    }
                }
            } else if pointer_state.button_released(egui::PointerButton::Middle) {
                self.is_dragging = false;
                self.dragged_vertex_index = None;
            }
        }
    }

    pub fn add_vertex(&mut self, x: f64, y: f64) {
        self.vertices.push([x, y]);
    }

    pub fn clear_vertices(&mut self) {
        self.vertices.clear();
    }

    pub fn draw(&self, plot_ui: &mut PlotUi) {
        if self.draw {
            let vertices: PlotPoints = PlotPoints::new(self.vertices.clone());
            let vertices_points = egui_plot::Points::new(self.vertices.clone())
                .radius(self.vertices_radius)
                .color(self.stroke.color)
                .id(egui::Id::new(self.name.clone()));

            let mut polygon = Polygon::new(vertices)
                .highlight(self.highlighted)
                .stroke(self.stroke)
                .width(self.width)
                .fill_color(Color32::TRANSPARENT);

            if self.name_in_legend {
                polygon = polygon.name(self.name.clone());
            }

            if self.style.is_some() {
                polygon = polygon.style(self.style.unwrap());
            }

            plot_ui.polygon(polygon);
            plot_ui.points(vertices_points);
        }
    }

    pub fn menu_button(&mut self, ui: &mut Ui) {
        ui.menu_button(self.name.to_string(), |ui| {
            ui.vertical(|ui| {
                ui.text_edit_singleline(&mut self.name);
                ui.checkbox(&mut self.draw, "Draw Polygon");
                ui.checkbox(
                    &mut self.interactive_clicking,
                    "Interactive Adding Vertices",
                );
                ui.checkbox(
                    &mut self.interactive_dragging,
                    "Interactive Dragging Vertices",
                );
                ui.checkbox(&mut self.name_in_legend, "Name in Legend")
                    .on_hover_text("Show in legend");
                ui.checkbox(&mut self.highlighted, "Highlighted");

                ui.add(
                    DragValue::new(&mut self.vertices_radius)
                        .speed(0.1)
                        .prefix("Vertex Radius: "),
                );

                ui.add(Slider::new(&mut self.width, 0.0..=10.0).text("Line Width"));

                self.stroke_color_selection_buttons(ui);

                ui.horizontal(|ui| {
                    ui.label("Line Style: ");
                    ui.radio_value(&mut self.style, Some(LineStyle::Solid), "Solid");
                    ui.radio_value(
                        &mut self.style,
                        Some(LineStyle::Dotted {
                            spacing: self.style_length,
                        }),
                        "Dotted",
                    );
                    ui.radio_value(
                        &mut self.style,
                        Some(LineStyle::Dashed {
                            length: self.style_length,
                        }),
                        "Dashed",
                    );
                    ui.add(
                        DragValue::new(&mut self.style_length)
                            .speed(1.0)
                            .clamp_range(0.0..=f32::INFINITY)
                            .prefix("Length: "),
                    );
                });
            });

            ui.separator();
            if ui.button("Clear Vertices").clicked() {
                self.clear_vertices();
            }
        });
    }

    pub fn stroke_color_selection_buttons(&mut self, ui: &mut Ui) {
        ui.label("Color");
        ui.horizontal_wrapped(|ui| {
            for &(color, _) in COLOR_OPTIONS.iter() {
                if ui.add(egui::Button::new(" ").fill(color)).clicked() {
                    self.stroke.color = color;
                    self.stroke_rgb = Rgb::from_color32(color);
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("RGB: ");
            ui.add(
                DragValue::new(&mut self.stroke_rgb.r)
                    .clamp_range(0..=255)
                    .prefix("R: "),
            );
            ui.add(
                DragValue::new(&mut self.stroke_rgb.g)
                    .clamp_range(0..=255)
                    .prefix("G: "),
            );
            ui.add(
                DragValue::new(&mut self.stroke_rgb.b)
                    .clamp_range(0..=255)
                    .prefix("B: "),
            );

            self.stroke.color = self.stroke_rgb.to_color32();
        });
    }
}
