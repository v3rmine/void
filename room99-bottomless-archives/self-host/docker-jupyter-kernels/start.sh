#!/bin/sh
sudo chmod -R 770 notebooks/
sudo chown -R :1000 notebooks/
docker-compose up --build
