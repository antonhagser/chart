use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
    time::Duration,
};

use web_sys::WebGl2RenderingContext;
use yew::html::Scope;

use crate::{components::chart::Props, webgl::engine::draw::DrawInfo};

use self::{
    color::Color,
    program::ShaderProgram,
    vertex::{DefaultVertex, VertexArrayObject},
};

pub mod color;
pub mod draw;
pub mod perspective;
pub mod primitives;
pub mod program;
pub mod renderer;
pub mod shader;
pub mod texture;
pub mod vertex;

type EngineContext = Rc<Context>;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Vertex {
    pos: [f32; 2],
    tex_coord: [f32; 2],
    color: [f32; 4],
}

pub struct Context {
    gl: Rc<WebGl2RenderingContext>,
    programs: RefCell<HashMap<String, ShaderProgram>>,
    camera: RefCell<perspective::Camera2D>,
}

impl Context {
    pub fn register_program(&self, id: String, program: ShaderProgram) {
        self.programs.borrow_mut().insert(id, program);
    }

    /// Get a reference to the context's gl.
    #[must_use]
    pub fn gl(&self) -> &WebGl2RenderingContext {
        &self.gl
    }

    /// Get a reference to the context's programs.
    #[must_use]
    pub fn programs(&self) -> Ref<HashMap<String, ShaderProgram>> {
        self.programs.borrow()
    }
}

pub struct Engine {
    context: EngineContext,
}

impl Engine {
    pub fn new(gl: WebGl2RenderingContext, width: u32, height: u32) -> Self {
        let gl = Rc::new(gl);

        info!("Rendering on canvas with size {}x{}", width, height);
        gl.viewport(0, 0, width as i32, height as i32);

        let rb = gl.create_renderbuffer();
        gl.bind_renderbuffer(WebGl2RenderingContext::RENDERBUFFER, rb.as_ref());

        let context = Rc::new(Context {
            gl,
            programs: RefCell::new(HashMap::new()),
            camera: RefCell::new(perspective::Camera2D::new(width, height)),
        });

        Self { context }
    }

    pub fn render<S: yew::Component>(
        &self,
        timestamp: f64,
        frametime: Duration,
        _link: &Scope<S>,
        _props: &Props,
    ) {
        // Begin by clearing the screen.
        self.gl()
            .clear_color(0.074509803921569, 0.078443137254902, 0.082352941176471, 1.0);
        self.gl().clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        self.gl().clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);

        let drawinfo = DrawInfo {
            frametime,
            timestamp,
        };

        // Debugging
        let mut vertices: Vec<DefaultVertex> = Vec::new();
        vertices.push(DefaultVertex {
            position: [-1.0, 1.0],
            color: Color::new(245, 184, 65).into(),
        });

        vertices.push(DefaultVertex {
            position: [1.0, 1.0],
            color: Color::new(245, 184, 65).into(),
        });

        vertices.push(DefaultVertex {
            position: [1.0, -1.0],
            color: Color::new(245, 184, 65).into(),
        });

        vertices.push(DefaultVertex {
            position: [-1.0, -1.0],
            color: Color::new(245, 184, 65).into(),
        });

        let indices = vec![0, 1, 2, 2, 3, 0];

        let vao = VertexArrayObject::new(
            self.context.clone(),
            "default".into(),
            Some(vertices),
            Some(indices),
        );

        vao.draw(drawinfo);
    }

    /// Get a reference to the context's WebGL2 Context.
    #[must_use]
    pub fn gl(&self) -> &WebGl2RenderingContext {
        &self.context.gl
    }

    /// Get a reference to the context's shader programs.
    #[must_use]
    pub fn programs(&self) -> Ref<HashMap<String, ShaderProgram>> {
        self.context.programs.borrow()
    }

    /// Get the renderer's context.
    #[must_use]
    pub fn context(&self) -> EngineContext {
        self.context.clone()
    }
}
