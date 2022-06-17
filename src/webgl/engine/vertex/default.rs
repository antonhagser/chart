use web_sys::WebGl2RenderingContext;

use crate::webgl::engine::color::PrimitiveColor;

use super::{Vertex, layout::{Layout, Attribute}};

pub struct DefaultVertex {
    pub position: [f32; 2],
    pub color: PrimitiveColor,
}

impl Vertex for DefaultVertex {
    fn get_layout<'a>() -> Layout<'a> {
        Layout {
            attribute: vec![
                Attribute {
                    name: "a_position".into(),
                    type_size: std::mem::size_of::<f32>(),
                    count: 2,
                    type_: WebGl2RenderingContext::FLOAT,
                },
                Attribute {
                    name: "a_color".into(),
                    type_size: std::mem::size_of::<f32>(),
                    count: 3,
                    type_: WebGl2RenderingContext::FLOAT,
                },
            ],
        }
    }
}