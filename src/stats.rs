use std::f32;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Stats {
    pub begin_time: SystemTime,
    pub end_time: SystemTime,

    pub rays: u64,
    pub hit_emitter: u64,
    pub hit_maxdepth: u64,
    pub hit_nothing: u64,
}


impl Stats {

    pub fn new(begin: SystemTime) -> Stats {
        Stats {
            begin_time: begin,
            end_time: UNIX_EPOCH,
            rays: 0,
            hit_emitter: 0,
            hit_maxdepth: 0,
            hit_nothing: 0
        }
    }

    pub fn duration(&self) -> f32 {
        let duration = self.end_time.duration_since(self.begin_time);
        match duration {
            Ok(d) => (d.as_secs() as f32),
            Err(_) => f32::NAN
        }
    }

    pub fn mega_rays_per_second(&self) -> f32 {
        let duration = self.end_time.duration_since(self.begin_time);
        match duration {
            Ok(d) => (self.rays as f32) / ((d.as_secs() as f32) * 1000.0),
            Err(_) => f32::NAN
        }
    }

    pub fn hits(&self) -> u64 {
        return self.hit_emitter + self.hit_maxdepth + self.hit_nothing;
    }
}
