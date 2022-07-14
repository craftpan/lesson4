pub fn print() {
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;
    println!("traffic light color: {:?}, time:{:?}s", red, red.red_time());
    println!("traffic light color: {:?}, time:{:?}s", green, green.green_time());
    println!("traffic light color: {:?}, time:{:?}s", yellow, yellow.yellow_time());
}

#[derive(Debug)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}
trait TrafficLightTrait {
    fn red_time(&self) -> u32;
    fn green_time(&self) -> u32;
    fn yellow_time(&self) -> u32;
}

impl TrafficLightTrait for TrafficLight {
    fn red_time(&self) -> u32 {
        60
    }
    fn green_time(&self) -> u32 {
        70
    }
    fn yellow_time(&self) -> u32 {
        5
    }
}