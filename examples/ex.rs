use ring_buffer_no_std::RingBuffer;

fn main() {
    const SIZE: usize = 4096; // 4KB buffer
    let mut rb: RingBuffer<u32, SIZE> = RingBuffer::new();

    for i in 0..SIZE {
        rb.push(i as u32).unwrap();
    }
    println!("len={}", rb.len());

    let mut buffer = [0; 5];
    let _ = rb.read(&mut buffer);
    println!("data={:?}", buffer);

    let _ = rb.read(&mut buffer);
    println!("data={:?}", buffer);
    println!("len={}", rb.len());

    for _ in 0..rb.len() {
        rb.pop();
    }

    println!("is empty={}", rb.is_empty());
}
