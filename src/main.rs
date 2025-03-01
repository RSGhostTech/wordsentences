use crate::init::{check_eula, init_config};

mod web;
mod utils;
mod init;

fn main() {
    let config = init_config();
    check_eula(&config);
}
