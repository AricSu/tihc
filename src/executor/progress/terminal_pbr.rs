use pbr::ProgressBar;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::{thread, time::Duration};

#[derive(Debug)]
pub struct Bar {
    header: String,
    format: String,
    is_format: bool,
    finish: String,
    progress_count: u64,
}

impl Bar {
    pub fn new(
        header: String,
        format: String,
        is_format: bool,
        finish: String,
        progress_count: u64,
    ) -> Self {
        return {
            Bar {
                header: header,
                is_format: is_format,
                format: format,
                finish: finish,
                progress_count: progress_count,
            }
        };
    }

    pub fn single_bar(&mut self, channel_recv: Receiver<u64>) {
        let mut pb = ProgressBar::new(self.progress_count.clone());
        pb.format(&self.format);
<<<<<<< HEAD
        // println!("{}", self.header);
=======
        println!("{}", self.header);
>>>>>>> f135a7322fc9ee67432e7a272fc336f00f6c4e1b

        let mut old_left_task = self.progress_count - 1;

        loop {
            {
                if let Result::Ok(received) = channel_recv.try_recv() {
                    let new_left_task = self.progress_count - received;
                    let mut inc_progress = old_left_task - new_left_task;
                    old_left_task = new_left_task;
                    if self.progress_count == received {
                        pb.finish_println(&self.finish);
                        break;
                    } else if inc_progress > 0 {
                        for _ in 0..inc_progress {
                            pb.inc();
                        }
                        inc_progress = 0;
                    }
                }
            }
        }
    }
}

#[test]
fn test_single_bar() {
    let format = "╢▌▌░╟".to_string();
    let header_str = "Application Test header :".to_string();
    let finish_str = "Done -- Single Bar -- TiHC ".to_string();
    let mut bar = Bar::new(header_str, format, true, finish_str, 100);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        bar.single_bar(rx);
    });
    tx.send(3).unwrap();
    tx.send(6).unwrap();
    tx.send(9).unwrap();
    tx.send(50).unwrap();
    thread::sleep(Duration::from_millis(5000));
    tx.send(100).unwrap();
}

pub fn move_cursor_to_next_line() {
    println!("\r");
}
