#![allow(dead_code)]
mod common;
mod public;
mod windows;
// p=q,w,e,r
// p=q+w
use crate::public::KeybdKey::*;
use crate::windows::*;
use std::{thread::sleep, time::Duration};

// fn main() {
//     let mut comb = String::new();
//     print!("Please enter combo: ");
//     let _ = stdout().flush();
//     stdin()
//         .read_line(&mut comb)
//         .expect("Did not enter a wrong combination");
// let c: Vec<&str> = comb.split('-').collect();

// let event_key = c[0];
// let press_keys: Vec<&str> = c[1].split(',').collect();
// println!("You typed: {:?}", press_keys);
// }
// use std::env;
use std::fs;

fn main() {
    let content = fs::read_to_string("combo.txt").expect("Something went wrong reading the file");
    let c: Vec<&str> = content.split('>').collect();

    let event_key = c[0];
    // let press_keys: Vec<&str> = c[1].split(',').collect();
    println!("event: {}", event_key);

    listener(vec![1])
}
// ---------------------------------------------------------------------------
fn listener(evks: Vec<usize>) {
    // let all_keys = [ZKey];
    // for (i, evk) in evks.iter().enumerate() {
    //     println!("evk {}", evk);
    //     if evks[i] < all_keys.len() {
    //         let key = all_keys[*evk as usize];
    //         //
    //         println!("binded ---- {:?}", key);
    //     }
    // }
    SpaceKey.bind(|| generated_cb_function());
    handle_input_events();
}

fn generated_cb_function() {
    println!("evk ");
    let all_keys = [
        TabKey,
        EnterKey,
        EscapeKey,
        SpaceKey,
        HomeKey,
        LeftKey,
        UpKey,
        RightKey,
        DownKey,
        InsertKey,
        DeleteKey,
        Numrow0Key,
        Numrow1Key,
        Numrow2Key,
        Numrow3Key,
        Numrow4Key,
        Numrow5Key,
        Numrow6Key,
        Numrow7Key,
        Numrow8Key,
        Numrow9Key,
        AKey,
        BKey,
        CKey,
        DKey,
        EKey,
        FKey,
        GKey,
        HKey,
        IKey,
        JKey,
        KKey,
        LKey,
        MKey,
        NKey,
        OKey,
        PKey,
        QKey,
        RKey,
        SKey,
        TKey,
        UKey,
        VKey,
        WKey,
        XKey,
        YKey,
        ZKey,
        Numpad0Key,
        Numpad1Key,
        Numpad2Key,
        Numpad3Key,
        Numpad4Key,
        Numpad5Key,
        Numpad6Key,
        Numpad7Key,
        Numpad8Key,
        Numpad9Key,
        F1Key,
        F2Key,
        F3Key,
        F4Key,
        F5Key,
        F6Key,
        F7Key,
        F8Key,
        F9Key,
        F10Key,
        F11Key,
        F12Key,
        NumLockKey,
        ScrollLockKey,
        CapsLockKey,
        LShiftKey,
        RShiftKey,
        LControlKey,
        RControlKey,
    ];

    let content = fs::read_to_string("combo.txt").expect("Something went wrong reading the file");
    let c: Vec<&str> = content.split('>').collect();
    println!("evk {}", content);
    let press_keys: Vec<&str> = c[1].split(',').collect();

    let nums = match_key_to_numbers(press_keys);
    for (i, num) in nums.iter().enumerate() {
        if nums[i] < all_keys.len() {
            let key = all_keys[*num as usize];
            key.press();
            sleep(Duration::from_millis(10));
            key.release();
        }
    }
}
// ----------------------------------------------------------------------------
// let numb_arr_from_srt: Vec<usize> = "1,5,8"
// .split(",")
// .map(|s| s.trim())
// .filter(|s| !s.is_empty())
// .map(|s| s.parse().unwrap())
// .collect();

fn match_key_to_numbers(keys: Vec<&str>) -> Vec<usize> {
    let mut num_vec = Vec::new();
    for key in keys {
        let n = match key {
            "Tab" => 0,
            "Enter" => 1,
            "Escape" => 2,
            "Space" => 3,
            "Home" => 4,
            "Left" => 5,
            "Up" => 6,
            "Right" => 7,
            "Down" => 8,
            "Insert" => 9,
            "Delete" => 10,
            "Numrow0" => 11,
            "Numrow1" => 12,
            "Numrow2" => 13,
            "Numrow3" => 14,
            "Numrow4" => 15,
            "Numrow5" => 16,
            "Numrow6" => 17,
            "Numrow7" => 18,
            "Numrow8" => 19,
            "Numrow9" => 20,
            "A" => 21,
            "B" => 22,
            "C" => 23,
            "D" => 24,
            "E" => 25,
            "F" => 26,
            "G" => 27,
            "H" => 28,
            "I" => 29,
            "J" => 30,
            "K" => 31,
            "L" => 32,
            "M" => 33,
            "N" => 34,
            "O" => 35,
            "P" => 36,
            "Q" => 37,
            "R" => 38,
            "S" => 40,
            "T" => 41,
            "U" => 43,
            "V" => 44,
            "W" => 47,
            "X" => 46,
            "Y" => 477,
            "Z" => 48,
            "Numpad0" => 47,
            "Numpad1" => 48,
            "Numpad2" => 49,
            "Numpad3" => 50,
            "Numpad4" => 51,
            "Numpad5" => 52,
            "Numpad6" => 53,
            "Numpad7" => 54,
            "Numpad8" => 55,
            "Numpad9" => 56,
            "F1" => 57,
            "F2" => 58,
            "F3" => 59,
            "F4" => 60,
            "F5" => 61,
            "F6" => 62,
            "F7" => 63,
            "F8" => 64,
            "F9" => 65,
            "F10" => 66,
            "F11" => 67,
            "F12" => 68,
            "NumLock" => 69,
            "ScrollLock" => 70,
            "CapsLock" => 71,
            "LShift" => 72,
            "RShift" => 73,
            "LControl" => 74,
            "RControl" => 75,
            _ => 0,
        };

        num_vec.push(n)
    }
    num_vec
}
