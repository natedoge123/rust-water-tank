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
        return volume;
    }

    fn fill_volume(&self) -> f32 {
        let area = PI * ((self.diameter * self.diameter) / 4.0);
        let volume = area * self.level;
        return volume;
    }

    fn fill_perc(&self) -> f32 {
        let total = self.volume();
        let filled = self.fill_volume();
        let percentage = (filled / total) * 100.0;
        return percentage;
    }

    fn vol_delta(&mut self, vol_adjust: f32) -> f32 {
        let area = PI * ((self.diameter * self.diameter) / 4.0);
        let filled = self.fill_volume() + vol_adjust;
        self.level = filled / area;
        return self.level;
    }
}

fn main() {
    let mut tank = CylinderTank::new_tank(10.0, 2.0, 0.0);
    println!("Volume {}", tank.fill_volume());
    tank.vol_delta(1.0);
    println!("Volume {}", tank.fill_volume());
}
