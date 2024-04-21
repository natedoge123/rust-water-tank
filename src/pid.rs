pub struct PIDCtrl {
    set_point: f32,
    process_var: f32,
    p: f32,
    i: f32,
    d: f32,
    error: f32,
    error_total: f32,
    delta_t: f32,
    
}

impl PIDCtrl {
    pub fn new_ctrl(p: f32, i: f32, d: f32) -> PIDCtrl {
        let set_point = 0.0;
        let process_var = 0.0;
        let error = 0.0;
        let error_total = 0.0;
        let delta_t = 0.0;
        PIDCtrl {
            set_point,
            process_var,
            p,
            i,
            d,
            error,
            error_total,
            delta_t,
        }
    }

    pub fn update(&self) {
        self.error = 

    }
}
