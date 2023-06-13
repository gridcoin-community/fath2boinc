use std::env;

fn is_md5_hex(data: String) -> bool {
    return (data.len() == 32) && data.chars().all(|c| -> bool { c.is_ascii_hexdigit() });
}

fn main() -> Result<(), String> {
    if env::args().len() != 4 {
        return Err("USAGE: fath2boinc <local data path> <f@h data path> <boinc data path>".to_string());
    }

    return Ok(());
}
