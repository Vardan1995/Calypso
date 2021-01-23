mod common;
mod public;
mod windows;
use crate::public::KeybdKey::*;
use crate::windows::*;
use std::fs;
use std::{thread::sleep, time::Duration};

fn main() {
    listener()
}

// ---------------------------------------------------------------------------
fn listener() {
    let content = fs::read_to_string("combo.txt").expect("Something went wrong reading the file");
    let numb_arr_from_srt: Vec<usize> = content
        .split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    SpaceKey.bind(move || generated_cb_function(&numb_arr_from_srt));
    handle_input_events();
}

fn generated_cb_function(nums: &Vec<usize>) {
    // println!("{:?}", a);
    let arr = [
        LShiftKey, QKey, WKey, EKey, RKey, TKey, YKey, UKey, IKey, OKey, PKey,
    ];
    for (i, num) in nums.iter().enumerate() {
        if nums[i] < arr.len() {
            let key = arr[*num as usize];
            key.press();
            sleep(Duration::from_millis(10));
            key.release();
        }
    }
}

// ----------------------------------------------------------------------------
