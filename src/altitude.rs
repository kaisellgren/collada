#[derive(Debug)]
pub enum AltitudeMode { Absolute, RelativeToGround }

#[derive(Debug)]
pub struct Altitude(f64, AltitudeMode);
