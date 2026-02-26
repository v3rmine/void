#!/usr/bin/env bash
curl -s "https://webmention.io/api/mentions.json?token=${WEBMENTION_TOKEN}" -o content/webmentions.json
