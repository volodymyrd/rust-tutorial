#[derive(Debug)]
struct X<'a>(&'a i32);

//impl Drop for X<'_> {
//    fn drop(&mut self) {}
//}

#[allow(unused_variables)]
#[allow(unused_labels)]
pub fn lifetime1() {
    let mut data = vec![1, 2, 3];
    //let x = &data[0]; // compile
    let x = X(&data[0]); // doesn't compile if there a drop impl for struct X.
    //data.push(4); // doesn't compile
    println!("{:?}", x);
    data.push(4);
}//immutable borrow might be used here, when `x` is dropped and runs the `Drop` code for type `X`

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetime1() {
        lifetime1();
    }
}
