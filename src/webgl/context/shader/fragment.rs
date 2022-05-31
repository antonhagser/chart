use std::rc::Rc;

use web_sys::WebGl2RenderingContext;

use super::Shader;

pub struct FragmentShader {
    shader: web_sys::WebGlShader,
}

impl FragmentShader {
    pub fn new(gl: Rc<WebGl2RenderingContext>, vert_code: &str) -> Self {
        let vert_shader = gl.create_shader(WebGl2RenderingContext::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&vert_shader, vert_code);
        gl.compile_shader(&vert_shader);

        Self { shader: vert_shader }
    }
}

impl Shader for FragmentShader {
    fn borrow(&self) -> &web_sys::WebGlShader {
        &self.shader
    }
}
