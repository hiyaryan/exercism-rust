use enum_iterator::{all, Sequence};
use int_enum::IntEnum;
use std::fmt::Debug;

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, IntEnum, Sequence, PartialOrd, Ord)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    if let Some(color) = ResistorColor::from_int(value).ok() {
        // Use the enum's `Debug` output, which is the variant's name.
        // This is a bit "brittle" in that we need to mind the enum's
        // actual identifier, and special cases could become more difficult
        // to untangle (what if we wanted to print "Gray" vs "Grey" based on locale?)
        // but it allows us to keep the "source of truth" clear (the enum declaration)
        // avoids repeatition
        format!("{color:?}")
    } else {
        "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut v: Vec<ResistorColor> = all::<ResistorColor>().collect();
    v.sort();
    v
}
