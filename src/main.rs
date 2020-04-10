extern crate log;
extern crate simple_logger;

extern crate rustyreader_lib as lib;

use lib::run;

fn main() {
    simple_logger::init().unwrap();
    log::set_max_level(log::LevelFilter::Info);
    run();
}
