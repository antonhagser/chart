use gloo_render::{request_animation_frame, AnimationFrame};
use log::info;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlProgram};
use yew::{html::Scope, prelude::*};

use crate::webgl::context::RenderingContext;

pub enum Msg {
    Render(f64),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub color: (f32, f32, f32),
}

pub struct Chart {
    gl: Option<GL>,
    node_ref: NodeRef,
    _render_loop: Option<AnimationFrame>,

    shader_program: Option<WebGlProgram>,

    context: Option<RenderingContext>,
}

impl Component for Chart {
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            gl: None,
            node_ref: NodeRef::default(),
            _render_loop: None,

            shader_program: None,

            context: None,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas class="chart" ref={self.node_ref.clone()} />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            // Get the canvas
            let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

            // Get the WebGL2 context
            let gl: GL = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();

            self.context = Some(RenderingContext::new(gl.clone()));

            log::info!("First render of chart");

            // The callback to request animation frame is passed a time value which can be used for
            // rendering motion independent of the framerate which may vary.
            let handle = {
                let link = ctx.link().clone();
                request_animation_frame(move |time| link.send_message(Msg::Render(time)))
            };

            // // Initialize open gl shaders
            // let vert_code = include_str!("./basic.vert");
            // let frag_code = include_str!("./basic.frag");

            // let gl = self.gl.as_ref().unwrap();

            // let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
            // gl.shader_source(&vert_shader, vert_code);
            // gl.compile_shader(&vert_shader);

            // let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
            // gl.shader_source(&frag_shader, frag_code);
            // gl.compile_shader(&frag_shader);

            // let shader_program = gl.create_program().unwrap();
            // gl.attach_shader(&shader_program, &vert_shader);
            // gl.attach_shader(&shader_program, &frag_shader);
            // gl.link_program(&shader_program);

            // self.shader_program = Some(shader_program);

            // A reference to the handle must be stored, otherwise it is dropped and the render
            // won't occur.
            self._render_loop = Some(handle);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Render(timestamp) => {
                info!("Render at {}", timestamp);

                // Render functions are likely to get quite large, so it is good practice to split
                // it into it's own function rather than keeping it inline in the update match
                // case. This also allows for updating other UI elements that may be rendered in
                // the DOM like a framerate counter, or other overlaid textual elements.
                self.render(timestamp, ctx.link(), ctx.props());
                false
            }
        }
    }
}

impl Chart {
    fn render(&mut self, timestamp: f64, link: &Scope<Self>, props: &Props) {
        // let gl = self.gl.as_ref().expect("GL Context not initialized!");

        // // This list of vertices will draw two triangles to cover the entire canvas.
        // let vertices: Vec<f32> = vec![
        //     -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        // ];

        // let vertex_buffer = gl.create_buffer().unwrap();
        // let verts = js_sys::Float32Array::from(vertices.as_slice());

        // gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        // gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

        // gl.use_program(Some(self.shader_program.as_ref().unwrap()));

        // // Attach the position vector as an attribute for the GL context.
        // let position =
        //     gl.get_attrib_location(self.shader_program.as_ref().unwrap(), "a_position") as u32;
        // gl.vertex_attrib_pointer_with_i32(position, 2, GL::FLOAT, false, 0, 0);
        // gl.enable_vertex_attrib_array(position);

        // // Attach the color as a uniform for the GL context.
        // let color = gl.get_uniform_location(self.shader_program.as_ref().unwrap(), "u_color");
        // gl.uniform3fv_with_f32_array(
        //     color.as_ref(),
        //     &[props.color.0, props.color.1, props.color.2],
        // );

        // let time = gl.get_uniform_location(self.shader_program.as_ref().unwrap(), "u_time");
        // gl.uniform1f(time.as_ref(), timestamp as f32);

        // gl.draw_arrays(GL::TRIANGLES, 0, 6);

        self.context
            .as_ref()
            .expect("webgl2 context not initialized!")
            .render::<Self>(timestamp, link, props);

        let handle = {
            let link = link.clone();
            request_animation_frame(move |time| link.send_message(Msg::Render(time)))
        };

        // A reference to the new handle must be retained for the next render to run.
        self._render_loop = Some(handle);
    }
}
