// Cylindrical Tank Implementation

use std::f32::consts::PI;

pub struct CylinderTank {
    height: f32,
    diameter: f32,
    pub level: f32,
    volume: f32,
    area: f32,
}

impl CylinderTank {
    pub fn new_tank(height: f32, diameter: f32) -> CylinderTank {
        let level = 0.0;
        let area = PI * ((diameter * diameter) / 4.0);
        let volume = area * height;
        CylinderTank {
            height,
            diameter,
            level,
            volume,
            area,
        }
    }

    pub fn fill_volume(&self) -> f32 {
        let volume = self.area * self.level;
        return volume;
    }

    pub fn fill_percent(&self) -> f32 {
        let percent = (self.level / self.height) * 100.0;
        return percent;
    }

    pub fn delta_vol(&mut self, vol_adjust: f32) {
        let fill_vol = self.fill_volume() + vol_adjust;
        self.level = fill_vol / self.area;
    }

    pub fn print_tank(&self) {
        println!("Total Volume: {}", self.volume);
        println!("Fill Level: {}", self.level);
        println!("Filled Volume: {}", self.fill_volume());
        println!("Filled Percent: {}", self.fill_percent());
    }

    pub fn diameter_update(&mut self, new_diameter: f32) {
        self.diameter = new_diameter;
        println!("Diameter updated to {}", self.diameter);
    }
}
