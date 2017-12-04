[![](http://meritbadge.herokuapp.com/token_store)](https://crates.io/crates/token_store)
[![Build Status](https://travis-ci.org/vberger/token_store.svg?branch=master)](https://travis-ci.org/token_store)
[![Coverage Status](https://coveralls.io/repos/github/vberger/token_store/badge.svg)](https://coveralls.io/github/vberger/token_store)

# Token Store

This crate provides a simple token-based store for arbitrary types.

## What is it for?

This crate was actually part of wayland-rs initially and extracted. The
reason for its existence is the way these crates are designed, there is
strong separation of data vs logic.

This `token_store` works well in such configurations: you have a set of
well-separated modules that need to share data but don't need to access
it conccurently. And also, the modules are not necessarily aware of each
other (and so you can not really use a big fixed struct for storing all
the shared data.

Using `token_store`, at initialization each module will store the value it
needs in the store, and keep the tokens internally. It can optionnaly provide
to the outside world tokens to access values that are to be shared.

Then, when each module needs to do its work, it just needs a `&mut Store`
and can retrieve with its tokens the data it needs to work on, independently
of what other modules may have stored in.

## How do I use it?

```rust
use token_store::Store;

// create a store
let mut store = Store::new();

// insert some things in it, you are given tokens
let token1 = store.insert(42);

// you can store any type as log as it is `Any + 'static`
let token2 = store.insert(String::from("I like trains"));

// the tokens keep the information of the store type,
// as such you don't need any annotation to retrieve a value:
store.get_mut(&token2).push_str(", and cars too!");
```

The retrieved tokens can be cloned and shared as you like between various
parts of your code.


## Documentation

The documentation for the master branch is [available online](https://vberger.github.io/token_store/).

The documentation for the releases can be found on [docs.rs](https://docs.rs/token_store):

