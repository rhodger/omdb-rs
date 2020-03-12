# ![logo](./logo.png)omdb-rs
> OMDb library for Rust

This is a library of tools for searching/interacting with the [Online Movie Database](http://www.omdbapi.com/). This is achieved through the use of a `Film` object that can hold certain common features of a film. For example...

```rust
use omdbrs::Film;

let NAME = String::from("Shrek");
let API_KEY = String::from("[YOUR_API_KEY]");

let film: Film = Film::from_title(NAME, API_KEY);

assert_eq!(film.get_year(), "2001");
```
