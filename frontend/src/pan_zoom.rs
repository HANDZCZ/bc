use egui::emath::TSTransform;

pub struct PanZoom {
    transform: TSTransform,
    objects: Vec<Box<dyn PanZoomObject>>,
    title: String,
    pub open: bool,
}

pub trait PanZoomObject {
    fn id(&self) -> String;
    fn pos(&self) -> egui::Pos2;
    fn ui(&mut self, ui: &mut egui::Ui);
}

impl PanZoom {
    pub fn new(title: String, objects: Vec<Box<dyn PanZoomObject>>) -> Self {
        PanZoom {
            objects,
            title,
            open: true,
            transform: Default::default()
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;
        let window = egui::Window::new(&self.title)
            .default_width(300.0)
            .default_height(300.0)
            .vscroll(false)
            .open(&mut open);
        window.show(ctx, |ui| self.ui(ui));
        self.open = open;
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        let (id, rect) = ui.allocate_space(ui.available_size());
        let response = ui.interact(rect, id, egui::Sense::click_and_drag());
        // Allow dragging the background as well.
        if response.dragged() {
            self.transform.translation += response.drag_delta();
        }

        // Plot-like reset
        if response.double_clicked() {
            self.transform = TSTransform::default();
        }

        let transform =
            TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * self.transform;

        if let Some(pointer) = ui.ctx().input(|i| i.pointer.hover_pos()) {
            // Note: doesn't catch zooming / panning if a button in this PanZoom container is hovered.
            if response.hovered() {
                let pointer_in_layer = transform.inverse() * pointer;
                let zoom_delta = ui.ctx().input(|i| i.zoom_delta());
                let pan_delta = ui.ctx().input(|i| i.smooth_scroll_delta);

                // Zoom in on pointer:
                self.transform = self.transform
                    * TSTransform::from_translation(pointer_in_layer.to_vec2())
                    * TSTransform::from_scaling(zoom_delta)
                    * TSTransform::from_translation(-pointer_in_layer.to_vec2());

                // Pan:
                self.transform = TSTransform::from_translation(pan_delta) * self.transform;
            }
        }

        for obj in &mut self.objects {
            let id = egui::Area::new(obj.id())
                .fixed_pos(obj.pos())
                // Need to cover up the pan_zoom demo window,
                // but may also cover over other windows.
                .order(egui::Order::Foreground)
                .show(ui.ctx(), |ui| {
                    ui.set_clip_rect(transform.inverse() * rect);
                    egui::Frame::default()
                        .rounding(egui::Rounding::same(4.0))
                        .inner_margin(egui::Margin::same(8.0))
                        .stroke(ui.ctx().style().visuals.window_stroke)
                        .fill(ui.style().visuals.panel_fill)
                        .show(ui, |ui| {
                            ui.style_mut().wrap = Some(false);
                            obj.ui(ui)
                        });
                })
                .response
                .layer_id;
            ui.ctx().set_transform_layer(id, transform);
        }
    }
}
