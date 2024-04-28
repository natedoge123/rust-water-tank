mod pid;
mod tank;

fn main() {
    let mut cycle = 0;
    let flow_in = 10.0;
    let mut flow_out = 0.0;
    let mut tank_1 = tank::CylinderTank::new_tank(1000.0, 2.0);
    let mut pid_1 = pid::PIDCtrl::new_ctrl(10.0, 0.01, 0.0);
    tank_1.print_tank();

    //  Declare all of the PID variables and tie them to the tank
    pid_1.set_point = 500.0;

    loop {
        // Update PID varibles per cycle
        pid_1.process_var = tank_1.level;
        flow_out = pid_1.control_var;

        tank_1.delta_vol(flow_in);
        tank_1.delta_vol(flow_out);
        println!("<><><><><><><><><><><><>");
        tank_1.print_tank();

        if tank_1.fill_percent() > 100.0 {
            println!("tank overflow!");
            break;
        }

        cycle += 1;
        println!("{} Cycle", cycle);
        pid_1.update()
    }
}
