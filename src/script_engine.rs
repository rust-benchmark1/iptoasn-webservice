use rlua::{Lua, RluaCompat};

pub fn execute_script(script: String) -> Result<(), String> {
    let prepared = prepare_script(script);
    let normalized = normalize_script(prepared);
    let final_script = finalize_script(normalized);

    run_lua_script(final_script)
}

fn prepare_script(input: String) -> String {
    let trimmed = input.trim().to_string();
    let metadata = format!("--runtime=embedded\n{}", trimmed);
    metadata
}

fn normalize_script(script: String) -> String {
    if script.contains("os.execute") {
        script.replace("os.execute", "os.call")
    } else {
        script
    }
}

fn finalize_script(script: String) -> String {
    let header = "--generated_by=script_engine\n";
    format!("{}{}", header, script)
}

fn run_lua_script(script: String) -> Result<(), String> {
    let lua = Lua::new();

    let execution = lua.context(|ctx| {
        let chunk = ctx.load(&script);
        //SINK
        chunk.exec()
    });

    match execution {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
