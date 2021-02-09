#![feature(test)]
extern crate test;
extern crate rand;

use rand::Rng;
use fov::BitGrid;
use fov::FOV;
#[cfg(test)]
use self::test::Bencher;

#[bench]
fn create_fov(b: &mut Bencher) {
    b.iter(|| {
        FOV::<101>::new();
    });
}

#[bench]
fn use_fov(bencher: &mut Bencher) {
    use rand_xorshift::XorShiftRng;

    let d = 100isize;
    let mut state = BitGrid::new(d as usize, d as usize, false);
    let mut rng: XorShiftRng = rand::SeedableRng::seed_from_u64(69_420);
    let fov = FOV::<100>::new();

    for _ in 0..(d * d) {
        let x = rng.gen_range(0, d as isize);
        let y = rng.gen_range(0, d as isize);
        state.set((x, y), true);
    }

    bencher.iter(|| {
        fov.compute(&state, (d / 2, d / 2), (d / 2) as u8);
    });
}
