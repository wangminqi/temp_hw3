mod problem1;
mod problem2;
// use problem1::Solution;
use problem1::*;
mod problem3;

fn main() {

    // problem1 test
    let light = problem1::TrafficLight::Yellow;
    println!("Time Left: {}", light.return_time());

    // problem2 test
    println!("sum is {:?}", problem2::sum(&[1,2,3, u32::MAX]));

    // problem3 test
    let sample =  problem3::Rectangle{Height:10, Width:20};
    let sample2 = problem3::Square{Radius: 10};
    println!("area is {:?}", problem3::Area::calculate_area(&sample));
    println!("area is {:?}", problem3::Area::calculate_area(&sample2));

}
