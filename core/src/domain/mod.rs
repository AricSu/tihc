pub mod service;

pub struct ExampleService;

impl ExampleService {
    pub fn do_something(&self) {
        println!("Application service doing something");
    }
}
