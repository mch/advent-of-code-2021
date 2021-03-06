# Questions and observations about Rust

## What does ``Box<dyn Error>` mean?
From the return type for `std::fs::read_to_string`.

`Box<T>` is a smart pointer that stores data on the heap.

## How can I clone a String inside a Result<String, io::Error>?
where io::Error doesn't implement Clone?

## Why can't I for loop over the same Vec twice without cloning?
Oh, I can if I borrow, but I'm not totally understanding the syntax.

```
let v = vec!(1,2,3);
for item in &v {
   println!("{}", v);
}

for item in &v {
   println!("{}", v);
}
```

I guess I'm passing a reference to the vec to the for loop and so iterating over references to the contents of the vector?

By default the IntoIterator produces an iterator by taking self by value, so the collection is consumed (values moved into the loop). But you can usually call `iter()` or `iter_mut()` to get iterators over references instead.

The example above, `for item in &v`, is shorthand for calling `iter()` on the vector. 

## What does IntoIterator mean? 
I initially read this as "pipe the contents of something into an iterator", but it's actually "take this and make an iterator out of it", or "turn this into an iterator". 

So types that implement IntoIterator can be made into an iterator.

## How does iterator mutability work?
From day1, I don't understand why numbers_iter has to be mutable. Is it because next() mutates self? If so, how do for loops etc get away with it? Do they make mutable copies?

The docs for `std::iter` are very helpful here, and show how a for loop is actually implemented using `IntoIterator`, so it internally does create a mutable iterator.

## Fold accumulators can be mutable!
For some reason I got the impression that they couldn't be, which is why my day1 and day2 solutions us immutable accumulators. 

For day 4 a mutable accumulator was really useful, and I stumbled across that while googling for something related. 

```
thing.fold(initial_state, |mut accumulator, item| { /* ... */ });
//                         ^^^
```

## It's kind of cool that iterators are lazy and composable
Python 3 iterators are also, and it always annoyed me that there was an extra step to get an actual list of vector out, but in Rust calling `collect()` is fine, although I forget and have to ponder the type error for a while... like what is a `Map` and why is it here??

## What is turbofish `::<>`?
From the docs for `str::parse`. How does `parse` use type information to do the parsing?

## What is this `<F as FromStr>::Err`?
From the docs for `str::parse`. Is the generic type `F` being cast to the `FromStr` trait, which must provide it's own error type? If so, why can't `Result` figure out the error type from the target type? Or maybe a specialized `Result` is need that can do that, they way `std::io::Result` is specialized.

## What is #[derive(PartialEq)]?
From the docs for `std::str::FromStr`. 

It's the trait for equality comparisons which are [partial equivalence relations](https://en.wikipedia.org/wiki/Partial_equivalence_relation).

Not sure why it's used on the Point struct in the example though. It seems to be needed for comparing in the `assert_eq!` macro.

## What is `Self` in `Result<Self, Self::Err>`?
From the docs for `std::str::FromStr`.

## What's going on with Box when building a vector for functions in main.rs?

## Is there way to better structure tests?
Seems like it's all based on the function name, no nesting like you get with `describe` in Mocha.

## Is there a way to set up test fixtures?
`before_each`, `after_each`, etc?

The lack of fixtures became kind of annoying in day9, where I wanted to used the same data across multiple tests, but not all tests.

Maybe the right answer is to just use tools provided by the language. Call a function that returns a struct with the constructed test data in it, and destructure it to extract the parts needed for an individual test. 

## What is the convention for naming test functions? 
I've included the prefix `test_` but it seems redundant given the `#[test]` annotation.

## What's the best way to assert Err Results?

## Is it expected to make new Error structs for each type of Error?

## When implementing an iterator, how should it connect to it's parent?
Keeping in mind lifetimes, etc... the natural thing might be to attempt to store a reference to the parent in the iter struct, but what if the iter lives longer than the thing it is iterating over?

## Whats the difference between `use` and `mod` and how should they be used?
I was confused about main.rs bringing in local modules using `mod` and modules from std using `use`. It seems
to me like `mod` declares a module, but if that's the case why isn't the `mod` keyword used in the file where 
the module lives, and `use` used where the module is imported?

## How do I make best make a vector of Strings?
`Vec<String>` is fine for the declaration, but then `vec!["foo"]` doesn't work because `"foo"` is not a `String`, but rather a `&str`. If I try `Vec<&str>` it fails to compile because it needs a lifetime parameter which I haven't learned about yet. 

## What is the difference between String and str, and how do they relate?

## What should constructors do if they are given data that can't construct a valid object?
E.g. day9, Heightmap... the list of points might not be valid, e.g. not enough points to have N complete rows.

## Should I delete tests that outlive they value?
In day9, while writing Heightmap, I added lots of small functions and unit tests for them in the interest of taking small steps. But many of these functions should not be part of the higher level Heightmap interface. Should I make them private and remove the unit tests?

Or perhaps extract a more generic struct that Heightmap uses internally to provide it's higher level abstraction. This is probably the right choice.

## Is there a way to assert the contents of a list, without caring about order?
Maybe just use a set type?

## Why does IntoIterator::into_iter take self without &?
I think it is because self has the type Self, which depends on the type IntoIterator is templated on, which already includes the &?

## How do closures capture scope and how to I control mutable vs immutable borrowing?
See day11.

Maybe the `move` keyword to create a move closure?

What does `drop(v);` do?

## How do I implement printing of a struct?
Implement the `std::fmt::Display` trait. Use `write!()?;` to return early if there is a write error.

## Whats the "right" way to handle Result and Option types without unwrap() all the time?
Properly handling errors in general.

## In petgraph algo, what does where mean?
```
pub fn dijkstra<G, F, K>(
    graph: G,
    start: G::NodeId,
    goal: Option<G::NodeId>,
    mut edge_cost: F,
) -> HashMap<G::NodeId, K>
where
    G: IntoEdges + Visitable,
    G::NodeId: Eq + Hash,
    F: FnMut(G::EdgeRef) -> K,
    K: Measure + Copy,
{
```

## What is the type of the tuple elements?
```

```
