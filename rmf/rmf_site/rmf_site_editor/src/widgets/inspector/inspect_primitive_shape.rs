/*
 * Copyright (C) 2023 Open Source Robotics Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/

use bevy_egui::egui::Ui;
use rmf_site_format::{PrimitiveShape, RecallPrimitiveShape};

pub struct InspectPrimitiveShape<'a> {
    pub primitive: &'a PrimitiveShape,
    pub recall: &'a RecallPrimitiveShape,
}

impl<'a> InspectPrimitiveShape<'a> {
    pub fn new(primitive: &'a PrimitiveShape, recall: &'a RecallPrimitiveShape) -> Self {
        Self { primitive, recall }
    }

    pub fn show(self, ui: &mut Ui) -> Option<PrimitiveShape> {
        let mut new_primitive = self.primitive.clone();
        ui.horizontal(|ui| {
            ui.label("Primitive:");
            // TODO(luca) make this a ComboBox again once we have a system to update primitives
            ui.label(new_primitive.label());
        });
        // TODO(luca) Make these values editable and implement a system to parse changes
        match &mut new_primitive {
            PrimitiveShape::Box { size } => {
                ui.label("Size");
                ui.horizontal(|ui| {
                    ui.label("X:");
                    ui.label(size[0].to_string());
                    ui.label("Y:");
                    ui.label(size[1].to_string());
                    ui.label("Z:");
                    ui.label(size[2].to_string());
                });
            }
            PrimitiveShape::Cylinder { radius, length }
            | PrimitiveShape::Capsule { radius, length } => {
                ui.horizontal(|ui| {
                    ui.label("Radius:");
                    ui.label(radius.to_string());
                    ui.label("Length:");
                    ui.label(length.to_string());
                });
            }
            PrimitiveShape::Sphere { radius } => {
                ui.horizontal(|ui| {
                    ui.label("Radius:");
                    ui.label(radius.to_string());
                });
            }
        }
        if &new_primitive != self.primitive {
            Some(new_primitive)
        } else {
            None
        }
    }
}
