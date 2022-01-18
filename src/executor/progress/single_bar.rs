use crate::executor::progress::display_props::DisplayProps;
use colored::*;
use std::sync::mpsc;
use std::time;

const REFRESH_RATE: time::Duration = time::Duration::from_millis(10);
const DONE_TAIL: &str = "Done";
const ERROR_TAIL: &str = "Error";
const SPINNER_TEXT: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

const MODE_SPINNER: usize = 1;
const MODE_PROGRESS: usize = 2;
const MODE_DONE: usize = 3;
const MODE_ERROR: usize = 4;

pub struct singleBarCore {
    pub display_props: DisplayProps,
    spinner_frame: usize,
}

impl singleBarCore {
    fn render_done_or_error(&self) {
        let display_prefix: String;
        let tail_color: ColoredString;

        match Some(self.display_props.mode) {
            ModeDone => tail_color = DONE_TAIL.to_string().green(),
            ModeError => tail_color = ERROR_TAIL.to_string().red(),
            _ => {
                panic!("Unexpect self.Mode");
            }
        }

        println!("{}{}", self.display_props.prefix, tail_color);
    }

    fn render_spinner(&mut self) {
        let display_refix = self.display_props.prefix.clone();
        println!(
            "{} {} {}",
            display_refix, SPINNER_TEXT[self.spinner_frame], self.display_props.suffix
        );
        self.spinner_frame = (self.spinner_frame + 1) % SPINNER_TEXT.len();
    }

    fn renderTo(&mut self) {
        if self.display_props.mode == MODE_DONE || self.display_props.mode == MODE_ERROR {
            self.render_done_or_error()
        } else {
            self.render_spinner()
        }
    }

    fn newSingleBarCore(prefix: String, suffix: String) -> singleBarCore {
        let tmp_display_props = DisplayProps {
            prefix: prefix,
            suffix: suffix,
            mode: MODE_SPINNER,
        };
        return {
            singleBarCore {
                display_props: tmp_display_props,
                spinner_frame: 0,
            }
        };
    }
}
