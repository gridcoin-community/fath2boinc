use std::collections::HashMap;
use std::env;
use std::f64::consts::LN_2;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::SystemTime;

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
    fn new(cpid: &String) -> User {
        return User {
            cpid: cpid.clone(),
            total_credit: 0.0,
            expavg_credit: 0.0,
            expavg_time: 0.0,
        };
    }

    fn to_xml(&self, buf: &mut BufWriter<File>) -> Result<(), std::io::Error> {
        writeln!(
            buf,
            "\
<user>
<total_credit>{total_credit:.8}</total_credit>
<expavg_credit>{expavg_credit:.8}</expavg_credit>
<expavg_time>{expavg_time:.8}</expavg_time>
<cpid>{cpid}</cpid>
</user>",
            total_credit = self.total_credit,
            expavg_credit = self.expavg_credit,
            expavg_time = self.expavg_time,
            cpid = self.cpid
        )?;

        return Ok(());
    }

    fn to_csv(&self, buf: &mut BufWriter<File>) -> Result<(), std::io::Error> {
        writeln!(
            buf,
            "{0:.8},{1:.8},{2:.8},{3}",
            self.total_credit, self.expavg_credit, self.expavg_time, self.cpid
        )?;

        return Ok(());
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
            let weight = (-diff * LN_2 / CREDIT_HALF_LIFE).exp();

            self.expavg_credit *= weight;
            if (1.0 - weight) > 0.000001 {
                self.expavg_credit += (1.0 - weight) * (work / diff_days);
            } else {
                self.expavg_credit += LN_2 * work * 86400.0 / CREDIT_HALF_LIFE;
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
        .as_secs() as f64;

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

    let mut new_credits = HashMap::new();
    let summary_file = File::open(&args[2]).expect("Could not open summary file.");

    for line in BufReader::new(summary_file).lines().flatten() {
        let parts: Vec<&str> = line.split('\t').collect();

        let full_name = parts[0].replace([',', '<', '>'], "");
        if parts.len() != 4 || full_name == "name" {
            continue;
        }

        let name_parts: Vec<&str> = full_name.split('_').collect();
        if name_parts.len() < 3 {
            continue;
        }

        let cpid = String::from(name_parts[name_parts.len() - 1]);
        if (name_parts[name_parts.len() - 2] != "GRC") || !is_md5_hex(&cpid) {
            continue;
        }

        let new_score = parts[1].parse::<f64>().unwrap();
        new_credits
            .entry(cpid)
            .and_modify(|score| *score += new_score)
            .or_insert(new_score);
    }

    println!(
        "Merging {} entries from F@H user summary data.",
        new_credits.len()
    );

    for (cpid, score) in new_credits {
        users
            .entry(cpid)
            .or_insert_with_key(User::new)
            .update_stats(score, now);
    }

    return Ok(());
}
