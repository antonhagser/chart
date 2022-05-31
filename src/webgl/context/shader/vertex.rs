use std::rc::Rc;

use web_sys::WebGl2RenderingContext;

use super::Shader;

pub struct VertexShader {
    shader: web_sys::WebGlShader,
}

impl VertexShader {
    pub fn new(gl: Rc<WebGl2RenderingContext>, vert_code: &str) -> Self {
        let vert_shader = gl.create_shader(WebGl2RenderingContext::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, vert_code);
        gl.compile_shader(&vert_shader);

        Self { shader: vert_shader }
    }
}

impl Shader for VertexShader {
    fn borrow(&self) -> &web_sys::WebGlShader {
        &self.shader
    }
}
