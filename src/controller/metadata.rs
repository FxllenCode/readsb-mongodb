pub struct Metadata {
    time_to_check: u64,
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata {
            time_to_check: 30 * 1000,
        }
    }

    pub fn get_time_to_check(&self) -> u64 {
        self.time_to_check
    }

    pub fn set_time_to_check(&mut self, time: u64) {
        self.time_to_check = time;
    }
}