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




fn execute_iteration(n: usize) -> Result<String, String> {
    
    
    //SINK
    if let Some(ch) = std::iter::repeat('x').nth(n) {
        return Ok(format!("Char: {}", ch));
    }

    Err("No char".into())
}
