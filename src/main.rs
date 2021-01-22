mod common;
mod public;
mod windows;

use crate::public::KeybdKey::*;
use crate::windows::*;
use std::{thread::sleep, time::Duration};

fn main() {
    listener()
}

// ---------------------------------------------------------------------------
fn listener() {
    SpaceKey.bind(|| generated_cb_function(vec![1, 1, 1, 0, 2, 3, 4]));
    handle_input_events();
}

fn generated_cb_function(nums: Vec<usize>) {
    let arr = [QKey, WKey, EKey, RKey, IKey];
    for (i, num) in nums.iter().enumerate() {
        if nums[i] < arr.len() {
            let key = arr[*num as usize];
            key.press();
            sleep(Duration::from_millis(10));
            key.release();
        }
    }
    // }
}
// ----------------------------------------------------------------------------
