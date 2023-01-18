// Copyright (c) 2023 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use tinyid::TinyId;

fn main() {
    println!("Generating TinyIds until a collision occurs...");
    let start = std::time::Instant::now();
    let iters = get_collision();
    let elapsed = start.elapsed();
    let pretty_iters = iters
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(","); // separator
    println!("Collision after {pretty_iters} iterations.");
    println!("Elapsed time: {elapsed:#?}");
}

fn get_collision() -> usize {
    let mut ids = std::collections::HashSet::new();
    loop {
        let id = TinyId::random();
        if !ids.insert(id) {
            break;
        }
    }
    ids.len()
}
