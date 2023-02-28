import math
import os
import strings
import time

[heap]
pub struct User {
pub:
    cpid string
pub mut:
    total_credit f64 = 0.0
    expavg_time f64 = 0.0
    expavg_credit f64 = 0.0
}

fn (user &User) to_xml(mut b strings.Builder) {
    b.writeln("<user>")

    b.write_string("<total_credit>")
    b.write_string("${user.total_credit:.8f}")
    b.writeln("</total_credit>")

    b.write_string("<expavg_credit>")
    b.write_string("${user.expavg_credit:.8f}")
    b.writeln("</expavg_credit>")

    b.write_string("<expavg_time>")
    b.write_string("${user.expavg_time:.8f}")
    b.writeln("</expavg_time>")

    b.write_string("<cpid>")
    b.write_string(user.cpid)
    b.writeln("</cpid>")

    b.writeln("</user>")
}

fn (user &User) to_csv(mut b strings.Builder) {
    b.write_string("${user.total_credit:.8f}")
    b.write_byte(`,`)

    b.write_string("${user.expavg_credit:.8f}")
    b.write_byte(`,`)

    b.write_string("${user.expavg_time:.8f}")
    b.write_byte(`,`)

    b.write_string(user.cpid)
    b.write_byte(`\n`)
}

const credit_half_life = 86400 * 7

fn (mut user User) update_stats(new_total_credit f64, now f64) {
    // Based on BOINC RAC calculations.
    // See https://github.com/BOINC/boinc/blob/73a7754e7fd1ae3b7bf337e8dd42a7a0b42cf3d2/html/inc/credit.inc#L24
    work := new_total_credit - user.total_credit
    if work < 0 {
        return
    }
    user.total_credit = new_total_credit

    if user.expavg_time > 0.0 {
        mut diff := math.max(now - user.expavg_time, 0.0)
        diff_days := diff / 86400
        weight := math.exp(-diff * math.ln2 / credit_half_life)

        user.expavg_credit *= weight
        if (1.0 - weight) > 0.000001 {
            user.expavg_credit += (1.0 - weight) * (work / diff_days)
        } else {
            user.expavg_credit += math.ln2 * work * 86400 / credit_half_life
        }
    }

    user.expavg_time = now
}

fn main() {
    if os.args.len != 4 {
        eprintln("USAGE: fath2boinc <local data path> <f@h data path> <boinc data path>")
        exit(1)
    }
    now := f64(time.utc().unix_time_milli()) / 1000.0

    // F@H's bulk user statistics export only contains an username
    // which can be shared with other users. To prove ownership we
    // require the usernames to be in format of <name>_GRC_<cpid>.
    //
    // If there are multiple users with the same CPID, the points
    // are summed regardless of if their names are different or not.
    mut users := map[string]&User{}

    mut count := 0
    // Reload users from file for RAC calculations.
    for line in os.read_lines(os.args[1]) or { [] } {
        parts := line.split(",")
        users[parts[3]] = &User{
            total_credit: parts[0].f64()
            expavg_credit: parts[1].f64()
            expavg_time: parts[2].f64()
            cpid: parts[3]
        }

        count += 1
    }

    println("Loaded $count entries from local user data.")

    mut new_credits := map[string]f64{}
    for line in os.read_lines(os.args[2])! {
        parts := line.split("\t")
        // Basic input sanitatization. Protects against XML and CSV injections.
        full_name := parts[0].replace(",", "").replace("<", "").replace(">", "")
        if parts.len != 4 || full_name == "name" {
            continue
        }
        name_parts := full_name.split("_")
        if name_parts.len < 3 {
            continue
        }
        cpid := name_parts[name_parts.len - 1]
        // name_parts are checked from the back to allow underscores
        // in names.
        if name_parts[name_parts.len - 2] != "GRC" || cpid.len != 32 {
            continue
        }
        for c in cpid {
            if !c.is_hex_digit() {
                continue
            }
        }

        score := parts[1].f64()
        new_credits[cpid] = new_credits[cpid] or { 0.0 } + score
    }

    for cpid, score in new_credits {
        if cpid !in users {
            users[cpid] = &User{cpid: cpid total_credit: score expavg_time: now}
        } else {
            mut user := users[cpid] or { panic("This should never happen.") }
            user.update_stats(score, now)
        }
    }

    println("Updated local statistics from F@H statistics.")

    mut b := strings.new_builder(128 * (count + 4))
    b.writeln("<?xml version='1.0'?>")
    b.writeln("<users>")
    for _, user in users {
        user.to_xml(mut b)
    }
    b.writeln("</users>")
    os.write_file(os.args[3] + '~', b.str())!
    os.mv(os.args[3] + '~', os.args[3])!
    println("Updated boinc statistics.")

    for _, user in users {
        user.to_csv(mut b)
    }
    os.write_file(os.args[1] + '~', b.str())!
    os.mv(os.args[1] + '~', os.args[1])!
    println("Updated local statistics.")
}
