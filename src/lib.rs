#![feature(min_const_generics)]

mod bitvec;

pub use bitvec::BitGrid;

#[derive(Clone)]
pub struct FOV<const R: usize> {
    shading: Vec<BitGrid>,
    range_shading: Vec<BitGrid>,
}

impl<const R: usize> FOV<R> {
    pub fn new(permissive: bool) -> FOV<R> {
        let ri = R as isize;
        let mut shading = Vec::with_capacity(R * R);
        let mut range_shading = Vec::with_capacity(R);

        fn range((x0, y0): (isize, isize), (x1, y1): (isize, isize)) -> isize {
            let dx = x0 - x1;
            let dy = y0 - y1;

            dx * dx + dy * dy
        }

        for y in 0..ri {
            for x in 0..ri {
                shading.push(BitGrid::new(R, R, false));

                // Trace from outer edges (eliminates redundant tracing from inner points)
                if !permissive || x == (ri - 1) || y == (ri - 1) {
                    let line = trace((x, y), (0, 0));

                    // For each point on line shade along line to that point
                    for i in 0..line.len() {
                        let (px, py) = (line[i].0 as usize, line[i].1 as usize);
                        let ix = py * R + px;

                        for &xy in line.iter().take(i) {
                            shading[ix].set(xy, true);
                        }
                    }
                }
            }
        }

        for i in 1..=ri {
            let mut range_shade = BitGrid::new(R, R, false);
            for y in 0..ri {
                for x in 0..ri {
                    if range((0, 0), (x, y)) >= i * i {
                        range_shade.set((x, y), true);
                    }
                }
            }
            range_shading.push(range_shade);
        }

        FOV {
            shading,
            range_shading,
        }
    }

    // Given a point and a BitGrid where 'true' values are fov blockers,
    // returns an iterator over visible tiles.
    pub fn compute(&self, state: &BitGrid, (x, y): (isize, isize), r: usize) -> FOVIter<R> {
        if r == 0 {
            panic!("You can't compute a FOV that is 0.")
        }

        if r > R.into() {
            panic!("You tried to compute a FOV larger than you have instantiated.")
        }

        let mut ne = self.range_shading[r - 1].clone();
        let mut se = ne.clone();
        let mut sw = ne.clone();
        let mut nw = ne.clone();
        let ri = R as isize;

        for yo in 0..ri {
            for xo in 0..ri {
                let xoyo = (xo, yo);
                let shade = &self.shading[(yo * ri + xo) as usize];

                if !ne.get(xoyo) && state.get((x + xo, y + yo)) {
                    ne.bitwise_or(shade);
                }

                if !nw.get(xoyo) && state.get((x - xo, y + yo)) {
                    nw.bitwise_or(shade);
                }

                if !se.get(xoyo) && state.get((x + xo, y - yo)) {
                    se.bitwise_or(shade);
                }

                if !sw.get(xoyo) && state.get((x - xo, y - yo)) {
                    sw.bitwise_or(shade);
                }
            }
        }

        FOVIter {
            ne,
            se,
            sw,
            nw,
            mid_x: x,
            mid_y: y,
            min_x: x - ri + 1,
            max_x: x + ri - 1,
            max_y: y + ri - 1,
            x: x - ri + 1,
            y: y - ri + 1,
        }
    }
}

pub struct FOVIter<const R: usize> {
    ne: BitGrid,
    se: BitGrid,
    sw: BitGrid,
    nw: BitGrid,
    mid_x: isize,
    mid_y: isize,
    min_x: isize,
    max_x: isize,
    max_y: isize,
    x: isize,
    y: isize,
}

impl<const R: usize> Iterator for FOVIter<R> {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.y > self.max_y {
                return None;
            }
            let xy = (self.x, self.y);
            let blocked = 
            if self.x >= self.mid_x {
                if self.y >= self.mid_y {
                    self.ne.get((self.x - self.mid_x, self.y - self.mid_y))
                } else {
                    self.se.get((self.x - self.mid_x, self.mid_y - self.y))
                }
            } else {
                if self.y >= self.mid_y {
                    self.nw.get((self.mid_x - self.x, self.y - self.mid_y))
                } else {
                    self.sw.get((self.mid_x - self.x, self.mid_y - self.y))
                }
            };

            self.x += 1;
            if self.x > self.max_x {
                self.x = self.min_x;
                self.y += 1;
            }

            if !blocked {
                return Some(xy);
            }
        }
    }
}

fn trace((mut x0, mut y0): (isize, isize), (x1, y1): (isize, isize)) -> Vec<(isize, isize)> {
    let mut vec = Vec::new();
    // Absolute x,y offset
    let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
    let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };

    // Slopes
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    // Initialize error
    let mut err = if dx > dy { dx } else { -dy } / 2;
    let mut err2;

    loop {
        vec.push((x0, y0));
        // Check end condition
        if x0 == x1 && y0 == y1 {
            return vec;
        };

        // Store old error
        err2 = 2 * err;

        if err2 > -dx {
            err -= dy;
            x0 += sx;
        }

        if err2 < dy {
            err += dx;
            y0 += sy;
        }
    }
}
