---
title: Anonymized RSS Feed Tracking
description: "Finding out how many people are subscribed to my RSS feed"
date: 2026-02-16
draft: true
taxonomies:
  tags:
    - vps
    - rss
    - tracking
extra:
  guid: cc4ce31e-f956-4bcd-a1c4-aa308259d108
  comments: { host: 'eldritch.cafe', id: "" }
---

Just a small post, because I recently restarted my blog with some [notes](@/notes/_index.md) (in French) and got my first RSS feed subscribers. I wanted to know how many people where subscribed, so I wrote a small script that I thought could be useful for others.  
(I write for myself, because I want to share what I do. But I still like to see that some people find what I write interesting)

So, I have [some public analytics](https://stats.astriiid.fr/) on the blog using GoatCounter, mostly for curiosity because I never check it. But it doesn't allow me to track RSS subscribers (I could add it in the future using the API). 

I also have Loki + Grafana to find which bot evaded [iocaine](https://iocaine.madhouse-project.org/), and I like to find new ways to spot scanners; it's a bit like a game. But it doesn't persist anything; it just drops everything automatically when it's becoming too big or too old. So, most of the time, I have the access logs of the last hours on the VPS and of the last week on Loki.

So, I made a small script that runs every 15 min, scrapes my Traefik access logs, hashes the IPs using `sha-256` (because I don't want to persist IPs on my VPS), and adds them to a log file.

```sh
#!/usr/bin/env bash
# I store only the astriiid.fr results 
# and if the file doesn't exist, I just return an empty string
# to differentiate scrapers and people subscribed to the feed
previous_readers=$(cat /var/log/logs/astriiid-fr-rss-readers.log 2>/dev/null || printf '')

new_readers=$(
  # I iterate through my Traefik access logs
  for file in /var/log/logs/traefik/*.log; do
    # Getting only the entries that match astriiid.fr
    grep '"RequestHost":"astriiid.fr"' $file | {
      # And the paths tha   t ends with an RSS file
      grep -E '"RequestPath":"[^"]+(rss|atom)\.xml"'
    };
  done | {
    # Then I use `yq` to extract only the fields I need: 
    # - The `.request_User-Agent`:
    #   Helps to know what software is used to read the feed
    # - The `.ClientHost`: 
    #   Contains the client IP and helps differentiate the users
    # - The `.RequestPath`: 
    #   Just to know how much uses the atom feed vs the RSS feed
    # - The `.time`: 
    #   To track the number of times a client requested the feed 
    #   
    # I output everything as CSV because its easier to work with in bash.
    yq -p=json '[.request_User-Agent, .ClientHost, .RequestPath, .time]' -o=csv --csv-separator='|' 
  } | {
    # yq returns null|null|null|null if input is empty so filter it out
    grep -v 'null|null'
  } | {
    awk '
    # FS: The separator used in the input
    # OFS: The separator used in the output
    BEGIN { FS="|"; OFS=FS }
    {
      # We extract the 2nd field (.ClientHost) and sha256 it
      cmd="echo "$2" | sha256sum";
      
      # We execute the command in `cmd` and store the result in `sha`
      cmd|getline sha; close(cmd);
      
      # The sha256sum output something like `abcd -`
      # So we remove everything after the first space (including it)
      sub(/ .*/,"",sha);
      
      # We replace the 2nd field (.ClientHost) with its sha256
      $2=sha;
      
      # We can now return the line
      print
    }'
  })

# We can now join the previously stored readers and the newly parsed ones
printf '%s\n%s' "$previous_readers" "$new_readers" | {
  # We don't keep any empty lines'
  grep -Ev "^$"
} | {
  # We sort everything by the 3rd field (.time)
  # because it's easier to debug 
  # if the first line is the oldest
  sort -t'|' -k3
} | {
  # We remove any duplicates caused by scraping the logs every 15min
  uniq
} > /var/log/logs/astriiid-fr-rss-readers.log
```

That's it. I don't know if it'll help anyone, but it was a fun small thing to do!
