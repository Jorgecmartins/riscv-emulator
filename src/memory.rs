pub struct Memory {
    start: usize,
    data: Vec<u8>,
}

impl Memory {
    pub fn new(start: usize, data: Vec<u8>) -> Self {
        Self { start, data }
    }

    pub fn belongs(&self, address: usize, nb_bytes: usize) -> bool {
        address >= self.start
            && nb_bytes <= self.data.len() // sanity check
            && address + nb_bytes  <= self.start + self.data.len()
    }

    pub fn write_8(&mut self, address: usize, value: u8) {
        assert!(address >= self.start);

        let offset = address - self.start;

        assert!(offset < self.data.len());

        self.data[offset] = value;
    }

    pub fn read_u8(&self, address: usize) -> u8 {
        assert!(address >= self.start);

        let offset = address - self.start;

        assert!(offset < self.data.len());

        self.data[offset]
    }

    pub fn write_16(&mut self, address: usize, value: u16) {
        assert!(address >= self.start);

        let offset = address - self.start;

        assert!(offset + 1 < self.data.len());

        self.data[offset] = (value & 0xff) as u8;
        self.data[offset + 1] = ((value & 0xff00) >> 8) as u8;
    }

    pub fn read_u16(&self, address: usize) -> u16 {
        assert!(address >= self.start);

        let offset = address - self.start;

        assert!(offset + 1 < self.data.len());

        (self.data[offset + 1] as u16) << 8 | self.data[offset] as u16
    }

    pub fn write_u32(&mut self, address: usize, value: u32) {
        assert!(address >= self.start);

        let offset = address - self.start;

        assert!(offset + 3 < self.data.len());

        self.data[offset] = (value & 0xff) as u8;
        self.data[offset + 1] = ((value & 0xff00) >> 8) as u8;
        self.data[offset + 2] = ((value & 0xff0000) >> (8 * 2)) as u8;
        self.data[offset + 3] = ((value & 0xff000000) >> (8 * 3)) as u8;
    }

    pub fn read_u32(&self, address: usize) -> u32 {
        assert!(address >= self.start);

        let offset = address - self.start;

        assert!(offset + 3 < self.data.len());

        (self.data[offset + 3] as u32) << (8 * 3)
            | (self.data[offset + 2] as u32) << (8 * 2)
            | (self.data[offset + 1] as u32) << 8
            | self.data[offset] as u32
    }

    pub fn write_n(&mut self, address: usize, bytes: Vec<u8>) {
        assert!(address >= self.start);

        let offset = address - self.start;

        assert!(offset + bytes.len() - 1 < self.data.len());

        for (i, byte) in bytes.iter().enumerate() {
            self.data[offset + i] = *byte;
        }
    }

    pub fn read_n(&self, address: usize, size: usize) -> Vec<u8> {
        assert!(address >= self.start);

        let offset = address - self.start;

        assert!(offset + size - 1 < self.data.len());

        let mut bytes = Vec::new();

        for i in 0..size {
            bytes.push(self.data[offset + i]);
        }

        bytes
    }

    // pub fn read_str(&self, address: usize) -> Vec<u8> {
    //     assert!(address >= self.start);
    //     assert!(address <= self.start + self.data.len());

    //     let mut bytes = Vec::new();
    //     let mut offset = address - self.start;
    //     loop {
    //         if offset >= self.data.len() {
    //             panic!("string goes beyong memory boundaries");
    //         }

    //         let byte = self.data[offset];
    //         if byte == 0 {
    //             break;
    //         }

    //         bytes.push(byte);

    //         offset += 1;
    //     }

    //     bytes
    // }
}