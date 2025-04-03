#!/bin/sh
days=$(find . -name 'day*.fnl' | grep -o "[0-9]*" | sort | tr '\n' ',' | sed 's/,$//g')
hyperfine -w 10 ---parameter-list daynum "$days" 'cat day{daynum}-input.txt | fennel day{daynum}.fnl'
