use web_sys::WebGlShader;

pub mod fragment;
pub mod vertex;

pub trait Shader {
    fn borrow(&self) -> &WebGlShader;
}