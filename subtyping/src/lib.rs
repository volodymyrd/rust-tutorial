use std::cell::RefCell;

// it's ok to treat &'a T as a subtype of &'b T if 'a <: 'b,
// therefore we can say that &'a T is covariant over 'a.
#[allow(dead_code)]
fn debug<'a>(a: &'a str, b: &'a str) {
    println!("a = {a:?} b = {b:?}");
}

#[allow(dead_code)]
fn foo<'short, 'long: 'short>(x: &'long u32) -> &'short u32 {
    x
}

// it's not ok to treat &mut &'a T as a subtype of &mut &'b T,
// therefore we can say that &mut T is invariant over T
#[allow(dead_code)]
fn assign<'a, T>(input: &'a mut T, val: T) {
    *input = val;
}

// function types, unlike anything else in the language, are contravariant over their arguments.
thread_local! {
    pub static STATIC_VECS: RefCell<Vec<&'static str>> = RefCell::new(Vec::new());
}

/// saves the input given into a thread local `Vec<&'static str>`
fn store(input: &'static str) {
    STATIC_VECS.with_borrow_mut(|v| v.push(input));
}

/// Calls the function with it's input (must have the same lifetime!)
fn demo<'a>(input: &'a str, f: fn(&'a str)) {
    f(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let hello: &'static str = "hello";

        {
            let world = String::from("world");
            let world = &world; // 'world has a shorter lifetime than 'static hello
            debug(hello, world);
        }
    }
    #[test]
    fn test2() {
        //let mut hello: &'static str = "hello";
        {
            //let world = String::from("world");
            //assign(&mut hello, &world); doesn't compile
        }
        //println!("{hello}"); // use after free
    }

    #[test]
    fn test3() {
        demo("hello", store); // "hello" is 'static. Can call `store` fine

        {
            let smuggle = String::from("smuggle");

            // `&smuggle` is not static. If we were to call `store` with `&smuggle`,
            // we would have pushed an invalid lifetime into the `STATIC_VECS`.
            // Therefore, `fn(&'static str)` cannot be a subtype of `fn(&'a str)`

            //demo(&smuggle, store);// doesn't compile
        }

        // use after free 😿
        STATIC_VECS.with_borrow(|v| println!("{v:?}"));
    }
}
