#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Gui {
    pub scale: u32,
}

impl Gui {
    pub fn new() -> Self {
        Self::default()
    }
}
