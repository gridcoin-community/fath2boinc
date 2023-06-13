use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::SystemTime;

const LN2: f64 = 0.693147180559945309417;
const CREDIT_HALF_LIFE: f64 = 86400.0 * 7.0;

fn is_md5_hex(data: &String) -> bool {
    return (data.len() == 32) && data.chars().all(|c| -> bool { c.is_ascii_hexdigit() });
}

struct User {
    cpid: String,
    total_credit: f64,
    expavg_credit: f64,
    expavg_time: f64,
}

impl User {
    fn to_xml(&self) -> String {
        return format!(
            "\
<user>
<total_credit>{total_credit:.8}</total_credit>
<expavg_credit>{expavg_credit:.8}</expavg_credit>
<expavg_time>{expavg_time:.8}</expavg_time>
<cpid>{cpid}</cpid>
</user>
",
            total_credit = self.total_credit,
            expavg_credit = self.expavg_credit,
            expavg_time = self.expavg_time,
            cpid = self.cpid
        );
    }

    fn to_csv(&self) -> String {
        return format!(
            "{0:.8},{1:.8},{2:.8},{3}\n",
            self.total_credit, self.expavg_credit, self.expavg_time, self.cpid
        );
    }

    fn update_stats(&mut self, new_total_credit: f64, now: f64) {
        // Based on BOINC RAC calculations.
        // See https://github.com/BOINC/boinc/blob/73a7754e7fd1ae3b7bf337e8dd42a7a0b42cf3d2/html/inc/credit.inc#L24
        let work = new_total_credit - self.total_credit;
        if work < 0.0 {
            return;
        }
        self.total_credit = new_total_credit;

        if self.expavg_time > 0.0 {
            let diff = f64::max(now - self.expavg_time, 0.0);
            let diff_days = diff / 86400.0;
            let weight = (-diff * LN2 / CREDIT_HALF_LIFE).exp();

            self.expavg_credit *= weight;
            if (1.0 - weight) > 0.000001 {
                self.expavg_credit += (1.0 - weight) * (work / diff_days);
            } else {
                self.expavg_credit += LN2 * work * 86400.0 / CREDIT_HALF_LIFE;
            }
        }

        self.expavg_time = now;
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        return Err(
            "USAGE: fath2boinc <local data path> <f@h data path> <boinc data path>".to_string(),
        );
    }
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut users = HashMap::new();

    match File::open(&args[1]) {
        Ok(file) => {
            for line in BufReader::new(file).lines().flatten() {
                let parts: Vec<&str> = line.split(',').collect();
                let cpid: String = String::from(parts[3]);

                assert!(is_md5_hex(&cpid));
                users.insert(
                    cpid.clone(),
                    User {
                        total_credit: parts[0].parse::<f64>().unwrap(),
                        expavg_credit: parts[1].parse::<f64>().unwrap(),
                        expavg_time: parts[2].parse::<f64>().unwrap(),
                        cpid: cpid,
                    },
                );
            }

            println!("Loaded {} entries from local user data.", users.len());
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("Local user data file does not exist, skipping initial load.");
        }
        Err(e) => {
            panic!("Could not open local data path: {}", e);
        }
    }

    return Ok(());
}
