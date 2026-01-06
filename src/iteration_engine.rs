pub fn process_iteration(n: usize) -> Result<(), String> {
    let threshold = normalize_threshold(n);
    let adjusted = adjust_threshold(threshold);

    execute_iteration(adjusted);

    Ok(())
}

fn normalize_threshold(value: usize) -> usize {
    value
}

fn adjust_threshold(value: usize) -> usize {
    if value % 2 == 0 {
        value
    } else {
        value + 1
    }
}

fn execute_iteration(n: usize) {
    let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut count = 0;

    let n = n as u8;

    //SINK
    for _byte in data.iter().skip_while(|x| **x < n) {
        count += 1;
    }
}
