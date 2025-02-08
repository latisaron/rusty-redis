use std::any::type_name;
pub struct Serializer {}



#[derive(PartialEq, Debug)]
pub enum Value {
    Null,
    NullArray,
    String(String),
    Error(String),
    Integer(i32),
    Bulk(String),
    Array(Vec<Value>),
}

impl Serializer {
    pub fn new() -> Self {
        Serializer { }
    }

    fn internal_serialize(&self, value: &Value, tmp_string: &mut String) {
        match value {
            Value::Null => {
                tmp_string.push_str("$-1\r\n");
            },
            Value::NullArray => {
                tmp_string.push_str("*-1\r\n");
            },
            Value::String(internal) => {
                tmp_string.push_str(&format!("+{}\r\n", internal));
            }
            Value::Error(internal) => {
                tmp_string.push_str(&format!("-{}\r\n", internal));
            },
            Value::Integer(internal) => {
                tmp_string.push_str(&format!(":{}\r\n", internal));
            },
            Value::Bulk(internal) => {
                tmp_string.push_str(&format!("${}\r\n{}\r\n", internal.len(), internal));
            },
            Value::Array(internal) => {
                tmp_string.push_str(&format!("*{}\r\n", internal.len()));
                for item in internal {
                    self.internal_serialize(item, tmp_string);
                }
                // tmp_string.push_str("\r\n");
            },
            _ => {}
        }
    }

    pub fn serialize(&self, value: &Value) -> String {
        let mut tmp_string = String::new();
        self.internal_serialize(value, &mut tmp_string);
        return tmp_string;
    }

    pub fn deserialize(&self, basic_string: &str) -> Value {
        let mut current_index = 0;
        return self.internal_deserialize(basic_string, &mut current_index).0;
    }

    fn internal_deserialize(&self, incoming_str: &str, current_index: &mut usize) -> (Value, usize) {
        let basic_string = &incoming_str[*current_index..];
        let mut char_iter = basic_string.chars().enumerate();
        if let Some((_, first_char)) = char_iter.next() {
            match first_char {
                '+' => {
                    let new_string = basic_string[1..(basic_string.len()-2)].to_string();
                    let str_len = new_string.len();
                    return (Value::String(new_string), str_len);
                },
                '-' => {
                    let new_string = basic_string[1..(basic_string.len()-2)].to_string();
                    let str_len = new_string.len();
                    return (Value::Error(new_string), str_len);
                },
                ':' => {
                    let new_string = basic_string[1..(basic_string.len()-2)].to_string();
                    let str_len = new_string.len();
                    return (Value::Integer(new_string.parse::<i32>().unwrap()), str_len);
                },
                '$' => {
                    let mut split_iter = basic_string.split("\r\n");
                    let first_elem = split_iter.next().unwrap();
                    
                    if first_elem[1..].parse::<i32>().unwrap() == -1 {
                        return (Value::Null, 5);
                    } else {
                        let actual_string = split_iter.next().unwrap();

                        return (Value::Bulk(actual_string.to_string()), first_elem.len() + actual_string.len());
                    }
                },
                '*' => {
                    let mut split_iter = basic_string.split("\r\n");
                    let first_elem = split_iter.next().unwrap();
                    let number_of_elems = first_elem[1..].parse::<i32>().unwrap();

                    if number_of_elems == -1 {
                        return (Value::NullArray, 5);
                    } else {
                        let mut tmp_vec = Vec::<Value>::new();
                        for _ in 0..number_of_elems {
                            *current_index = *current_index + first_elem.len() + 2;
                            let result = self.internal_deserialize(incoming_str, current_index);
                            *current_index = *current_index + result.1;
                            tmp_vec.push(
                                result.0
                            )
                        }
                        return (Value::Array(tmp_vec), *current_index);
                    }
                },
                _ => {
                    return (Value::Null, 0);
                },
            }   
        }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_value() {
        let serializer = Serializer::new();
        let string1 = String::from("$-1\r\n");
        let value = serializer.deserialize(&string1);
        assert_eq!(value, Value::Null);
        assert_eq!(serializer.serialize(&value), string1);
    }

    #[test]
    fn one_elem_ary() {
        let serializer = Serializer::new();
        let string2 = String::from("*1\r\n$4\r\nping\r\n");
        let value = serializer.deserialize(&string2);
        if let Value::Array(tmp_ary) = &value {
            assert_eq!(tmp_ary[0], Value::Bulk(String::from("ping")));
        }
        assert_eq!(serializer.serialize(&value), string2);
    }

    #[test]
    fn echo_ary() {
        let serializer = Serializer::new();
        let string3 = String::from("*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n");
        let value = serializer.deserialize(&string3);
        if let Value::Array(tmp_ary) = &value {
            assert_eq!(tmp_ary[0], Value::Bulk(String::from("echo")));
            assert_eq!(tmp_ary[1], Value::Bulk(String::from("hello world")));
        }
        assert_eq!(serializer.serialize(&value), string3);
    }

    #[test]
    fn get_key_ary() {
        let serializer = Serializer::new();
        let string4 = String::from("*2\r\n$3\r\nget\r\n$3\r\nkey\r\n");
        let value = serializer.deserialize(&string4);
        if let Value::Array(tmp_ary) = &value {
            assert_eq!(tmp_ary[0], Value::Bulk(String::from("get")));
            assert_eq!(tmp_ary[1], Value::Bulk(String::from("key")));
        }
        assert_eq!(serializer.serialize(&value), string4);
    }

    #[test]
    fn ary_of_arys() {
        let serializer = Serializer::new();
        let string4 = String::from("*3\r\n$3\r\nget\r\n$3\r\nkey\r\n*1\r\n+3\r\n");
        let value = serializer.deserialize(&string4);
        if let Value::Array(tmp_ary) = &value {
            println!("tmp ary is {:?}", tmp_ary);
            assert_eq!(tmp_ary[0], Value::Bulk(String::from("get")));
            assert_eq!(tmp_ary[1], Value::Bulk(String::from("key")));
            if let Value::Array(tmp_ary_2) = &tmp_ary[2] {
                assert_eq!(tmp_ary_2[0], Value::String(String::from("3")));
            }
        }
        assert_eq!(serializer.serialize(&value), string4);
    }

    #[test]
    fn simple_string() {
        let serializer = Serializer::new();
        let string5 = String::from("+OK\r\n");
        let value = Value::String(String::from("OK"));
        assert_eq!(serializer.deserialize(&string5), value);
        assert_eq!(serializer.serialize(&value), string5);
    }

    #[test]
    fn error() {
        let serializer = Serializer::new();
        let string6 = String::from("-Error message\r\n");
        let value = Value::Error(String::from("Error message"));
        assert_eq!(serializer.deserialize(&string6), value);
        assert_eq!(serializer.serialize(&value), string6);
    }

    #[test]
    fn empty_bulk() {
        let serializer = Serializer::new();
        let string7 = String::from("$0\r\n\r\n");
        let value = Value::Bulk(String::from(""));
        assert_eq!(serializer.deserialize(&string7), value);
        assert_eq!(serializer.serialize(&value), string7);
    }

    #[test]
    fn simple_string_again() {
        let serializer = Serializer::new();
        let string8 = String::from("+hello world\r\n");
        let value = Value::String(String::from("hello world"));
        assert_eq!(serializer.deserialize(&string8), value);
        assert_eq!(serializer.serialize(&value), string8);
    }
}