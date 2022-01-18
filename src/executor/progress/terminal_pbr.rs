use pbr::{MultiBar, ProgressBar};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

pub struct Bar {
    header: String,
    format: String,
    is_format: bool,
    finish: String,
    progress_count: u64,
    progress_rate: Duration,
}

impl Bar {
    pub fn new(
        header: String,
        format: String,
        is_format: bool,
        finish: String,
        progress_count: u64,
        progress_rate: u64,
    ) -> Self {
        let rate = Duration::from_millis(progress_rate);
        return {
            Bar {
                header: header,
                is_format: is_format,
                format: format,
                finish: finish,
                progress_count: progress_count,
                progress_rate: rate,
            }
        };
    }
}

pub fn single_bar(
    header_str: String,
    is_format: bool,
    format_str: String,
    finish_str: String,
    progress_count: u64,
    progress_rate: u64,
) {
    let single_bar = Bar::new(
        header_str,
        format_str,
        is_format,
        finish_str,
        progress_count,
        progress_rate,
    );
    let mut pb = ProgressBar::new(progress_count.clone());
    pb.format(&single_bar.format);
    for _ in 0..single_bar.progress_count {
        thread::sleep(Duration::from_millis(50));
        pb.inc();
    }
    pb.finish_println(&single_bar.finish);
}

pub fn multi_bar(bar_vec: Vec<Bar>) {
    let complete = Arc::new(AtomicBool::new(false));
    let progress = Arc::new(MultiBar::new());

    thread::spawn({
        let complete = Arc::clone(&complete);
        let progress = Arc::clone(&progress);
        move || {
            for task in bar_vec {
                thread::spawn({
                    let progress = Arc::clone(&progress);
                    move || {
                        let mut bar = progress.create_bar(task.progress_count);
                        bar.message(&format!("Task : {} ", task.header));

                        for _ in 0..100 {
                            thread::sleep(Duration::from_millis(50));
                            bar.inc();
                        }

                        bar.finish_print(&format!("Task {} Complete", task.finish));
                    }
                });

                thread::sleep(Duration::from_millis(1000));
            }

            complete.store(true, Ordering::SeqCst);
        }
    });

    while !complete.load(Ordering::SeqCst) {
        let _ = progress.listen();
        thread::sleep(Duration::from_millis(1000));
    }

    let _ = progress.listen();
}

#[test]
fn test_single_bar() {
    let format = "╢▌▌░╟".to_string();
    let header_str = "Application Test header :".to_string();
    let finish_str = "Done -- Single Bar -- TiHC ".to_string();
    single_bar(header_str, true, format, finish_str, 100, 50);
}

#[test]
fn test_multi_bar() {
    let format = "╢▌▌░╟".to_string();
    let header_str = "Application Test header :".to_string();
    let finish_str = "Done -- Multi Bar -- TiHC ".to_string();
    let bar1 = Bar::new(
        header_str.clone(),
        format.clone(),
        true,
        finish_str.clone(),
        100,
        50,
    );
    let bar2 = Bar::new(header_str.clone(), format, true, finish_str, 200, 50);
    let bar_bucket = vec![bar1, bar2];
    multi_bar(bar_bucket);
}

pub fn moveCursorUp(n: usize) {
    println!("\x1b[{}A", n);
}

pub fn moveCursorDown(n: usize) {
    println!("\x1b[{}B", n)
}

pub fn move_cursor_to_line_start() {
    println!("\r");
}
