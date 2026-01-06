pub fn process_arithmetic(b: i32) -> Result<(), String> {
    let value = normalize_value(b);
    let adjusted = adjust_value(value);

    execute_operation(adjusted);

    Ok(())
}

fn normalize_value(v: i32) -> i32 {
    v
}

fn adjust_value(v: i32) -> i32 {
    if v < 0 { -v } else { v }
}

fn execute_operation(b: i32) {
    let a: i32 = 100;

    //SINK
    let _result = a.wrapping_rem(b);
}
