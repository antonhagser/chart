pub struct PrimitiveColor(f32, f32, f32);

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    pub const fn with_alpha(mut self, a: u8) -> Self {
        self.a = a;
        self
    }
}

impl From<Color> for PrimitiveColor {
    fn from(color: Color) -> Self {
        PrimitiveColor(
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
        )
    }
}
