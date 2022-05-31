use std::{collections::HashMap, rc::Rc};

use uuid::Uuid;
use web_sys::WebGl2RenderingContext;
use yew::html::Scope;

use crate::components::chart::Props;

use self::program::ShaderProgram;

pub mod program;
pub mod shader;

pub struct RenderingContext {
    gl: Rc<WebGl2RenderingContext>,
    programs: HashMap<String, ShaderProgram>,
}

impl RenderingContext {
    pub fn new(gl: WebGl2RenderingContext) -> Self {
        Self {
            gl: Rc::new(gl),
            programs: HashMap::new(),
        }
    }

    pub fn register_program(&mut self, id: String, program: ShaderProgram) {
        self.programs.insert(id, program);
    }

    pub fn render<S: yew::Component>(&self, timestamp: f64, link: &Scope<S>, props: &Props) {}

    /// Get a reference to the context's gl.
    #[must_use]
    pub fn gl(&self) -> &WebGl2RenderingContext {
        &self.gl
    }
}
