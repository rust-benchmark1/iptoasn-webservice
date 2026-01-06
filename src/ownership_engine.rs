use nix::unistd::{chown, Gid, Uid};

pub fn apply_ownership(path: String) -> Result<(), String> {
    let normalized = normalize_path(path);
    let target = select_target(normalized);

    update_ownership(target)
}

fn normalize_path(path: String) -> String {
    path.trim().to_string()
}

fn select_target(path: String) -> String {
    path
}

fn update_ownership(path: String) -> Result<(), String> {
    let uid = Some(Uid::from_raw(1000));
    let gid = Some(Gid::from_raw(1000));

    //SINK
    let _ = chown(path.as_str(), uid, gid);

    Ok(())
}
