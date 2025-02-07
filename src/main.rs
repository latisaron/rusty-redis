use std::any::type_name;

pub struct Serializer {}

#[derive(PartialEq, Debug)]
enum Value {
    Null,
    NullArray,
    String(String),
    Error(String),
    Integer(i32),
    Bulk(String),
    Array(Vec<Value>),
}

impl Serializer {
    fn new() -> Self {
        Serializer { }
    }

    fn serialize(&self, basic_string: &str) -> Value {
        if basic_string == "$-1\r\n" {
            return Value::Null;
        } else if basic_string == "*-1\r\n" {
            return Value::NullArray;
        } else {
            let mut split_iter = basic_string.split("\r\n");
            let first_part = split_iter.next().unwrap();

            let (_, first_char) = first_part.chars().enumerate().next().unwrap();

            match first_char {
                '+' => {
                    return Value::String(first_part[1..].to_string());
                },
                '-' => {
                    return Value::Error(first_part[1..].to_string());
                },
                ':' => {
                    return Value::Integer(first_part[1..].parse::<i32>().unwrap());
                },
                '$' => {
                    let actual_string = split_iter.next().unwrap();

                    return Value::Bulk(actual_string.to_string());
                },
                '*' => {
                    let number_of_elems = first_part[1..].parse::<i32>().unwrap();
                    
                    todo!()
                },
                _ => {
                    return Value::Null;
                },
            }   
        }
    }

    fn deserialize(&self) {

    }
}

fn main() {
    let a = "aron\r\nfrom\r\ncarton\r\n";
    let mut spititer = a.split("\r\n");
    println!("val is {:?}", spititer.take(2).collect::<String>());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    #[test]
    fn null_value() {
        let serializer = Serializer::new();
        let string1 = String::from("$-1\r\n");
        assert_eq!(serializer.serialize(&string1), Value::Null);
    }

    #[test]
    fn one_elem_ary() {
        let serializer = Serializer::new();
        let string2 = String::from("*1\r\n$4\r\nping\r\n");
        if let Value::Array(tmp_ary) = serializer.serialize(&string2) {
            assert_eq!(tmp_ary[0], Value::Bulk(String::from("ping")));
        }
    }

    #[test]
    fn echo_ary() {
        let serializer = Serializer::new();
        let string3 = String::from("*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n");
        if let Value::Array(tmp_ary) = serializer.serialize(&string3) {
            assert_eq!(tmp_ary[0], Value::Bulk(String::from("hello world")));
        }
    }

    #[test]
    fn get_key_ary() {
        let serializer = Serializer::new();
        let string4 = String::from("*2\r\n$3\r\nget\r\n$3\r\nkey\r\n");
        if let Value::Array(tmp_ary) = serializer.serialize(&string4) {
            assert_eq!(tmp_ary[0], Value::Bulk(String::from("get")));
            assert_eq!(tmp_ary[1], Value::Bulk(String::from("key")));

        }
    }

    #[test]
    fn simple_string() {
        let serializer = Serializer::new();
        let string5 = String::from("+OK\r\n");
        assert_eq!(serializer.serialize(&string5), Value::String(String::from("OK")));
    }

    #[test]
    fn error() {
        let serializer = Serializer::new();
        let string6 = String::from("-Error message\r\n");
        assert_eq!(serializer.serialize(&string6), Value::Error(String::from("Error message")));
    }

    #[test]
    fn empty_bulk() {
        let serializer = Serializer::new();
        let string7 = String::from("$0\r\n\r\n");
        assert_eq!(serializer.serialize(&string7), Value::Bulk(String::from("")));
    }

    #[test]
    fn simple_string_again() {
        let serializer = Serializer::new();
        let string8 = String::from("+hello world\r\n");
        assert_eq!(serializer.serialize(&string8), Value::String(String::from("hello world")));
    }
}