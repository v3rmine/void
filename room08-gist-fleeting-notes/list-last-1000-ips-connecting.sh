#!/bin/sh
# Only in tcp
tcpdump -- -i eth0 -n 'tcp[tcpflags] & tcp-syn != 0' 2>/dev/null | sed 's/^.*IP \([^ ]*\) >.*$/\1/' | sed -E 's/\.[0-9]+$//' | head -n 1000 | sort | uniq -c | sort -n
