<!--
 Copyright (c) 2022 Tony Barbitta
 
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

![Rust](https://github.com/tonyb983/tinyid/actions/workflows/rust_cached.yml/badge.svg)
[![coveralls.io](https://coveralls.io/repos/github/tonyb983/tinyid/badge.svg?branch=main)](https://coveralls.io/github/tonyb983/tinyid?branch=main)
[![codecov](https://codecov.io/gh/tonyb983/tinyid/branch/main/graph/badge.svg?token=TKNPNKU8IC)](https://codecov.io/gh/tonyb983/tinyid)

# TinyId

A small, 8-byte, ID type for use in rust applications that need a *pretty unique* identifier that is not required to be cryptographically secure / correct. They can be randomly generated but no work has been done to make sure that these random generations are secure (all RNG is done through the excellent [`fastrand`](https://crates.io/crates/fastrand) crate).

I made this type because I needed *mostly* / *somewhat* random identifiers that could be easily read and retyped by a user, but would also prevent collisions in somewhat small (less than a million or so) applications.

## Example
```rust
use tinyid::TinyId;

let mut id = TinyId::random();
assert!(id.is_valid());
assert!(!id.is_null());

id.make_null();
assert!(!id.is_valid());
assert!(id.is_null());
assert_eq!(id, TinyId::null());
```