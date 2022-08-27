pkg: fath2boinc update_folding_stats
	mkdir -p pkg/usr/bin
	cp fath2boinc pkg/usr/bin/fath2boinc
	cp update_folding_stats pkg/usr/bin/update_folding_stats

fath2boinc: fath2boinc.v
	v -prod -gc none -manualfree -prealloc fath2boinc.v 
