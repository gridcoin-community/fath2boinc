pkg: fath2boinc update_folding_stats init/fath2boinc.service init/fath2boinc.timer init/fath2boinc.sysusers init/fath2boinc.tmpfiles
	mkdir -p pkg/usr/bin
	cp fath2boinc pkg/usr/bin/fath2boinc
	cp update_folding_stats pkg/usr/bin/update_folding_stats
	mkdir -p pkg/usr/lib/systemd/system
	cp init/fath2boinc.service pkg/usr/lib/systemd/system/fath2boinc.service
	cp init/fath2boinc.timer pkg/usr/lib/systemd/system/fath2boinc.timer
	mkdir -p pkg/usr/lib/sysusers.d
	cp init/fath2boinc.sysusers pkg/usr/lib/sysusers.d/fath2boinc.conf
	mkdir -p pkg/usr/lib/tmpfiles.d
	cp init/fath2boinc.tmpfiles pkg/usr/lib/tmpfiles.d/fath2boinc.conf

fath2boinc: fath2boinc.v
	v -prod -gc none -manualfree -prealloc fath2boinc.v 
