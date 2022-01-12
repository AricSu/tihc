use colored::*;
use std::sync::mpsc;
use std::time;
use crate::executor::progress::display_props::DisplayProps;

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

// // SingleBar renders single progress bar.
// pub struct SingleBar  {
// 	core     :singleBarCore,
// 	renderer :renderer
// }

// impl SingleBar{
//     // NewSingleBar creates a new SingleBar.
//     pub fn new_single_bar(&mut self,prefix: String) -> SingleBar {
//     	let b = SingleBar{
//     		core:     newSingleBarCore(prefix),
//     		renderer: newRenderer(),
//     	};
//     	 self.renderer.renderFn =  self.render;
//     	return b
//     }

//     // UpdateDisplay updates the display property of this single bar.
//     // This fntion is thread safe.
//     pub fn UpdateDisplay(&mut self,newDisplay: DisplayProps) {
//     	 self.core.displayProps.Store(newDisplay)
//     }

//     // StartRenderLoop starts the render loop.
//     // This fntion is thread safe.
//     pub fn StartRenderLoop() {
//     	 self.preRender();
//     	 self.renderer.startRenderLoop();
//     }

//     // StopRenderLoop stops the render loop.
//     // This fntion is thread safe.
//     pub fn StopRenderLoop() {
//     	 self.renderer.stopRenderLoop();
//     }

//     pub fn preRender() {
//     	// Preserve space for the bar
//     	fmt.Println("");
//     }

//     fn render() {
//     	f = bufio.NewWriter(os.Stdout);

//     	moveCursorUp(f, 1);
//     	moveCursorToLineStart(f);
//     	clearLine(f);

//     	 self.core.renderTo(f);

//     	moveCursorDown(f, 1);
//     	moveCursorToLineStart(f);
//     	let _ = f.Flush();
//     }

// }

// pub struct renderer {
// 	isUpdaterRunning: Bool,
// 	stopChan:         mpsc::channel(),
// 	stopFinishedChan: mpsc::channel(),
// 	renderFn:         func()
// }

// impl renderer{

//     pub fn newRenderer() -> renderer {
//     	return &renderer{
//     		isUpdaterRunning: Bool,
//     		stopChan:         nil,
//     		stopFinishedChan: nil,
//     		renderFn:         nil,
//     	}
//     }

//     pub fn startRenderLoop(&self) {
//     	if  self.renderFn == nil {
//     		panic("renderFn must be set")
//     	}
//     	if ! self.isUpdaterRunning.CAS(false, true) {
//     		return
//     	}
//     	self.stopChan = make(mpsc::channel());
//     	self.stopFinishedChan = make(mpsc::channel());
//     	go  self.renderLoopFn()
//     }

//     pub fn stopRenderLoop() {
//     	if ! self.isUpdaterRunning.CAS(true, false) {
//     		return
//     	}
//     	 self.stopChan <- struct{}{};
//     	close( self.stopChan);
//     	 self.stopChan = nil;

//     	<- self.stopFinishedChan;
//     	close( self.stopFinishedChan);
//     	 self.stopFinishedChan = nil
//     }

//     pub fn renderLoopFn() {
//     	let ticker = time.NewTicker(refreshRate);

//     	for {
//     		select {
//     		case <-ticker.C:
//     			 self.renderFn()
//     		case <- self.stopChan:
//     			 self.renderFn()
//     			 self.stopFinishedChan <- struct{}{}
//     			return
//     		}
//     	}
//     }

// }
