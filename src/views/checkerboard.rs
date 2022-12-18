use std::time::Duration;

use crate::{TerminalScreen, TerminalView};

pub struct Checkerboard;

impl TerminalView for Checkerboard {
    fn init(&mut self, empty: &mut TerminalScreen) -> Result<(), ()> {
        for i in 0..empty.dimensions.1 {
            if i % 2 == 0 {
                empty.edit_line(
                    i, 
                    ['█', ' '].into_iter().cycle().take(empty.dimensions.0).collect()
                )
            } else {
                empty.edit_line(
                    i, 
                    [' ', '█'].into_iter().cycle().take(empty.dimensions.0).collect()
                )
            }
            
        }
        Ok(())
    }

    fn redraw(&mut self, old: &mut TerminalScreen) -> Result<(), ()> {
        Ok(())
    }

    fn fps(&self) -> f32 {
        1.0
    }
}