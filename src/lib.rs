#![no_std]


pub struct RingBuffer<T, const S: usize>
where
    T: Default + Copy,
{
    buffer: [T; S],
    head: usize,
    tail: usize,
    len: usize,
}

impl<T, const S: usize> RingBuffer<T, S>
where
    T: Default + Copy,
{
    pub fn new() -> Self {
        RingBuffer {
            buffer: [T::default(); S],
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.len == S {
            return Err("Buffer is full");
        }
        self.buffer[self.head] = item;
        self.head = (self.head + 1) % S;
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let item = self.buffer[self.tail];
        self.tail = (self.tail + 1) % S;
        self.len -= 1;
        Some(item)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == S
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn write(&mut self, data: &[T]) -> Result<(), &'static str> {
        for &item in data {
            self.push(item)?;
        }
        Ok(())
    }

    pub fn read(&mut self, buffer: &mut [T]) -> usize {
        for i in 0..buffer.len() {
            if let Some(item) = self.pop() {
                buffer[i] = item;
            } else {
                return i;
            }
        }
        buffer.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        const SIZE: usize = 4096; // 4KB buffer
        let mut rb: RingBuffer<u32, SIZE> = RingBuffer::new();
        assert!(rb.is_empty());
        assert!(!rb.is_full());

        for i in 0..SIZE {
            rb.push(i as u32).unwrap();
        }
        assert_eq!(rb.len(), SIZE);
        assert!(!rb.is_empty());
        assert!(rb.is_full());

        let mut buffer = [0; 5];
        let read_count = rb.read(&mut buffer);
        assert_eq!(read_count, 5);
        assert_eq!(&buffer[..5], &[0, 1, 2, 3, 4]);

        let read_count = rb.read(&mut buffer);
        assert_eq!(read_count, 5);
        assert_eq!(&buffer[..5], &[5, 6, 7, 8, 9]);

        for _ in 0..rb.len() {
            rb.pop();
        }

        assert!(rb.is_empty());

        assert_eq!(size_of::<RingBuffer<u8, 1>>(), 32);
        assert_eq!(size_of::<RingBuffer<u8, 2>>(), 32); 
        assert_eq!(size_of::<RingBuffer<u8, 4>>(), 32);
        assert_eq!(size_of::<RingBuffer<u8, 8>>(), 32);
        assert_eq!(size_of::<RingBuffer<u8, 16>>(), 40);
        assert_eq!(size_of::<RingBuffer<u8, 32>>(), 56);
        assert_eq!(size_of::<RingBuffer<u8, 64>>(), 88);
        assert_eq!(size_of::<RingBuffer<u8, 128>>(), 152);
        assert_eq!(size_of::<RingBuffer<u8, 256>>(), 280);
        assert_eq!(size_of::<RingBuffer<u8, 512>>(), 536);
        assert_eq!(size_of::<RingBuffer<u8, 1024>>(), 1048);
        assert_eq!(size_of::<RingBuffer<u8, 4096>>(), 4120);
        assert_eq!(size_of::<RingBuffer<u8, 10>>(), 40); // 10 * 4 bytes for u8 + 3 * 4 bytes for usize
    }
}
