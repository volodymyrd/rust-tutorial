#[derive(Debug)]
struct Foo;

impl Foo {
    // 1. mutate_and_share<'a>(&'a mut self) -> &'a Self
    // This signature is key. It says: "I take a mutable reference with lifetime 'a,
    // and I return an immutable reference that has the *same* lifetime 'a."
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn only_share(&self) -> &Self {
        &*self
    }
    // 2. share<'a>(&'a self)
    // This takes an immutable reference with lifetime 'a.
    fn share(&self) {}
}
fn main() {
    // 'b: { // Outermost scope, where 'foo' lives
    let mut foo = Foo;
    //'c: { // This is the lifetime for 'loan' and the temporary `&mut foo` used in the call
    // to `mutate_and_share`
    // 3. let loan: &'c Foo = Foo::mutate_and_share::<'c>(&'c mut foo);
    let loan = foo.mutate_and_share(); // Rust infers 'c for the arguments and return

    // Explanation point 1: "The lifetime system is forced to extend the &mut foo to have lifetime 'c,
    // due to the lifetime of loan and mutate_and_share's signature."

    // Let's elaborate:
    // When `mutate_and_share` is called, it temporarily takes a `&mut foo`.
    // Its signature says: input `&'a mut self`, output `&'a Self`.
    // Since the *output* (`loan`) needs to live for lifetime `'c`,
    // the *input* mutable reference (`&mut foo` that was passed to the function)
    // must also have effectively lasted for lifetime `'c`.
    // This means that for the entire duration of `'c` (from the call to `mutate_and_share`
    // until `loan` goes out of scope), `foo` is considered "mutably borrowed."
    // The conceptual `&'c mut foo` persists for the entire `'c` scope.

    //'d: {
    // This is the lifetime for the temporary `&foo` used in the call to `share`
    // 4. Foo::share::<'d>(&'d foo);
    // Foo::share(&foo); // Rust infers 'd for the argument

    // Explanation point 2: "Then when we try to call share, it sees we're trying to alias
    // that &'c mut foo and blows up in our face!"

    // Let's elaborate:
    // At this point, inside scope 'd (which is nested within 'c):
    //   - `loan` (of type `&'c Foo`) is still active. This means `foo` is considered to be under
    // a *mutable* borrow that lasts for `'c`.
    //   - We are attempting to call `foo.share()`, which requires an *immutable* borrow
    // of `foo` (`&'d foo`).
    //
    // The borrow checker sees this:
    //   - Active: `&'c mut foo` (because `loan` exists and its lifetime `'c` is still active,
    // and `loan` originated from a `&mut` call).
    //   - Attempted: `&'d foo` (for the `share` call).
    //
    // This is a direct violation of the rule: you cannot have an active mutable borrow
    // (`&'c mut foo`) and another borrow (even an immutable one like `&'d foo`)
    // of the same data (`foo`) simultaneously. The borrow checker detects this aliasing attempt
    // and prevents compilation.
    foo.share(); // doesn't compile
    //} 'd ends, temporary `&foo` for `share` goes out of scope

    println!("{:?}", loan); // This line *uses* 'loan', so 'loan' must be valid.
    // This confirms 'c must extend at least up to this line.

    //} 'c ends. `loan` goes out of scope, releasing the mutable borrow on `foo`.
    //} 'b ends. `foo` goes out of scope.
}
