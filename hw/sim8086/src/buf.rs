#[derive(Debug)]
pub enum BufferError {
    ReadOverrun,
}

pub struct Buf {
    bytes: Vec<u8>,
    pos: usize,
}

impl Buf {
     pub fn new(bytes: Vec<u8>) -> Buf {
        Buf {
            bytes,
            pos: 0,
        }
    }

    pub fn is_at_end(&self) -> bool {
        return self.pos >= self.bytes.len();
    }

    pub fn step(&mut self, steps: usize) {
        self.pos += steps;
    }

    pub fn read_u8(&mut self) -> Result<u8, BufferError> {
        if self.pos >= self.bytes.len() {
            return Err(BufferError::ReadOverrun);
        }

        let res: u8 = self.bytes[self.pos];
        self.step(1);
        return Ok(res);
    }

    pub fn read_i8(&mut self) -> Result<i16, BufferError> {
        if self.pos >= self.bytes.len() {
            return Err(BufferError::ReadOverrun);
        }

        let res: i16 = ((self.bytes[self.pos] as i16) << 8) >> 8;
        self.step(1);
        return Ok(res);
    }

    pub fn read_u16(&mut self) -> Result<u16, BufferError> {
        if self.pos + 1 >= self.bytes.len() {
            return Err(BufferError::ReadOverrun);
        }

        let mut res: u16 = self.bytes[self.pos] as u16;
        self.step(1);
        res |= (self.bytes[self.pos] as u16) << 8;
        self.step(1);
        return Ok(res);
    }

    pub fn read_i16(&mut self) -> Result<i16, BufferError> {
        if self.pos + 1 >= self.bytes.len() {
            return Err(BufferError::ReadOverrun);
        }

        let mut res: i16 = self.bytes[self.pos] as i16;
        self.step(1);
        res |= (self.bytes[self.pos] as i16) << 8;
        self.step(1);
        return Ok(res);
    }
}