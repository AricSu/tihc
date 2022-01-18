#[macro_use]
extern crate clap;
extern crate colored;
extern crate docx_rs;
extern crate pbr;
extern crate yaml_rust;
mod cmd;
mod components;
mod executor;
mod util;
use cmd::load::cli_build;

fn main() {
    let _ = cli_build();
}
