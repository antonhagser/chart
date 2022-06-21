use web_sys::WebGl2RenderingContext;

use crate::webgl::engine::EngineContext;

use super::Shader;

pub struct FragmentShader {
    shader: web_sys::WebGlShader,
}

impl FragmentShader {
    pub fn new(context: EngineContext, vert_code: &str) -> Self {
        let vert_shader = context
            .gl()
            .create_shader(WebGl2RenderingContext::FRAGMENT_SHADER)
            .unwrap();
        context.gl().shader_source(&vert_shader, vert_code);
        context.gl().compile_shader(&vert_shader);

        Self {
            shader: vert_shader,
        }
    }
}

impl Shader for FragmentShader {
    fn borrow(&self) -> &web_sys::WebGlShader {
        &self.shader
    }
}
