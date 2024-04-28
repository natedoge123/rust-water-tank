use std::time::Instant;

pub struct PIDCtrl {
    pub set_point: f32,
    pub process_var: f32,
    pub control_var: f32,
    pub p: f32,
    pub i: f32,
    pub d: f32,
    error: f32,
    error_total: f32,
    error_prev: f32,
    pub last_time: Instant,
}

impl PIDCtrl {
    pub fn new_ctrl(p: f32, i: f32, d: f32) -> PIDCtrl {
        PIDCtrl {
            set_point: 0.0,
            process_var: 0.0,
            control_var: 0.0,
            p,
            i,
            d,
            error: 0.0,
            error_total: 0.0,
            error_prev: 0.0,
            last_time: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        self.error = self.set_point - self.process_var;
        let now = Instant::now();
        let dt = now.duration_since(self.last_time).as_secs_f32();

        self.error_total += self.error * dt;
        let derivative = (self.error - self.error_prev) / dt;

        let pro = self.p * self.error;
        let int = self.i * self.error_total;
        let div = self.d * derivative;

        self.control_var = pro + int + div;

        self.error_prev = self.error;
        self.last_time = now;
    }
}
