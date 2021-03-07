

/* Get time remaining in a match as seconds */
pub fn get_remaining_time_as_seconds() -> u32 {
    unsafe { smash_utils::externs::get_remaining_time_as_frame() / 60 }
}

/* 
Initial time should be set to get_remaining_time_as_seconds when you want to "start" the timer, duration obv amount of time... 
this will return true once 'duration' seconds have elapsed since 'intial_time', false otherwise 
*/
pub unsafe fn is_time_range(initial_time: u32, duration: u32) -> bool {
    if initial_time - get_remaining_time_as_seconds() >= duration {
        return true;
    }
    false
}


/* String stuff */

/* Using this stuff cus native rust string slices are weird and crash sometimes */

// https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351/10
use std::ops::{Bound, RangeBounds};

pub trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}