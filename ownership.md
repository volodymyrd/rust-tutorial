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
