mod pid;
mod tank;

use std::f32;

fn main() {
    let mut vol_adjust = 1.0;
    let mut tank_1 = tank::CylinderTank::new_tank(10.0, 2.0);
    let mut pid_1 = pid::PIDCtrl::new_ctrl(1.0, 0.0, 0.0);
    tank_1.print_tank();
    loop {
        tank_1.delta_vol(vol_adjust);
        println!("<><><><><><><><><><><><>");
        tank_1.print_tank();

        if tank_1.fill_percent() > 100.0 {
            println!("tank overflow!");
            break;
        }
    }
}
