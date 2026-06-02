fn maximum_odd_binary(s: &[u8], result: &mut Vec<u8>) {
    let len = s.len();
    let ones: usize = bytecount::count(s, b'1');

    if ones == 0 {
        result.clear();
        return;
    }

    unsafe {
        result.set_len(0);
        result.reserve(len);
        let p: *mut u8 = result.as_mut_ptr();
        std::ptr::write_bytes(p, b'1', ones - 1);
        std::ptr::write_bytes(p.add(ones - 1), b'0', len - ones);
        *p.add(len - 1) = b'1';
        result.set_len(len);
    }
}

fn main() {
    let s = std::fs::read("large_binary.txt").expect("Failed to open large_binary.txt");
    let mut result = Vec::with_capacity(s.len());

    for i in 0..5 {
        let start = std::time::Instant::now();
        maximum_odd_binary(&s, &mut result);
        let elapsed = start.elapsed();
        println!("Run {}: {} µs", i + 1, elapsed.as_micros());
    }
}
