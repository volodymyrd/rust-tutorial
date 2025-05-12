// cargo expand --lib --tests
#[macro_export]
macro_rules! vecmac {
    () => {
        Vec::new()
    };
    ($element:expr) => {{
        let mut v = Vec::new();
        v.push($element);
        v
    }};
    ($el1:expr, $el2:expr) => {{
        let mut v = Vec::new();
        v.push($el1);
        v.push($el2);
        v
    }};
    ($($element:expr),* $(,)?) => {{
        let mut v = Vec::new();
        $(v.push($element);)*
        v
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_vec() {
        // vecmac!();
        // vecmac! {};
        // vecmac![];

        let x: Vec<i32> = vecmac!();
        assert!(x.is_empty())
    }

    #[test]
    fn single() {
        let x: Vec<u32> = vecmac![42];

        assert_eq!(x, vec![42]);
    }

    #[test]
    fn double() {
        let x: Vec<u32> = vecmac![42, 43];

        assert_eq!(x, vec![42, 43]);
    }

    #[test]
    fn any_number() {
        let x: Vec<u32> = vecmac![42, 43, 45, 47, 49,];

        assert_eq!(x, vec![42, 43, 45, 47, 49]);
    }
}
