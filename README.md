# fath2boinc

fath2boinc is a BOINC to Folding@Home statistics translator for the Gridcoin Network.

## Installation

For building fath2boinc, you'll need to have a recent version of V installed.

```bash
make fath2boinc # or make pkg for packaging
```

Alternatively if you're running Arch Linux, you can visit https://github.com/div72/PKGBUILDs and build/install with `makepkg -si`.

## Running

For running fath2boinc, you'll need bzip2 and curl to fetch statistics from Folding@Home and gzip to compress them for BOINC-style project stats export.

fath2boinc only translates the statistics and calculates the RAC, you'll need a webserver to actually serve the translated statistics files.

```bash
export DATADIR=/var/lib/fath2boinc # The datadir that fath2boinc will use to store the user data.
export BOINCSTATSDIR=/usr/share/nginx/html # The directory for the translated BOINC stats to be exported.

# This expects the fath2boinc binary and upgrade_folding_stats script to be in PATH.
upgrade_folding_stats
```

Alternatively, if you have installed using the Arch package you can directly use systemd:

```bash
sudo systemctl enable fath2boinc.timer # Automatic daily update.
sudo systemctl start fath2boinc.service # One-shot statistics update.
```
