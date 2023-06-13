use std::env;
use std::fmt;
use std::fmt::Write;

fn is_md5_hex(data: String) -> bool {
    return (data.len() == 32) && data.chars().all(|c| -> bool { c.is_ascii_hexdigit() });
}

struct User {
    cpid: String,
    total_credit: f64,
    expavg_credit: f64,
    expavg_time: f64
}

impl User {
    fn to_xml(&self) -> String {
        return format!("\
<user>
<total_credit>{total_credit:.8}</total_credit>
<expavg_credit>{expavg_credit:.8}</expavg_credit>
<expavg_time>{expavg_time:.8}</expavg_time>
<cpid>{cpid}</cpid>
</user>
", total_credit = self.total_credit, expavg_credit = self.expavg_credit, expavg_time = self.expavg_time, cpid = self.cpid);
    }

    fn to_csv(&self) -> String {
        return format!("{0:.8},{1:.8},{2:.8},{3}\n", self.total_credit, self.expavg_credit, self.expavg_time, self.cpid);
    }
}

fn main() -> Result<(), String> {
    if env::args().len() != 4 {
        return Err("USAGE: fath2boinc <local data path> <f@h data path> <boinc data path>".to_string());
    }

    return Ok(());
}
