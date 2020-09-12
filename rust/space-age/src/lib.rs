// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Duration { seconds }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! planet_impl {
    ($type:ty, $earth_years:expr) => {
        impl Planet for $type {
            fn years_during(d: &Duration) -> f64 {
                d.seconds as f64 / (31557600.0 * $earth_years)
            }
        }
    };
}

planet_impl!(Mercury, 0.2408467);
planet_impl!(Venus, 0.61519726);
planet_impl!(Earth, 1.0);
planet_impl!(Mars, 1.8808158);
planet_impl!(Jupiter, 11.862615);
planet_impl!(Saturn, 29.447498);
planet_impl!(Uranus, 84.016846);
planet_impl!(Neptune, 164.79132);
