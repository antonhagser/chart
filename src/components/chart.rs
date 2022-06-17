use std::time::Duration;

use gloo_render::{request_animation_frame, AnimationFrame};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use yew::{html::Scope, prelude::*};

use crate::webgl::engine::{
    program::ShaderProgram,
    shader::{fragment::FragmentShader, vertex::VertexShader},
    Engine,
};

pub enum Msg {
    Render(f64),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub color: (f32, f32, f32),
}

pub struct Chart {
    node_ref: NodeRef,
    _render_loop: Option<AnimationFrame>,

    renderer: Option<Engine>,
    frametime: Duration,
    frames: u64,
}

impl Component for Chart {
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            _render_loop: None,

            renderer: None,
            frametime: Duration::default(),
            frames: 0,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas class="chart" width="800" height="600" ref={self.node_ref.clone()} />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            // Get the canvas
            let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

            // Get the WebGL2 renderer
            let gl: WebGl2RenderingContext = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();

            self.renderer = Some(Engine::new(gl.clone(), canvas.width(), canvas.height()));

            // The callback to request animation frame is passed a time value which can be used for
            // rendering motion independent of the framerate which may vary.
            let handle = {
                let link = ctx.link().clone();
                request_animation_frame(move |time| link.send_message(Msg::Render(time)))
            };

            let r = self.renderer.as_ref().unwrap();

            // Initialize webgl `default` shader program
            let vert_code = include_str!("../../assets/shaders/basic.vert");
            let frag_code = include_str!("../../assets/shaders/basic.frag");

            let vertex = VertexShader::new(r.context(), vert_code);
            let fragment = FragmentShader::new(r.context(), frag_code);

            let program = ShaderProgram::new(r.context(), vertex, fragment);

            r.context().register_program("default".into(), program);

            self._render_loop = Some(handle);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Render(timestamp) => {
                // Render the scene
                self.render(timestamp, ctx.link(), ctx.props());
                false
            }
        }
    }
}

impl Chart {
    fn render(&mut self, timestamp: f64, link: &Scope<Self>, props: &Props) {
        // Get instant
        let now = wasm_timer::Instant::now();

        // Render the scene
        self.renderer
            .as_ref()
            .expect("webgl2 renderer not initialized!")
            .render::<Self>(timestamp, self.frametime, link, props);

        // Update the frametime
        let el = now.elapsed();
        self.frametime = el;
        self.frames += 1;

        if self.frames > 100 && self.frames % 100 == 0 && (timestamp / 1000.0) > 0.0 {
            info!("{}", self.frames / (timestamp / 1000.0) as u64);
        }

        // Request the next frame
        let handle = {
            let link = link.clone();
            request_animation_frame(move |time| link.send_message(Msg::Render(time)))
        };

        // A reference to the new handle must be retained for the next render to run.
        self._render_loop = Some(handle);
    }
}
