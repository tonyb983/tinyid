// Copyright (c) 2023 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use tinyid::TinyId;

const TIMES: usize = 100;

fn main() {
    println!(
        "Checking average iterations until collision using N = {TIMES}, this WILL TAKE SEVERAL (OR MORE) MINUTES..."
    );
    println!();
    println!("Progress: 00%");
    let mut total_iters = 0;
    for i in 0..TIMES {
        total_iters += run_until_collision();
        let percent = ((i + 1) as f64 / TIMES as f64) * 100.0;
        // Move cursor to beginning of previous line
        print!("\u{1b}[1F");
        // Clear from cursor to end of line
        print!("\u{1b}[2K");
        // Rewrite progress. This doesn't seem to work with `print!`, it seems to need the `println!`.
        println!("Progress: {percent:02.0}%");
    }
    println!();
    let avg_iters = total_iters / TIMES;
    let pretty_iters = avg_iters
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(","); // separator
    println!("Average iterations until collision after {TIMES} attempts: {pretty_iters}");
}

fn run_until_collision() -> usize {
    let mut ids = std::collections::HashSet::new();
    let mut iters = 0;
    loop {
        iters += 1;
        let id = TinyId::random();
        if !ids.insert(id) {
            break;
        }
    }
    iters
}
