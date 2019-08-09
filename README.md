# Circular reader

[![Build Status](https://travis-ci.org/PhilboBaggins/circular-reader.svg?branch=master)](https://travis-ci.org/PhilboBaggins/circular-reader)

Read from an iterator again and again and again...

***Deprecation notice*** - This crate is deprecated by a standard library function that existed before I even thought of writing it. Don't you hate it when that happens? I thought this crate would be useful for use in my [cryptXor](https://github.com/PhilboBaggins/cryptXor) application but then I found that the authors of the Rust standard library beat me to it by putting the [cycle function in the Iterator trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cycle). Their function does exactly what I was looking for and is actually completed (unlike my solution which only works vectors at the moment). So I guess I will abandon <strike>all hope</strike> development of this crate and just use the standard library function instead.

## License

Licensed under either of the following:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
