# Ownership and Lifetimes

Rust's **ownership** model enforces memory safety without a garbage collector.
Each value in Rust has a single owner, and when ownership is **moved**,
the original binding can no longer access the value. **Cloning** creates a copy,
and **borrowing** (via references) allows access without taking ownership.
The **lifetimes** ensure references don’t outlive the data they point to,
preventing dangling pointers. Advanced behaviors like **aliasing**
and **mutability** are carefully controlled to avoid undefined behavior.

## References

There are two kinds of references:

* **Shared** reference: &
* **Mutable** reference: &mut

Which obey the following rules:

* A reference cannot **outlive** its referent
* A mutable reference cannot be **aliased**

That's it. That's the whole model references follow.

**Violating aliasing or mutability rules leads to undefined behavior**,
so extra care is needed when working with raw pointers or unsafe

## Aliasing

Aliasing means having **multiple pointers or references to the same memory**.
In Rust, this is carefully controlled: you can have **multiple immutable references** (&T)
or **one mutable reference** (&mut T), but not both at the same time. This rule prevents
undefined behavior from conflicting reads and writes.
Unsafe code must strictly follow this rule—even with raw pointers—to maintain memory safety.

## Lifetimes

**What are Lifetimes?**

* Lifetimes are named regions of code that a reference must be valid for. They ensure that a reference does not outlive
  the data it points to (its "referent").
* Rust uses lifetimes to prevent common programming errors like dangling pointers and use-after-free bugs at compile
  time.
* While they can correspond to simple scopes in basic examples, lifetimes can represent more complex execution paths,
  even with "holes" if a reference is invalidated and reinitialized.
* Types that contain references can also be tagged with lifetimes to prevent their invalidation.

**How Rust Handles Lifetimes:**

* **Inference and Elision:** Rust heavily relies on aggressive inference and "elision" (omission) of lifetimes, meaning
  you
  often don't have to write them explicitly, especially within function bodies. This makes the code much cleaner.
* **Explicit Lifetimes:** When references cross function boundaries (e.g., in function signatures), you need to
  explicitly
  denote lifetimes using an apostrophe (e.g., 'a, 'static).
* **Implicit Scopes:** Each let statement implicitly introduces a scope, which can influence how lifetimes are inferred,
  especially when variables refer to each other. The borrow checker tries to minimize the extent of a lifetime.

**Key Concepts Explained:**

* References that Outlive Referents: This is a core problem lifetimes prevent. The example of as_str function
  demonstrates how returning a reference to a temporary value (like s created inside the function) with a lifetime tied
  to an outer scope leads to a compile-time error. The solution is often to return an owned value (e.g., String) instead
  of a reference if the data is created within the function.

* Aliasing a Mutable Reference: Rust's rules prevent having a live shared reference (&T) to data at the same time as a
  mutable reference (&mut T) to the same data (or its descendants). The example with Vec and data.push(4) illustrates
  this. Even though x might be a reference to a sub-part of data, Rust's lifetime system, being more coarse, sees x as
  having a lifetime tied to data and disallows the mutable borrow while x is still considered "alive."

**Area Covered by a Lifetime:**

* A reference is "alive" from its creation to its last use. The borrowed value only needs to outlive the active borrows.
* Rust can be smart about this: a reference might exist as a variable, but if its last use has passed, subsequent
  operations on the original data might be allowed (e.g., data.push(4) after println!("{}", x) if x is no longer
  needed).
* Destructors as Uses: If a value has a destructor (Drop implementation), running the destructor at the end of its scope
  is considered a "use," which can prevent operations that would invalidate the reference before the destructor runs.
* Conditional Last Uses: Lifetimes can adapt to conditional paths, where the last use of a borrow might occur in
  different branches.
* Paused Lifetimes: A lifetime can effectively have pauses or be seen as distinct borrows tied to the same variable,
  particularly around loops where a new reference is assigned.

In essence, lifetimes are Rust's non-garbage-collected solution to memory safety, allowing it to provide strong
guarantees without runtime overhead by performing strict checks at compile time.

## Lifetime Elision

The compiler uses three rules to figure out the lifetimes of the references
when there aren’t explicit annotations.

The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a
function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two
separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output
lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because
this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much
nicer to read and write because fewer symbols are necessary.

## Unbounded Lifetimes

Unsafe code can often end up producing references or lifetimes out of thin air.
Such lifetimes come into the world as unbounded.
The safest and easiest way to bound a lifetime is to return it from a function with a bound lifetime.

## Higher-Rank Trait Bounds (HRTBs)

HRTBs (Higher-Ranked Trait Bounds) allow you to say “for all lifetimes” in a trait bound.
They make it possible to express that a trait implementation must work regardless of the lifetime used.

```rust
impl<F> Closure<F>
where
        for<'a> F: Fn(&'a (u8, u16)) -> &'a u8,
{
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}
```

`for<'a> `can be read as "for all choices of 'a", and basically produces an infinite list of trait bounds
that F must satisfy.

## Subtyping and Variance

Subtyping in Rust refers to situations where one type can be used in place of another, typically involving lifetimes.
It's mostly relevant in unsafe code and lifetime variance.

* Subtyping mainly applies to lifetimes, not general types.
* 'a is a subtype of 'b if 'a lives at least as long as 'b.
* Types like &'a T are covariant over 'a, meaning &'static T can be used where &'short T is expected.
* Variance (covariant, contravariant, invariant) determines how lifetimes behave in complex types.
* Unsafe code must be careful with subtyping and variance to avoid memory safety issues.

## Drop Check

Drop Check (Dropck), Rust’s mechanism for ensuring that destructors (i.e., Drop implementations) run safely.

* Dropck ensures safety during destruction by checking that all types a value owns or touches are valid when its Drop
  code runs.
* It prevents use-after-free bugs by enforcing that generic types with Drop cannot assume they own references unless
  explicitly bounded (e.g., with T: 'a).
* If a type has a destructor (impl Drop), then Rust will conservatively check the lifetimes of its fields at the point
  of drop.
* You can influence this behavior using #[may_dangle], a special attribute that allows a type parameter to be dangling
  when the destructor runs — used only with unsafe code and advanced patterns.

## PhantomData

PhantomData is a zero-sized type used to indicate that a type logically owns or acts as if it uses a generic type, even
if that type isn't actually used in memory.

The Rust compiler uses type information to enforce ownership, variance, and drop check rules. If a generic parameter
isn’t used in a way that the compiler can detect, you must use PhantomData to tell the compiler how your type should
behave.

**Main purposes:**

- Ownership / drop checking: Letting the compiler know that your type conceptually owns a value of type T.
- Variance control: Influencing whether a generic parameter is covariant, contravariant, or invariant.
- Unsafe code correctness: Ensuring the compiler enforces memory safety assumptions when your type interacts with
  lifetimes or raw pointers.
