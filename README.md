# omdb-rs
> OMDb library for Rust

This is a library of tools for searching/interacting with the [Online Movie Database](http://www.omdbapi.com/). This is achieved through the use of a `Film` object that can hold certain common features of a film. For example...

```rust
use omdbrs::Film;

let film: Film = Film::from_title("Shrek");

assert_eq!(film.get_year(), "2001");
```

Note: This library uses an outdated version of Reqwest.

Note: This library currently uses a hardcoded API key with a limit of 1000 requests daily; this will be changed to a dynamic key in a future release.
