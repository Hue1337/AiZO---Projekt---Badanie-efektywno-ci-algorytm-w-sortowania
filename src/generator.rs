use rand::Rng;

pub struct Generator {
    pub arr_size: u32,
    pub arr_i32: Vec<i32>,
    pub arr_f32: Vec<f32>,
    pub arr_f64: Vec<f64>,
}
impl Generator {
    pub fn run(&mut self, data_type: u32) {
        match data_type {
            1 => {
                // Integer
                self.generate_data_i32();
            },

            2 => {
                // Float
                self.generate_data_f32();
            },

            3 => {
                // Double
                self.generate_data_f64();
            },

            _ => {
                println!("[-] Error occurred! Failed to generate data. Wrong DATA_TYPE!");
            }
        }
    }

    pub fn new() -> Generator {
        Generator {
            arr_size: 0,
            arr_i32: Vec::new(),
            arr_f32: Vec::new(),
            arr_f64: Vec::new(),
        }
    }

    fn generate_data_i32(&mut self) {
        self.arr_i32.clear();
        let mut rng = rand::thread_rng();
        for i in 0 .. self.arr_size {
            self.arr_i32.push(rng.gen_range(0..i32::MAX/2));
        }
    }
    
    fn generate_data_f32(&mut self) {
        self.arr_f32.clear();
        let mut rng = rand::thread_rng();
        for i in 0 .. self.arr_size {
            self.arr_f32.push(rng.gen_range(0.0..f32::MAX/2.0));
        }
    }

    fn generate_data_f64(&mut self) {
        self.arr_f64.clear();
        let mut rng = rand::thread_rng();
        for i in 0 .. self.arr_size {
            self.arr_f64.push(rng.gen_range(0.0..f64::MAX/2.0));
        }
    }


    // Getters and setters
    pub fn get_arr_i32(&self) -> Vec<i32> {
        return self.arr_i32.clone();
    }

    pub fn get_arr_f32(&self) -> Vec<f32> {
        return self.arr_f32.clone();
    }

    pub fn get_arr_f64(&self) -> Vec<f64> {
        return self.arr_f64.clone();
    }

    pub fn set_arr_size(&mut self, size_tmp: u32) {
        self.arr_size = size_tmp;
    }
}
