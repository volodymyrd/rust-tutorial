// pub fn strtok<'a, 'b>(s: &'a mut &'b str, delimimiter: char) -> &'b str {
// pub fn strtok<'a>(s: &'_ mut &'a str, delimimiter: char) -> &'a str {
pub fn strtok<'a>(s: &mut &'a str, delimimiter: char) -> &'a str {
    if let Some(i) = s.find(delimimiter) {
        let prefix = &s[..i];
        let suffix = &s[i + delimimiter.len_utf8()..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s: &'static str = "Hello World!";

        let result = strtok(&mut s, ' ');

        assert_eq!(result, "Hello");
        assert_eq!(s, "World!");
    }
}
