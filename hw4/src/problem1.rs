
pub trait Solution {
    fn return_time(&self) -> u32;
}

impl Solution for TrafficLight {
    fn return_time(&self) -> u32 {
        match &self {
            TrafficLight::Red=> 100,
            TrafficLight::Green=> 0,
            TrafficLight::Yellow=> 50,
        }
    }
}

pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}