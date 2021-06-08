#[macro_use]
extern crate bencher;

use bencher::Bencher;
extern crate rand;

use rand::Rng;
use fov::BitGrid;
use fov::FOV;

fn create_fov(b: &mut Bencher) {
    b.iter(|| {
        FOV::<101>::default();
    });
}


fn use_fov(bencher: &mut Bencher) {
    use rand_xorshift::XorShiftRng;

    let d = 100isize;
    let mut state = BitGrid::new(d as usize, d as usize, false);
    let mut rng: XorShiftRng = rand::SeedableRng::seed_from_u64(69_420);
    let fov = FOV::<100>::default();

    for _ in 0..(d * d) {
        let x = rng.gen_range(0, d as isize);
        let y = rng.gen_range(0, d as isize);
        state.set((x, y), true);
    }

    bencher.iter(|| {
        fov.compute_visible(&state, (d / 2, d / 2), (d / 2) as u8);
    });
}

benchmark_group!(benches, create_fov, use_fov);
benchmark_main!(benches);