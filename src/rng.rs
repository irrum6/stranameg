pub mod rng {
    struct RNG {
        w: u32,
        x: u32,
        y: u32,
        z: u32,
    }
    impl RNG {
        fn new() -> RNG {
            RNG {
                w: 0,
                x: 0,
                y: 0,
                z: 0,
            }
        }

        fn seed(&mut self) {
            use std::time::{SystemTime, UNIX_EPOCH};
            self.w = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos();
            self.x = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos();
            self.y = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos();
            self.z = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos();
        }

        fn get(&mut self) -> u32 {
            let tmp: u32 = self.x ^ (self.x << 15);
            self.x = self.y;
            self.y = self.z;
            self.z = self.w;
            self.w = (self.w ^ (self.w >> 21)) ^ (tmp ^ (tmp >> 4));
            return self.w;
        }
    }

    pub struct RNGWheel {
        rng: RNG,
        len: usize,
        index: usize,
    }
    impl RNGWheel {
        pub fn new(l: usize) -> RNGWheel {
            let mut r = RNG::new();
            r.seed();
            return RNGWheel {
                rng: r,
                len: l,
                index: 0,
            };
        }
    }
    impl Iterator for RNGWheel {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.len {
                let result = self.rng.get();
                self.index += 1;
                return Some(result);
            }
            return None;
        }
    }
}
