fn maximum_odd_binary(s: &[u8], result: &mut [u8]) {
    let len = s.len();
    let ones: usize = bytecount::count(s, b'1');

    if ones == 0 {
        result.fill(b'0');
        return;
    }

    result[..ones - 1].fill(b'1');
    result[ones - 1..len - 1].fill(b'0');
    result[len - 1] = b'1';
}

fn main() {
    let s = std::fs::read("large_binary.txt").expect("Failed to open large_binary.txt");
    let mut result = vec![0u8; s.len()];

    for i in 0..5 {
        let start = std::time::Instant::now();
        maximum_odd_binary(&s, &mut result);
        let elapsed = start.elapsed();
        println!("Run {}: {} µs", i + 1, elapsed.as_micros());
    }
}
