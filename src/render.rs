use std::{time::{Instant, Duration}, thread::sleep};

use crate::{TerminalScreen, TerminalView};

const ESC: char = 27 as char;

pub fn render_cycle(mut view: impl TerminalView) -> ! {
    let mut screen = TerminalScreen::new(10, 10);
    view.init(&mut screen).unwrap();
    loop {
        let start = Instant::now();
        view.redraw(&mut screen).unwrap();
        print!("{ESC}[{}A", screen.dimensions.1);
        for i in 0..screen.dimensions.1 {
            print!("{ESC}[2K{}{ESC}[1E", screen.lines[i])
        }
        sleep(Duration::from_secs_f32(1./view.fps()).saturating_sub(start.elapsed()))
    }
}
