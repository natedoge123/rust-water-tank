use std::f32::consts::PI;

struct CylinderTank {
    height: f32,
    diameter: f32,
    level: f32,
}

impl CylinderTank {
    fn new_tank(height: f32, diameter: f32, level: f32) -> CylinderTank {
        CylinderTank {
            height,
            diameter,
            level,
        }
    }

    fn volume(&self) -> f32 {
        let area = PI * ((self.diameter * self.diameter) / 4.0);
        let volume = area * self.height;
        volume
    }

    fn fill_volume(&self) -> f32 {
        let area = PI * ((self.diameter * self.diameter) / 4.0);
        let volume = area * self.level;
        volume
    }
}

fn main() {
    let tank = CylinderTank::new_tank(10.0, 2.0, 1.0);

    println!("The volume of the tank is {}", tank.fill_volume());
}
