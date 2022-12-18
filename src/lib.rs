mod views;
mod render;

pub use render::render_cycle;
pub use views::Checkerboard;

pub struct Size(usize, usize);

pub struct TerminalScreen {
    /// List of the lines of the terminal screen.
    lines: Vec<String>,
    /// The height and size of the terminal screen.
    pub dimensions: Size
}

impl TerminalScreen {
    fn new(width: usize, height: usize) -> Self {
        Self {
            lines: vec![String::new(); height],
            dimensions: Size(width, height)
        }
    }
    fn edit_line(&mut self, index: usize, new: String) {
        self.lines[index] = new;
    }
}

pub trait TerminalView {
    fn init(&mut self, empty: &mut TerminalScreen) -> Result<(), ()>;
    /// Redraws the old screen.
    fn redraw(&mut self, old: &mut TerminalScreen) -> Result<(), ()>;
    fn fps(&self) -> f32;
}