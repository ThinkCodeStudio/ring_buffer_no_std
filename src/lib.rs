#![no_std]

///
/// 
/// A simple ring buffer implementation in Rust.
/// This ring buffer supports fixed-size storage and provides methods for pushing, popping, and reading data.
/// It is designed to be efficient and easy to use, with a focus on performance and safety.
///
/// # Example
/// ```
/// use ring_buffer_no_std::RingBuffer;
/// let mut rb: RingBuffer<u32, 4096> = RingBuffer::new();
/// rb.push(42).unwrap();
/// let item = rb.pop();
/// assert_eq!(item, Some(42));
/// ```
/// 

#[derive(Debug)]
pub enum RingBufferError {
    Full,
    BeyondRange,
    BufferIncomplete,
}

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
    /**
     * Creates a new instance of `RingBuffer` with a fixed size.
     *  ```
     * use ring_buffer_no_std::RingBuffer;
     * let mut rb: RingBuffer<u32, 4096> = RingBuffer::new();
     * ```
     */
    pub fn new() -> Self {
        RingBuffer {
            buffer: [T::default(); S],
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    /**
     * Pushes an item into the ring buffer.
     * Returns an error if the buffer is full.
     * ```
     * use ring_buffer_no_std::RingBuffer;
     * let mut rb: RingBuffer<u32, 4096> = RingBuffer::new();
     * rb.push(42).unwrap();
     * ```
     */
    pub fn push(&mut self, item: T) -> Result<(), RingBufferError> {
        if self.len == S {
            return Err(RingBufferError::Full);
        }
        self.buffer[self.head] = item;
        self.head = (self.head + 1) % S;
        self.len += 1;
        Ok(())
    }

    /**
     * Pops an item from the ring buffer.
     * Returns `None` if the buffer is empty.
     * ```
     * use ring_buffer_no_std::RingBuffer;
     * let mut rb: RingBuffer<u32, 4096> = RingBuffer::new();
     * rb.push(42).unwrap();
     * let item = rb.pop();
     * assert_eq!(item, Some(42));
     * ```
     */
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let item = self.buffer[self.tail];
        self.tail = (self.tail + 1) % S;
        self.len -= 1;
        Some(item)
    }

    /**
     * Clears the ring buffer, resetting its state.
     * ```
     * use ring_buffer_no_std::RingBuffer;
     * let mut rb: RingBuffer<u32, 4096> = RingBuffer::new();
     * rb.push(42).unwrap();
     * rb.clear();
     * assert!(rb.is_empty());
     * ```
     */
    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.len = 0;
    }

    /**
     * Pops a specified number of items from the ring buffer.
     * Returns an error if the count exceeds the current length of the buffer.
     * ```
     * use ring_buffer_no_std::RingBuffer;
     * let mut rb: RingBuffer<u32, 4096> = RingBuffer::new();
     * rb.push(1).unwrap();
     * rb.push(2).unwrap();
     * let remaining_capacity = rb.pop_continuous(1).unwrap();
     * assert_eq!(remaining_capacity, 4095);
     * ```
     */
    pub fn pop_continuous(&mut self, count: usize) -> Result<usize, RingBufferError> {
        if count > self.len {
            return Result::Err(RingBufferError::BeyondRange);
        }
        self.tail = (self.tail + count) % S;
        self.len -= count;

        Ok(self.remaining_capacity())
    }

    /**
     * Returns the capacity of the ring buffer.
     * ```
     * use ring_buffer_no_std::RingBuffer;
     * let mut rb: RingBuffer<u32, 4096> = RingBuffer::new();
     * assert_eq!(rb.capacity(), 4096);
     * ```
     */
    #[inline]
    pub fn capacity(&self) -> usize {
        S
    }

    /**
     * Returns the remaining capacity of the ring buffer.
     * ```
     * use ring_buffer_no_std::RingBuffer;
     * let mut rb: RingBuffer<u32, 4096> = RingBuffer::new();
     * assert_eq!(rb.remaining_capacity(), 4096);
     * rb.push(1).unwrap();
     * assert_eq!(rb.remaining_capacity(), 4095);
     * ```
     */
    #[inline]
    pub fn remaining_capacity(&self) -> usize {
        S - self.len
    }

    /**
     * Checks if the ring buffer is empty.
     */
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    /**
     * Checks if the ring buffer is full.
     */
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len == S
    }
    /**
     * Returns the current length of the ring buffer.
     */
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /**
     * Writes a slice of data into the ring buffer.
     * Returns an error if the buffer is full.
     * ```
     * use ring_buffer_no_std::RingBuffer;
     * let mut rb: RingBuffer<u32, 4096> = RingBuffer::new();
     * rb.write(&[1, 2, 3, 4]).unwrap();
     * ``` 
     */
    pub fn write(&mut self, data: &[T]) -> Result<(), RingBufferError> {
        for &item in data {
            self.push(item)?;
        }
        Ok(())
    }

    /**
     * Reads data from the ring buffer into a provided mutable slice.
     * Returns the number of items read.
     * If the buffer has fewer items than the slice, it fills as many as possible.
     * ```
     * use ring_buffer_no_std::RingBuffer;
     * let mut rb: RingBuffer<u32, 4096> = RingBuffer::new();
     * rb.push(1).unwrap();
     * rb.push(2).unwrap();
     * let mut buffer = [0; 5];
     * let read_count = rb.read(&mut buffer);
     * assert_eq!(read_count, 2);
     * assert_eq!(&buffer[..read_count], &[1, 2]);
     * ```
     */
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

    
    /**
     * Reads a slice of data from the ring buffer.
     * Returns an error if the requested length exceeds the current length of the buffer.
     * If the data is contiguous, it returns a slice; otherwise, it returns an error.
     * ```
     * use ring_buffer_no_std::RingBuffer;
     * let mut rb: RingBuffer<u32, 4096> = RingBuffer::new();
     * rb.push(1).unwrap();
     * rb.push(2).unwrap();
     * let slice = rb.read_slice(2).unwrap();
     * assert_eq!(slice, &[1, 2]);
     * ```  
     */
    pub fn read_slice(&self, len: usize) -> Result<&[T], RingBufferError> {
        if len > self.len {
            return Err(RingBufferError::BeyondRange);
        }
        if self.tail < self.head {
            // Data is contiguous
            Ok(&self.buffer[self.tail..self.tail + len])
        } else if self.tail + len <= S {
            // Data is contiguous from tail to end of buffer
            Ok(&self.buffer[self.tail..self.tail + len])
        } else {
            // Data wraps around, cannot return as a single slice
            Err(RingBufferError::BufferIncomplete)
        }
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
        assert_eq!(rb.len(), SIZE - 10);
        assert!(!rb.is_full());
        assert!(!rb.is_empty());
        assert_eq!(rb.remaining_capacity(), SIZE - rb.len());

        rb.clear();
        assert!(rb.is_empty());

        for i in 0..SIZE {
            rb.push(i as u32).unwrap();
        }
        let slice = rb.read_slice(10).unwrap();
        assert_eq!(rb.len(), SIZE);
        assert_eq!(slice, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let remaining_capacity = rb.pop_continuous(10).unwrap();
        assert_eq!(remaining_capacity, 10);
        assert_eq!(rb.len(), SIZE - 10);
        let slice = rb.read_slice(10).unwrap();
        assert_eq!(slice, &[10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
        assert_eq!(rb.len(), SIZE - 10);


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
