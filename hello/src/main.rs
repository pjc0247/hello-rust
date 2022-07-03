use std::fmt;
use strum_macros;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(strum_macros::Display)]
enum ValueKind {
    Object,
    Array,
    Null,
    Undefined,
    Number,
    String
}

enum ParseState {
    Empty,

    Object,
    ObjectKey,
    ObjectValue,

    Array,

    Value
}

trait JValueImpl {

    fn dump(&self) {}
}

struct JObject {
    
}

struct JArray {

}

struct JValue {
    kind: ValueKind
}

impl JValueImpl for JObject {
    fn dump(&self) {
        println!("JObject")
    }
}

impl JValueImpl for JArray {
    fn dump(&self) {
        println!("JArray")
    }
}

impl JValueImpl for JValue {
    fn dump(&self) {
        println!("JValue {}", self.kind);
    }
}



fn peek_type (input: &str, offset: usize) -> ValueKind {
    match input.chars().nth(offset).unwrap() {
        '{' => { return ValueKind::Object }
        '[' => { return ValueKind::Array; }
        'n' => { 
            match &input[offset..4] {
                "null" => { return ValueKind::Null; }
                &_ => { }
            }
        }
        'u' => { 
            match &input[offset..9] {
                "undefined" => { return ValueKind::Undefined; }
                &_ => { }
            }
        }
        _ => { println!("where am i? ") }
    }

    panic!("Unexpected token")
}

fn value_kind_to_parse_state (next_value_type: ValueKind) -> ParseState {
    match next_value_type {
        ValueKind::Array => { return ParseState::Array }
        ValueKind::Object => { return ParseState::Object }
        ValueKind::Null
            | ValueKind::Undefined
            | ValueKind::Number
            | ValueKind::String => { return ParseState::Value }
    }
}

fn parse (input: &str, offset: usize) -> Box<dyn JValueImpl> {
    let root = JObject { };
    let mut state = ParseState::Empty;

    for (i, c) in input.chars().enumerate() {
        match state {
            ParseState::Empty => {
                let t = peek_type(input, i);
                state = value_kind_to_parse_state(t);
            }
            ParseState::Object => {
                
            }
            ParseState::ObjectKey => todo!(),
            ParseState::ObjectValue => todo!(),
            ParseState::Array => todo!(),
            ParseState::Value => {
                print!("null")
            },
        }
    }

    Box::new(root)
}

fn main() {
//    let r = parse("{ a: 134 }", 0);
    let r = parse("null", 0);

    r.dump();

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek_type() {
        assert!(peek_type("null", 0) == ValueKind::Null);
        assert!(peek_type("undefined", 0) == ValueKind::Undefined);
        assert!(peek_type("{", 0) == ValueKind::Object);
        assert!(peek_type("[", 0) == ValueKind::Array);
    }

    #[test]
    #[should_panic]
    fn test_invalid_peek_type() {
        peek_type("noll", 0);
    }
}
