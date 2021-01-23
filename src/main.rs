mod common;
mod public;
mod windows;
use crate::public::KeybdKey::*;
use crate::windows::*;
use std::fs;
// use std::{thread::sleep, time::Duration};

fn main() {
    listener()
}

// ---------------------------------------------------------------------------
fn listener() {
    let content = fs::read_to_string("combo.txt").expect("Something went wrong reading the file");

    let content_lines: Vec<&str> = content.lines().collect();
    for line in content_lines {
        let two_keys: Vec<&str> = line.split("-").collect();
        let action_keys: Vec<&str> = two_keys[1].split(",").collect();
        let numb_arr_from_press_key = match_key_to_numbers(vec![two_keys[0]]);
        let os_keys_arr = os_keys();
        let listener_key = os_keys_arr[numb_arr_from_press_key[0] as usize];
        println!(
            "
        ------------------------------------------------------
          {:?} = {:?}
        ------------------------------------------------------
        ",
            listener_key, action_keys
        );
        let numb_arr_from_action_key_str = match_key_to_numbers(action_keys);
        listener_key.bind(move || generated_cb_function(&numb_arr_from_action_key_str));
    }
    handle_input_events();
}

fn generated_cb_function(nums: &Vec<usize>) {
    let arr = os_keys();
    for (i, num) in nums.iter().enumerate() {
        if nums[i] < arr.len() {
            let key = arr[*num as usize];
            key.press();
        }
    }

    for (i, num) in nums.iter().enumerate() {
        if nums[i] < arr.len() {
            let key = arr[*num as usize];
            key.release();
        }
    }
}

// ----------------------------------------------------------------------------
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
            "S" => 39,
            "T" => 40,
            "U" => 41,
            "V" => 42,
            "W" => 43,
            "X" => 44,
            "Y" => 45,
            "Z" => 46,
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
            _ => 21,
        };

        num_vec.push(n)
    }
    num_vec
}

fn os_keys() -> [public::KeybdKey; 76] {
    let arr = [
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
    arr
}
