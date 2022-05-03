// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This file contains the code that was scrapped when the [`TinyId`] type was pulled out
//! of the project that it grew up in. These were mostly RNG and ID Generation comparisons
//! to measure the performance of different RNG libraries and id generation methods. This
//! can all probably be safely deleted, but I'm storing it for now just in case.

impl TinyId {
    #[must_use]
    pub(crate) fn random_nanorand1() -> Self {
        use nanorand::Rng;
        let mut rng = nanorand::tls_rng();
        let mut data = [NULL_CHAR; 8];
        for ch in &mut data {
            *ch = LETTERS[rng.generate_range(0..LETTER_COUNT)];
        }

        Self { data }
    }

    #[must_use]
    pub(crate) fn random_nanorand2() -> Self {
        use nanorand::{BufferedRng, Rng, WyRand};
        let mut rng = BufferedRng::new(nanorand::tls_rng());

        let mut data = [NULL_CHAR; 8];
        rng.fill_bytes(&mut data);
        for ch in &mut data {
            *ch = LETTERS[*ch as usize % LETTER_COUNT];
        }
        Self { data }
    }

    #[must_use]
    pub(crate) fn random_nanorand3() -> Self {
        use nanorand::Rng;
        let mut rng = nanorand::tls_rng();
        let mut data = rng.rand();
        for ch in &mut data {
            *ch = LETTERS[*ch as usize % LETTER_COUNT];
        }

        Self { data }
    }

    #[must_use]
    pub(crate) fn random_nanorand4() -> Self {
        use nanorand::Rng;
        let mut rng = nanorand::tls_rng();
        let mut data: [u8; 8] = rng.generate::<usize>().to_be_bytes();
        for ch in &mut data {
            *ch = LETTERS[*ch as usize % LETTER_COUNT];
        }

        Self { data }
    }

    #[must_use]
    pub(crate) fn random_rand1() -> Self {
        use rand::distributions::{Alphanumeric, Distribution, Uniform};
        let range = Uniform::new(0, LETTER_COUNT);
        let mut rng = rand::thread_rng();
        let mut data = [NULL_CHAR; 8];
        for b in range
            .sample_iter(&mut rng)
            .take(8)
            .enumerate()
            .map(|(i, l)| (i, LETTERS[l]))
        {
            data[b.0] = b.1;
        }

        Self { data }
    }

    #[must_use]
    pub(crate) fn random_rand2() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut data: [u8; 8] = rng.gen::<usize>().to_be_bytes();
        for b in &mut data {
            *b = LETTERS[*b as usize % LETTER_COUNT];
        }

        Self { data }
    }

    /// This method is pretty useless since it relies on having a random seed
    /// to create a new [`oorandom::Rand32`] or [`oorandom::Rand64`] instance.
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub(crate) fn random_oor() -> Self {
        const LETTER_COUNT_U32: u32 = LETTER_COUNT as u32;
        let mut rng: oorandom::Rand32 = oorandom::Rand32::new(fastrand::u64(..));
        let mut data = [NULL_CHAR; 8];
        for ch in &mut data {
            *ch = LETTERS[rng.rand_range(0..LETTER_COUNT_U32) as usize];
        }

        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    /// Test comparing the multiple different RNG methods. Ignored by default
    /// because it doesnt really test anything, it is just useful to see
    /// how long each method takes to generate the id.
    #[allow(clippy::cast_possible_truncation, clippy::similar_names)]
    #[test]
    #[ignore]
    #[cfg_attr(coverage, no_coverage)]
    fn rng_compare() {
        const ITERS: usize = 1_000_000;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _id = TinyId::random_fastrand();
        }
        let fr_elapsed = now.elapsed();
        let fr_average = fr_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _id = TinyId::random_fastrand2();
        }
        let fr2_elapsed = now.elapsed();
        let fr2_average = fr2_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _id = TinyId::random_rand1();
        }
        let rand1_elapsed = now.elapsed();
        let rand1_average = rand1_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _id = TinyId::random_rand2();
        }
        let rand2_elapsed = now.elapsed();
        let rand2_average = rand2_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _id = TinyId::random_nanorand1();
        }
        let nano1_elapsed = now.elapsed();
        let nano1_average = nano1_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _id = TinyId::random_nanorand2();
        }
        let nano2_elapsed = now.elapsed();
        let nano2_average = nano2_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _id = TinyId::random_nanorand3();
        }
        let nano3_elapsed = now.elapsed();
        let nano3_average = nano3_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _id = TinyId::random_nanorand4();
        }
        let nano4_elapsed = now.elapsed();
        let nano4_average = nano4_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _id = TinyId::random_oor();
        }
        let oor_elapsed = now.elapsed();
        let oor_average = oor_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _id = names_random();
        }
        let names_elapsed = now.elapsed();
        let names_average = names_elapsed / ITERS as u32;

        println!("Results after {} iterations:", ITERS);
        println!();
        println!(" fastrand1: {:>10?} ({:>10?} ave.)", fr_elapsed, fr_average);
        println!(
            " fastrand2: {:>10?} ({:>10?} ave.)",
            fr2_elapsed, fr2_average
        );
        println!(
            "    rand 1: {:>10?} ({:>10?} ave.)",
            rand1_elapsed, rand1_average
        );
        println!(
            "    rand 2: {:>10?} ({:>10?} ave.)",
            rand2_elapsed, rand2_average
        );
        println!(
            "nanorand 1: {:>10?} ({:>10?} ave.)",
            nano1_elapsed, nano1_average
        );
        println!(
            "nanorand 2: {:>10?} ({:>10?} ave.)",
            nano2_elapsed, nano2_average
        );
        println!(
            "nanorand 3: {:>10?} ({:>10?} ave.)",
            nano3_elapsed, nano3_average
        );
        println!(
            "nanorand 4: {:>10?} ({:>10?} ave.)",
            nano4_elapsed, nano4_average
        );
        println!(
            "  oorandom: {:>10?} ({:>10?} ave.)",
            oor_elapsed, oor_average
        );
        println!(
            "     names: {:>10?} ({:>10?} ave.)",
            names_elapsed, names_average
        );
    }

    /// Same as the previous test, `rng_compare`, but this time the results
    /// are stored, and after the timing is captured, each ID is checked to
    /// confirm validity. I found a bug in the `random_nano2` method using this.
    #[allow(
        clippy::cast_possible_truncation,
        clippy::similar_names,
        clippy::needless_range_loop,
        clippy::too_many_lines
    )]
    #[test]
    #[ignore]
    #[cfg_attr(coverage, no_coverage)]
    fn rng_compare_validated() {
        const ITERS: usize = 1_000_000;
        let mut generated = box [TinyId::null(); ITERS];

        generated = box [TinyId::null(); ITERS];
        let now = std::time::Instant::now();
        for i in 0..ITERS {
            generated[i] = TinyId::random_fastrand();
        }
        let fr_elapsed = now.elapsed();
        let fr_average = fr_elapsed / ITERS as u32;
        assert!(
            generated.iter().all(|id| id.is_valid()),
            "fastrand1 failed validation!"
        );

        generated = box [TinyId::null(); ITERS];
        let now = std::time::Instant::now();
        for i in 0..ITERS {
            generated[i] = TinyId::random_fastrand2();
        }
        let fr2_elapsed = now.elapsed();
        let fr2_average = fr2_elapsed / ITERS as u32;
        assert!(
            generated.iter().all(|id| id.is_valid()),
            "fastrand2 failed validation!"
        );

        generated = box [TinyId::null(); ITERS];
        let now = std::time::Instant::now();
        for i in 0..ITERS {
            generated[i] = TinyId::random_rand1();
        }
        let rand1_elapsed = now.elapsed();
        let rand1_average = rand1_elapsed / ITERS as u32;
        assert!(
            generated.iter().all(|id| id.is_valid()),
            "rand1 failed validation!"
        );

        generated = box [TinyId::null(); ITERS];
        let now = std::time::Instant::now();
        for i in 0..ITERS {
            generated[i] = TinyId::random_rand2();
        }
        let rand2_elapsed = now.elapsed();
        let rand2_average = rand2_elapsed / ITERS as u32;
        assert!(
            generated.iter().all(|id| id.is_valid()),
            "rand2 failed validation!"
        );

        generated = box [TinyId::null(); ITERS];
        let now = std::time::Instant::now();
        for i in 0..ITERS {
            generated[i] = TinyId::random_nanorand1();
        }
        let nano1_elapsed = now.elapsed();
        let nano1_average = nano1_elapsed / ITERS as u32;
        assert!(
            generated.iter().all(|id| id.is_valid()),
            "nanorand1 failed validation!"
        );

        generated = box [TinyId::null(); ITERS];
        let now = std::time::Instant::now();
        for i in 0..ITERS {
            generated[i] = TinyId::random_nanorand2();
        }
        let nano2_elapsed = now.elapsed();
        let nano2_average = nano2_elapsed / ITERS as u32;
        assert!(
            generated.iter().all(|id| id.is_valid()),
            "nanorand2 failed validation!"
        );

        generated = box [TinyId::null(); ITERS];
        let now = std::time::Instant::now();
        for i in 0..ITERS {
            generated[i] = TinyId::random_nanorand3();
        }
        let nano3_elapsed = now.elapsed();
        let nano3_average = nano3_elapsed / ITERS as u32;
        assert!(
            generated.iter().all(|id| id.is_valid()),
            "nanorand3 failed validation!"
        );

        generated = box [TinyId::null(); ITERS];
        let now = std::time::Instant::now();
        for i in 0..ITERS {
            generated[i] = TinyId::random_nanorand4();
        }
        let nano4_elapsed = now.elapsed();
        let nano4_average = nano4_elapsed / ITERS as u32;
        assert!(
            generated.iter().all(|id| id.is_valid()),
            "nanorand4 failed validation!"
        );

        generated = box [TinyId::null(); ITERS];
        let now = std::time::Instant::now();
        for i in 0..ITERS {
            generated[i] = TinyId::random_oor();
        }
        let oor_elapsed = now.elapsed();
        let oor_average = oor_elapsed / ITERS as u32;
        assert!(
            generated.iter().all(|id| id.is_valid()),
            "oorandom failed validation!"
        );

        println!("Results after {} iterations:", ITERS);
        println!();
        println!(" fastrand1: {:>10?} ({:>10?} ave.)", fr_elapsed, fr_average);
        println!(
            " fastrand2: {:>10?} ({:>10?} ave.)",
            fr2_elapsed, fr2_average
        );
        println!(
            "    rand 1: {:>10?} ({:>10?} ave.)",
            rand1_elapsed, rand1_average
        );
        println!(
            "    rand 2: {:>10?} ({:>10?} ave.)",
            rand2_elapsed, rand2_average
        );
        println!(
            "nanorand 1: {:>10?} ({:>10?} ave.)",
            nano1_elapsed, nano1_average
        );
        println!(
            "nanorand 2: {:>10?} ({:>10?} ave.)",
            nano2_elapsed, nano2_average
        );
        println!(
            "nanorand 3: {:>10?} ({:>10?} ave.)",
            nano3_elapsed, nano3_average
        );
        println!(
            "nanorand 4: {:>10?} ({:>10?} ave.)",
            nano4_elapsed, nano4_average
        );
        println!(
            "  oorandom: {:>10?} ({:>10?} ave.)",
            oor_elapsed, oor_average
        );
    }

    /// Compares generating 1,000,000 instances of:
    /// - [`TinyId::random`]
    /// - [`uuid::Uuid::new_v4`]
    /// - [`fastrand::u64`]
    /// - [`fastrand::u8`] x8
    /// Results are better now using the second fastrand implementation.
    #[allow(clippy::cast_possible_truncation, clippy::similar_names)]
    #[test]
    #[ignore]
    #[cfg_attr(coverage, no_coverage)]
    fn generation_comparison() {
        const ITERS: usize = 1_000_000;
        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _id = TinyId::random();
        }
        let sid_elapsed = now.elapsed();
        let sid_average = sid_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _num = fastrand::u64(..);
        }
        let num_elapsed = now.elapsed();
        let num_average = num_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _n1 = fastrand::u8(..);
            let _n2 = fastrand::u8(..);
            let _n3 = fastrand::u8(..);
            let _n4 = fastrand::u8(..);
            let _n5 = fastrand::u8(..);
            let _n6 = fastrand::u8(..);
            let _n7 = fastrand::u8(..);
            let _n8 = fastrand::u8(..);
        }
        let num8_elapsed = now.elapsed();
        let num8_average = num8_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _uuid = uuid::Uuid::new_v4();
        }
        let uuid_elapsed = now.elapsed();
        let uuid_average = uuid_elapsed / ITERS as u32;

        let now = std::time::Instant::now();
        for _ in 0..ITERS {
            let _name = names_random();
        }
        let names_elapsed = now.elapsed();
        let names_average = names_elapsed / ITERS as u32;

        println!("Results after {} iterations:", ITERS);
        println!();
        println!(
            "         TinyId: {:>10?} ({:>10?} ave.)",
            sid_elapsed, sid_average
        );
        println!(
            "   fastrand::u64: {:>10?} ({:>10?} ave.)",
            num_elapsed, num_average
        );
        println!(
            "fastrand::u8(x8): {:>10?} ({:>10?} ave.)",
            num8_elapsed, num8_average
        );
        println!(
            "    Uuid::new_v4: {:>10?} ({:>10?} ave.)",
            uuid_elapsed, uuid_average
        );
        println!(
            "     names crate: {:>10?} ({:>10?} ave.)",
            names_elapsed, names_average
        );
    }
}
