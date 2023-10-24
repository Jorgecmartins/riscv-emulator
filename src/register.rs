pub struct Register {
    value: u32,
    register: u8,
    name: String,
}

impl Register {
    pub fn new(value: u32, register: u8, name: String) -> Self {
        Self {
            value,
            register,
            name,
        }
    }

    pub fn set_value(&mut self, value: u32) {
        // x0 is always zero, jalr sets the result in x0 as a way of ignoring
        if self.register != 0 {
            self.value = value
        }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }
}
