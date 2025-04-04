#!/bin/sh
cp .env.example .env

mkdir ./data ./app_data ./config ./ssh

ssh-keygen -t rsa -b 2048 -f ./ssh/id_rsa
echo ================
echo "Please write this public key, it's the container's ssh public key"
echo ================
cat ./ssh/id_rsa.pub
echo ================
chmod 600 ssh/*
