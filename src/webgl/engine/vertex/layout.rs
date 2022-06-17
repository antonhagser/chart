use std::borrow::Cow;

pub struct Attribute<'a> {
    pub name: Cow<'a, str>,
    pub type_size: usize,
    pub count: usize,
    pub type_: u32,
}

impl<'a> Attribute<'a> {
    pub fn new<A>(name: &'a str, count: usize, type_: u32) -> Self {
        Self {
            name: name.into(),
            type_size: std::mem::size_of::<A>(),
            count,
            type_,
        }
    }

    pub fn size(&self) -> usize {
        self.type_size * self.count
    }
}

pub struct Layout<'a> {
    pub attribute: Vec<Attribute<'a>>,
}
