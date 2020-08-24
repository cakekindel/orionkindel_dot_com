use crate::convert;

pub struct TimeDelta(f64);
convert!(impl From<f64> for newtype TimeDelta {});
