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

I made this type because I needed *mostly* / *somewhat* random identifiers that could be easily read and retyped by a user, but would also prevent collisions in somewhat small (less than a million or so) use-cases.

Examples `collision.rs` or `collision_average.rs` (**beware, this can take quite a while to run**) can be run to get an idea of how many IDs can be generated before collision occurs, but this is ultimately down to luck I suppose.  
*Generally, an average of 50-100 times gave me results in the 20 million range (IDs created before collision), but unlucky RNG has lead to results as low as 6-8 million.*

## Dependencies
The crate has either one or two dependencies, depending on whether serialization is needed. `fastrand` is used for RNG, `serde` is used for de/serialization **only if** the `serde` feature flag is enabled.

## Example
Further examples can be found in [./examples/basic.rs](./examples/basic.rs).


```rust
use tinyid::TinyId;

// Generate a random ID.
let mut id = TinyId::random();
// Ensure that the ID is valid.
assert!(id.is_valid());
assert!(!id.is_null());

id.make_null();
assert!(!id.is_valid());
assert!(id.is_null());
assert_eq!(id, TinyId::null());
```

## Features
The crate only has one feature, `serde`, which will enable serde serialization and deserialization of the `TinyId` type. It will also bring in the `serde` dependency.