// Copyright (c) 2023 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use tinyid::TinyId;

const ITERS: usize = 100;

fn main() {
    println!("Generating {ITERS} TinyIds...");
    for x in (0..(ITERS)).step_by(2) {
        let id = TinyId::random();
        let mut n = x + 1;
        print!("#{n:03}: {id}");
        print!(" | ");
        let id = TinyId::random();
        n += 1;
        println!("#{n:03}: {id}");
    }
}
