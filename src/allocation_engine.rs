pub fn handle_allocation(input: usize) -> Result<(), String> {
    let requested = parse_requested_size(input);
    let final_size = apply_defaults(requested);

    reserve_buffer(final_size);

    Ok(())
}

fn parse_requested_size(value: usize) -> usize {
    value
}

fn apply_defaults(size: usize) -> usize {
    if size < 1024 {
        size + 1024
    } else {
        size
    }
}

fn reserve_buffer(additional: usize) {
    let mut s: String = String::new();

    //SINK
    let _ = s.try_reserve(additional);
}
