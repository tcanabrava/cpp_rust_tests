fn maximum_odd_binary(s: &[u8]) -> Vec<u8> {
    let len = s.len();

    let ones: usize = bytecount::count(s, b'1');

    if ones == 0 {
        return Vec::new();
    }

    // Write each output byte exactly once (no double-memset).
    // vec![b'1'; len] + fill(b'0') writes 1.5× the buffer; this writes 1×.
    let mut result = Vec::with_capacity(len);
    unsafe {
        let p: *mut u8 = result.as_mut_ptr();
        std::ptr::write_bytes(p, b'1', ones - 1);
        std::ptr::write_bytes(p.add(ones - 1), b'0', len - ones);
        *p.add(len - 1) = b'1';
        result.set_len(len);
    }
    result
}

fn main() {
    let s = std::fs::read("large_binary.txt").expect("Failed to open large_binary.txt");

    for i in 0..5 {
        let start = std::time::Instant::now();
        let _result = maximum_odd_binary(&s);
        let elapsed = start.elapsed();
        println!("Run {}: {} µs", i + 1, elapsed.as_micros());
    }
}
