#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

pub struct CircularBuffer<T:Clone> {
    buffer: Vec<Option<T>>,
    size: usize,
    next: usize,
}

impl<T:Clone> CircularBuffer<T> {
    pub fn new(size: usize) -> CircularBuffer<T> {
        CircularBuffer {
            buffer: vec![None; size],
            size: 0,
            next: 0,
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error:: EmptyBuffer);
        }else{
            // Calculate the index of the oldest element
            let index = (self.buffer.len() + self.next - self.size) % self.buffer.len();
            // Retrieve the element at the calculated index
            let result = self.buffer[index].clone();
            // Decrease the size of the buffer
            self.size -= 1;
            match result {
                Some(res) => Ok(res),
                None => Err(Error:: EmptyBuffer)
            }
        }
    }

    pub fn write(&mut self, byte: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error:: FullBuffer);
        }else{
            // Insert the byte at the next position
            self.buffer[self.next] = Some(byte);
            // Update the next position, wrapping around if necessary
            self.next = (self.next + 1) % self.buffer.len();
            // Increase the size of the buffer
            self.size += 1;
            // Return Ok to indicate success
            Ok(())
        }
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.next = 0;
        self.buffer = vec![None; self.buffer.len()];
    }

    pub fn is_empty(&self) -> bool {
        return self.size == 0;
    }

    pub fn is_full(&self) -> bool {
        return self.size == self.buffer.len();
    }
}
