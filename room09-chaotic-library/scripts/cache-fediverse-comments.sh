#!/usr/bin/env bash
set -eu

files_with_comments=$(\
    for file in content/**/*.md; do
        sed -n '2,/^---$/ {/^---$/d; p}' "$file" |\
            yq -f -p=yaml "[\"$file\",.extra.comments.host, .extra.comments.id]" -o=csv;
    done |\
        # Ignore the when the id is null
        grep -v "null$" |\
        # Ignore empty values
        grep -E '[^,]+,[^,]+$'\
)

echo "$files_with_comments" |\
    awk '
        BEGIN { FS=","; OFS="," }
        {
            markdown_file=$1
            comments_file=markdown_file "-comments.json"
            host=$2;
            postid=$3;
            print "https://" host "/api/v1/statuses/" postid "/context",comments_file;
        }
    ' |\
    xargs -I {} bash -c '
        URL=$(echo "{}" | cut -d, -f1)
        OUTPUT_FILE=$(echo "{}" | cut -d, -f2)
        if ! curl -sL "$URL" -o "$OUTPUT_FILE"; then
            echo "Error: Failed to cache $URL" >&2
        fi
    '

sed -i "s/last_cached_time.*/last_cached_time = \"$(date --iso-8601=seconds)\"/" config.toml
