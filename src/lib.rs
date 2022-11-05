#![no_std]

pub fn sum_accumulate(x: i32) -> i32 {
    x + unsafe { accumulate() }
}

extern "C" {
    fn accumulate() -> i32;
}
