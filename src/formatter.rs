use lazy_static::lazy_static;
use regex::{Captures, Regex};
use strfmt::{strfmt_map, Formatter, Result};

use std::collections::HashMap;

lazy_static! {
    static ref LITERAL_REGEX: Regex = Regex::new(
        r"(?x)
        %
        (?:\((?P<key>\w+)\))?         # Mapping key
        (?P<flags>[\#0\- +]*)?        # Conversion flags
        (?P<width>\*|\d+)?            # Minimum field width
        (?:.(?P<precision>\*|\d+))?   # Precision after decimal point
        [hlL]*                        # Ignored length modifier
        (?P<type>[diouxXeEfFgGcrs%])  # Conversion type
    "
    )
    .unwrap();
}

#[allow(dead_code)]
pub enum FormattingArgument {
    String(String),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    Usize(usize),
    Isize(isize),
    F32(f32),
    F64(f64),
}

fn rewrite_formatting(input: &str) -> String {
    let mut counter = -1;
    let result = LITERAL_REGEX.replace_all(input, |caps: &Captures| {
        counter += 1;
        // Funcom only uses flags, width, precision and type
        let flags = caps.name("flags").unwrap().as_str();
        let width: Option<usize> = caps.name("width").map(|i| i.as_str().parse().unwrap());
        let precision: Option<usize> = caps.name("precision").map(|i| i.as_str().parse().unwrap());
        let r#type = caps.name("type").unwrap().as_str();

        // Simple case: it's escaped
        if r#type == "%" {
            return String::from("%");
        }
        if width.is_none() && precision.is_none() {
            return format!("{{{}}}", counter);
        } else if width.is_some() && precision.is_none() {
            return format!("{{{}:{}{}}}", counter, flags, width.unwrap());
        } else if width.is_none() && precision.is_some() {
            return format!("{{{}:.{}}}", counter, precision.unwrap());
        } else {
            return format!(
                "{{{}:{}{}.{}}}",
                counter,
                flags,
                width.unwrap(),
                precision.unwrap()
            );
        }
    });

    result.to_string()
}

pub fn format_string(input: &str, arguments: Vec<FormattingArgument>) -> Result<String> {
    let string_to_format = rewrite_formatting(input);
    let mut params = HashMap::with_capacity(arguments.len());
    for (i, argument) in arguments.iter().enumerate() {
        params.insert(i.to_string(), argument);
    }

    let f = |mut fmt: Formatter| {
        let param = params.get(fmt.key).unwrap();
        match param {
            FormattingArgument::String(s) => fmt.str(s),
            FormattingArgument::U8(u) => fmt.u8(*u),
            FormattingArgument::I8(i) => fmt.i8(*i),
            FormattingArgument::U16(u) => fmt.u16(*u),
            FormattingArgument::I16(i) => fmt.i16(*i),
            FormattingArgument::U32(u) => fmt.u32(*u),
            FormattingArgument::I32(i) => fmt.i32(*i),
            FormattingArgument::U64(u) => fmt.u64(*u),
            FormattingArgument::I64(i) => fmt.i64(*i),
            FormattingArgument::Usize(u) => fmt.usize(*u),
            FormattingArgument::Isize(i) => fmt.isize(*i),
            FormattingArgument::F32(f) => fmt.f32(*f),
            FormattingArgument::F64(f) => fmt.f64(*f),
        }
    };

    strfmt_map(&string_to_format, &f)
}

#[test]
fn find_literals() {
    let text =
        "Team-mission chance of token reward upped to %0.0f%% due to the team's heroic effort.";
    let value = FormattingArgument::F64(52.55);
    let params = vec![value];
    println!("{}", format_string(text, params).unwrap());
    let text = "I am %s";
    let value = FormattingArgument::String(String::from("Yakachi"));
    let params = vec![value];
    println!("{}", format_string(text, params).unwrap());
    // This is currently unsupported by strfmt
    let text = "It's %02d:%02d:%02d";
    let params = vec![
        FormattingArgument::U32(5),
        FormattingArgument::U32(8),
        FormattingArgument::U32(19),
    ];
    println!("{:?}", format_string(text, params).ok());
    let text = "Dependencies: Sense 60%, Intelligence 20%, Agility 20%";
    let params = Vec::new();
    println!("{}", format_string(text, params).unwrap());
    let text = "Let's go for %d%%";
    let params = vec![FormattingArgument::U32(15)];
    println!("{}", format_string(text, params).unwrap());
    let text = "This is %05.02f%%";
    let params = vec![FormattingArgument::F64(32.35515)];
    println!("{}", format_string(text, params).unwrap());
}
