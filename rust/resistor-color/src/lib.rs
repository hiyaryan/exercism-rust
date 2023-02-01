use std::fmt::Debug;
use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

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

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    if value == ResistorColor::Black.int_value() {
        "Black".to_string()
    } else if value == ResistorColor::Blue.int_value() {
        "Blue".to_string()
    } else if value == ResistorColor::Brown.int_value() {
        "Brown".to_string()
    } else if value == ResistorColor::Green.int_value() {
        "Green".to_string()
    } else if value == ResistorColor::Grey.int_value() {
        "Grey".to_string()
    } else if value == ResistorColor::Orange.int_value() {
        "Orange".to_string()
    } else if value == ResistorColor::Red.int_value() {
        "Red".to_string()
    } else if value == ResistorColor::Violet.int_value() {
        "Violet".to_string()
    } else if value == ResistorColor::White.int_value() {
        "White".to_string()
    } else if value == ResistorColor::Yellow.int_value() {
        "Yellow".to_string()
    } else {
        "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut v: Vec<ResistorColor> = all::<ResistorColor>().collect();
    v.sort();
    v
}
