mod common;

mod public;
// use crate::public::*;

mod windows;

use crate::windows::*;

use crate::public::KeybdKey::*;
use std::{thread::sleep, time::Duration};

fn main() {
    listener()
}

fn listener() {
    SpaceKey.bind(|| {
        QKey.press();
        sleep(Duration::from_millis(10));
        QKey.release();
        WKey.press();
        sleep(Duration::from_millis(10));
        WKey.release();
        EKey.press();
        sleep(Duration::from_millis(10));
        EKey.release();
        RKey.press();
        sleep(Duration::from_millis(10));
        RKey.release();
    });

    // Call this to start listening for bound inputs.
    handle_input_events();
}
