use web_sys::WebGlProgram;

use super::{
    shader::{fragment::FragmentShader, vertex::VertexShader, Shader},
    RenderingContext,
};

pub struct ShaderProgram {
    vertex: VertexShader,
    fragment: FragmentShader,
    program: WebGlProgram,
}

impl ShaderProgram {
    pub fn new(ctx: RenderingContext, vertex: VertexShader, fragment: FragmentShader) -> Self {
        let program = ctx
            .gl
            .create_program()
            .expect("could not create shader program");

        // Attach shaders to program
        ctx.gl.attach_shader(&program, &vertex.borrow());
        ctx.gl.attach_shader(&program, &fragment.borrow());

        // Link the program
        ctx.gl.link_program(&program);

        Self {
            vertex,
            fragment,
            program,
        }
    }

    /// Get a reference to the shader program's program.
    #[must_use]
    pub fn get(&self) -> &WebGlProgram {
        &self.program
    }
}
