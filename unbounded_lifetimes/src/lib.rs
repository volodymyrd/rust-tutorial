#[allow(dead_code)]
fn get_str1(s: &str) -> &str {
    s
}

#[allow(dead_code)]
fn get_str2<'a>() -> &'a str {
    "hello"
}

// fn get_str2_1<'a>() -> &'a str {
//     let s = String::from("hello");
//     // error[E0515]: cannot return reference to local variable `s`
//     &s
// }

#[allow(dead_code)]
fn get_str2_2() -> &'static str {
    "hello"
}

#[allow(dead_code)]
fn get_str3<'a>(s: *const String) -> &'a str {
    unsafe { &*s }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_str1() {
        let s = "Hello World".to_string();
        let s1 = get_str1(&s);
        //drop(s); // this doesn't compile
        println!("s1: {s1}");
        assert_eq!(s1, "Hello World");
    }

    #[test]
    fn test_get_str2() {
        assert_eq!(get_str2(), "hello");
    }

    #[test]
    fn test_get_str2_2() {
        assert_eq!(get_str2_2(), "hello");
    }

    #[test]
    fn test_get_str3() {
        assert_eq!(get_str3(&"Hello World".to_string()), "Hello World");
    }

    #[test]
    #[should_panic]
    fn test_get_str3_1() {
        let s = "Hello World".to_string();
        let s1 = get_str3(&s);
        drop(s);
        println!("s1: {s1}");
        assert_eq!(s1, "Hello World");
    }
}
