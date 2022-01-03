use std::fs::create_dir_all;
use std::path::Path;

pub fn create_dir(path: String) {
    let path = Path::new(&path);
    match create_dir_all(path) {
        Ok(_f) => {
            println!("created folder")
        }
        Err(err) => {
            println!("{:?}", err);
        }
    };
}

// pub fn remove_dir(path: String) {
//     let path = Path::new(&path);
//     match create_dir_all(path) {
//         Ok(_f) => {
//             println!("created folder")
//         }
//         Err(err) => {
//             println!("{:?}", err);
//         }
//     };
// }
