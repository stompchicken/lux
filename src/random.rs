use std::f32;
use std::mem;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Rng {
    state: u64,
    inc: u64,
}

pub fn thread_rng() -> Rng {
    let start = SystemTime::now();
    match start.duration_since(UNIX_EPOCH) {
        Ok(d) => Rng::new(d.as_secs(), d.subsec_nanos() as u64),
        Err(_) => Rng::new(0,1),
    }
}

impl Rng {

    pub fn new(x1: u64, x2: u64) -> Rng {
        return Rng { state: x1, inc: x2 }
    }

    pub fn gen(&mut self) -> f32 {
        return self.next_f32();
    }

    pub fn next_u32(&mut self) -> u32 {
        let oldstate: u64 = self.state;
        let constant1: u64 = 6364136223846793005;

        self.state = oldstate.wrapping_mul(constant1).wrapping_add(self.inc);

        let shifted: u32 = (oldstate.wrapping_shr(18) ^ oldstate).wrapping_shr(27) as u32;
        let rot1: u32 = oldstate.wrapping_shr(59) as u32;
        let rot2: u32 = (rot1.overflowing_neg().0 & 31) as u32;
        let ret: u32 = shifted.wrapping_shr(rot1) | shifted.wrapping_shl(rot2);
//        println!("oldstate={}, state={}, inc={}, rand={}", oldstate, self.state, self.inc, ret);
        return ret;
    }

    pub fn next_f32(&mut self) -> f32 {
        const UPPER_MASK: u32 = 0x3F800000;
        const LOWER_MASK: u32 = 0x7FFFFF;
        let tmp = UPPER_MASK | (self.next_u32() & LOWER_MASK);
        let result: f32 = unsafe { mem::transmute(tmp) };
        let ret = result - 1.0;
//        println!("next_f32={}", ret);
        return ret;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        let mut rng = thread_rng();
        let n = 1000;
        for _i in 0..n {
            println!("{}", rng.gen());
            assert!(rng.gen() >= 0.0 && rng.gen() < 1.0);
        }
    }
}
