mod pid;
mod tank;

use crate::tank;
use std::f32;

struct PidCtrl {
    set_point: f32,
    process_var: f32,
    p: f32,
    i: f32,
    d: f32,
}

fn main() {
    let mut vol_adjust = 0.0;
    pub mut tank = CylinderTank::new_tank(10.0, 2.0);
    tank.print_tank();
    loop {
        vol_adjust += 0.01;
        tank.delta_vol(vol_adjust);
        println!("<><><><><><><><><><><><>");
        tank.print_tank();

        if tank.fill_percent() > 100.0 {
            println!("tank overflow!");
            break;
        }
    }
}
