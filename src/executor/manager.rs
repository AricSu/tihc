// fn test_single_bar() {
//     let format = "╢▌▌░╟".to_string();
//     let header_str = "Application Test header :".to_string();
//     let finish_str = "Done -- Single Bar -- TiHC ".to_string();
//     single_bar(header_str, true, format, finish_str, 100, 50);
// }

#[derive(Debug)]
pub struct BarSystemInfoManager {
    bar_name: String,
    item_count: u64,
    item_progress: u64,
}

impl BarSystemInfoManager {
    pub fn new(bar_name: String, item_count: u64, item_progress: u64) -> Self {
        return BarSystemInfoManager {
            bar_name: bar_name,
            item_count: item_count,
            item_progress: item_progress,
        };
    }

    pub fn set_progress(&mut self, new_progress: u64) {
        self.item_progress = new_progress;
    }
}

#[test]
fn test_set_progress() {
    let mut new_pro = BarSystemInfoManager::new("bar_name".to_string(), 10, 0);
    new_pro.set_progress(3);
    println!("{:?}", new_pro);
    assert_eq!(new_pro.item_progress, 3)
}
