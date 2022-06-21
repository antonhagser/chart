use std::borrow::Cow;

use js_sys::Uint32Array;
use nalgebra::{Scale3, Translation3};
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlVertexArrayObject};

use self::layout::Layout;

use super::{draw::DrawInfo, EngineContext};

pub use default::DefaultVertex;

mod default;
mod layout;

pub trait Vertex {
    fn get_layout<'a>() -> Layout<'a>;
}

pub struct VertexArrayObject<'p, V>
where
    V: Vertex,
{
    ctx: EngineContext,
    object: WebGlVertexArrayObject,
    program_id: Cow<'p, str>,
    vertices: Option<Vec<V>>,
    indices: Option<Vec<u32>>,
    array_buffer: Option<WebGlBuffer>,
    element_buffer: Option<WebGlBuffer>,
}

impl<'p, V> VertexArrayObject<'p, V>
where
    V: Vertex,
{
    pub fn new(
        ctx: EngineContext,
        program_id: Cow<'p, str>,
        mut vertices: Option<Vec<V>>,
        indices: Option<Vec<u32>>,
    ) -> Self {
        let ob = ctx
            .gl()
            .create_vertex_array()
            .expect("failed to create vertex array object");

        ctx.gl().bind_vertex_array(Some(&ob));

        // Get the shader program
        let programs = ctx.programs();
        let program = programs
            .get(program_id.as_ref())
            .expect("program does not exist");

        ctx.gl().use_program(Some(&program.get()));
        drop(programs);

        let vb = if let Some(vertices) = &mut vertices {
            let vertex_buffer = ctx.gl().create_buffer().unwrap();

            // Convert vector to array of bytes
            let len = std::mem::size_of::<V>() * vertices.len();
            let vertices = unsafe {
                let out = Vec::<u8>::from_raw_parts(vertices.as_mut_ptr() as _, len, len);
                out
            };

            ctx.gl()
                .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
            ctx.gl()
                .buffer_data_with_u8_array_and_src_offset_and_length(
                    WebGl2RenderingContext::ARRAY_BUFFER,
                    &vertices,
                    WebGl2RenderingContext::STATIC_DRAW,
                    0,
                    vertices.len() as _,
                );

            // Get the shader program
            let programs = ctx.programs();
            let program = programs
                .get(program_id.as_ref())
                .expect("program does not exist");

            // Assign the vertex attributes
            let layout = V::get_layout();
            let mut offset = 0;
            for at in layout.attribute.iter() {
                let position = ctx.gl().get_attrib_location(program.get(), &at.name) as u32;

                ctx.gl().vertex_attrib_pointer_with_i32(
                    position,
                    at.count as i32,
                    at.type_,
                    false,
                    std::mem::size_of::<V>() as i32,
                    offset,
                );
                ctx.gl().enable_vertex_attrib_array(position);

                offset += at.size() as i32;
            }

            ctx.gl()
                .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

            Some(vertex_buffer)
        } else {
            None
        };

        let ib = if let Some(ref indices) = indices {
            let element_buffer = ctx.gl().create_buffer().unwrap();
            ctx.gl().bind_buffer(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                Some(&element_buffer),
            );

            let indices_array = Uint32Array::from(indices.as_slice());

            ctx.gl().buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &indices_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );

            ctx.gl()
                .bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);

            Some(element_buffer)
        } else {
            None
        };

        ctx.gl().bind_vertex_array(None);

        Self {
            ctx,
            object: ob,
            program_id,
            vertices,
            indices,
            array_buffer: vb,
            element_buffer: ib,
        }
    }

    pub fn draw(&self, _draw_info: DrawInfo) {
        // Get the shader program
        let programs = self.ctx.programs();
        let program = programs
            .get(self.program_id.as_ref())
            .expect("program does not exist");

        self.ctx.gl().use_program(Some(program.get()));

        self.ctx.gl().bind_vertex_array(Some(&self.object));
        self.ctx.gl().bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            self.element_buffer.as_ref(),
        );

        let model = Translation3::new(400.0, 300.0, 0.0);
        let mut model_matrix = model.to_homogeneous();
        model_matrix *= Scale3::new(80.0, 80.0, 1.0).to_homogeneous();

        let loc_u_model = self.ctx.gl().get_uniform_location(program.get(), "u_model");
        let loc_u_view = self.ctx.gl().get_uniform_location(program.get(), "u_view");
        let loc_u_proj = self.ctx.gl().get_uniform_location(program.get(), "u_proj");

        let camera = self.ctx.camera.borrow();
        let proj_matrix = camera.projection_matrix();
        let view_matrix = camera.view_matrix();

        self.ctx.gl().uniform_matrix4fv_with_f32_array(
            loc_u_model.as_ref(),
            false,
            model_matrix.as_slice(),
        );

        self.ctx.gl().uniform_matrix4fv_with_f32_array(
            loc_u_view.as_ref(),
            false,
            view_matrix.as_slice(),
        );

        self.ctx.gl().uniform_matrix4fv_with_f32_array(
            loc_u_proj.as_ref(),
            false,
            proj_matrix.as_slice(),
        );

        self.ctx.gl().draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLE_STRIP,
            self.indices.as_ref().unwrap().len() as i32,
            WebGl2RenderingContext::UNSIGNED_INT,
            0,
        );
    }
}
