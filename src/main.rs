use std::{thread, time};
use winrt::ro_initialize;

mod winrt;

fn main() {
    let duration = time::Duration::from_secs(10);
    thread::sleep(duration);

    ro_initialize();
}
