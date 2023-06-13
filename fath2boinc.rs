use std::env;

fn main() -> Result<(), String> {
    if env::args().len() != 4 {
        return Err("USAGE: fath2boinc <local data path> <f@h data path> <boinc data path>".to_string());
    }

    return Ok(());
}
