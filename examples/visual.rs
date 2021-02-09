use fov::BitGrid;
use fov::FOV;
use rand::Rng;

pub fn main() {
    const DIAM: usize = 40usize;
    const RADIUS: u8 = (DIAM / 2 - 1) as u8;
    let d = DIAM as isize;
    let fov = FOV::<RADIUS>::new();
    let mut state = BitGrid::new(DIAM, DIAM, false);
    let mut vision = BitGrid::new(DIAM, DIAM, false);
    let mut rng = rand::thread_rng();
    
    let xy = (d / 2, d / 2);

    for _ in 0..d * d / 32 {
        let x = rng.gen_range(0, d);
        let y = rng.gen_range(0, d);
        // Set LOS Blockers
        state.set((x, y), true);
    }
    state.set(xy, false);

    for (x, y) in fov.compute(&state, xy, RADIUS) {
        // Set visible tiles
        vision.set((x, y), true);
    }

    print!("---------------------------------------------------------------");
    for y in 0..d {
        println!();
        for x in 0..d {
            if (x, y) == xy {
                print!("O ");
            } else if state.get((x, y)) {
                print!("\u{25A0} ");
            } else if vision.get((x, y)) {
                print!("  ");
            } else {
                print!("x ");
            }
        }
    }
}
