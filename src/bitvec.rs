#[derive(Clone)]
struct BitVec {
    vec: Vec<u64>,
}

impl BitVec {
    fn new(size: usize, val: bool) -> BitVec {
        let size = {
            if size % 64 == 0 {
                size / 64
            } else {
                size / 64 + 1
            }
        };

        let val = if val { 0xffffffffffffffff } else { 0 };
        let vec = vec![val; size];

        BitVec { vec }
    }

    fn get(&self, ix: usize) -> bool {
        unsafe {
            // Index of 64 bit chunk
            let jx = ix >> 6;
            // Get the 64 bit chunk
            let kx = *self.vec.get_unchecked(jx);
            // Get the actual bit
            let lx = kx & (1 << (ix & 63));
            lx > 0
        }
    }

    fn set(&mut self, ix: usize, v: bool) {
        unsafe {
            let jx = ix >> 6;
            let kx = *self.vec.get_unchecked(jx);
            let lx = if v {
                kx | (1 << (ix & 63))
            } else {
                kx & !(1 << (ix & 63))
            };

            *self.vec.get_unchecked_mut(jx) = lx;
        }
    }

    fn bitwise_or(&mut self, other: &BitVec) {
        for i in 0..self.vec.len() {
            let a = self.vec[i];
            let b = other.vec[i];

            unsafe {
                *self.vec.get_unchecked_mut(i) = a | b;
            }
        }
    }

    fn bitwise_and(&mut self, other: &BitVec) {
        for i in 0..self.vec.len() {
            let a = self.vec[i];
            let b = other.vec[i];

            unsafe {
                *self.vec.get_unchecked_mut(i) = a & b;
            }
        }
    }
}

#[derive(Clone)]
pub struct BitGrid {
    w: isize,
    h: isize,
    vec: BitVec,
}

impl BitGrid {
    pub fn new(w: usize, h: usize, val: bool) -> BitGrid {
        BitGrid {
            w: w as isize,
            h: h as isize,
            vec: BitVec::new(w * h, val),
        }
    }

    pub fn w(&self) -> isize {
        self.w
    }

    pub fn h(&self) -> isize {
        self.h
    }

    pub fn get(&self, (x, y): (isize, isize)) -> bool {
        x >= 0 && y >= 0 && x < self.w && y < self.h && self.vec.get((y * self.w + x) as usize)
    }

    pub fn set(&mut self, (x, y): (isize, isize), v: bool) {
        if x >= 0 && y >= 0 && x < self.w && y < self.h {
            self.vec.set((y * self.w + x) as usize, v)
        }
    }

    pub fn bitwise_and(&mut self, other: &BitGrid) {
        if self.w != other.w || self.h != other.h {
            panic!("bitwise_and: BitGrid is different width or height.");
        } else {
            self.vec.bitwise_and(&other.vec);
        }
    }

    pub fn bitwise_or(&mut self, other: &BitGrid) {
        if self.w != other.w || self.h != other.h {
            panic!("bitwise_or: BitGrid is different width or height.");
        } else {
            self.vec.bitwise_or(&other.vec);
        }
    }
}