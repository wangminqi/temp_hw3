use std::f64::consts::PI;

pub struct Triangle {
    pub Height: u32,
    pub Width: u32,
}

pub struct Rectangle {
    pub Height: u32,
    pub Width: u32,
}

pub struct Square {
    pub Radius: u32,
}


pub trait Area {
    fn calculate_area(&self) -> f64;
}

impl Area for Triangle {
    fn calculate_area(&self) -> f64 {
        f64::from(&self.Height*&self.Width/2)
    }
}

impl Area for Rectangle {
    fn calculate_area(&self) -> f64 {
        f64::from(&self.Height*&self.Width)
    }
}

impl Area for Square {
    fn calculate_area(&self) -> f64 {
        PI * f64::from(self.Radius.pow(2))
    }
}
