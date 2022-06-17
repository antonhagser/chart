use std::{rc::Rc, borrow::Cow};

use web_sys::{WebGlTexture, WebGl2RenderingContext};

pub struct Texture<'a, T: Clone> {
    texture: WebGlTexture,
    gl: Rc<WebGl2RenderingContext>,
    data: Cow<'a, [T]>,
}

impl<'a, T> Texture<'a, T> where T: Clone {
    pub fn new(gl: Rc<WebGl2RenderingContext>, data: Cow<'a, [T]>, height: usize, width: usize) -> Self {
        let texture = gl.create_texture().expect("failed to create texture");
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        let tex = Self {
            gl,
            texture,
            data,
        };

        // tex.clear();
        tex
    }

    // pub fn clear(&self) {
    //     self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
    //         WebGl2RenderingContext::TEXTURE_2D,
    //         0,
    //         WebGl2RenderingContext::RGBA as i32,
    //         1,
    //         1,
    //         0,
    //         WebGl2RenderingContext::RGBA,
    //         WebGl2RenderingContext::UNSIGNED_BYTE,
    //         Some(&[0, 0, 255, 255]),
    //     ).expect("failed to write to texture");
    // }

    pub fn write_data(&mut self, width: usize, height: usize, data: Cow<'a, [u8]>) {
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.texture));
        self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            width as i32,
            height as i32,
            0,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            Some(data.as_ref()),
        ).expect("failed to write to texture");

        trace!("wrote data to texture");
    }

    /// Get a reference to the texture's texture.
    #[must_use]
    pub fn texture(&self) -> &WebGlTexture {
        &self.texture
    }
}
