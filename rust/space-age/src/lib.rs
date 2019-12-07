#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    fn year() -> f64;
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (31_557_600_f64 * Self::year())
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn year() -> f64 {
        0.240_846_7
    }
}
impl Planet for Venus {
    fn year() -> f64 {
        0.615_197_26
    }
}
impl Planet for Earth {
    fn year() -> f64 {
        1.0
    }
}
impl Planet for Mars {
    fn year() -> f64 {
        1.880_815_8
    }
}
impl Planet for Jupiter {
    fn year() -> f64 {
        11.862_615
    }
}
impl Planet for Saturn {
    fn year() -> f64 {
        29.447_498
    }
}
impl Planet for Uranus {
    fn year() -> f64 {
        84.016_846
    }
}
impl Planet for Neptune {
    fn year() -> f64 {
        164.79132
    }
}
