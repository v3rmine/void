# Cwtch Server

**PLEASE I'M NO SECURIY EXPERT DO NOT USE IT WITHOUT DOING THE CHANGES YOURSELF ON THE LATEST VERSION**

Fork of cwtch.im/server to repair docker build
https://git.openprivacy.ca/cwtch.im/server

## Building

Pretty straight forward:
- build the app in `app/` with `go build`
- build the docker container in `docker/` with `docker build . -t openpriv/server`

### Windows

The server package relies on sqlite which in turn requires the use of CGO. As per [this issue](https://github.com/golang/go/issues/12029) that means [TDM-GCC](https://jmeubank.github.io/tdm-gcc/download/) is required to be installed and used to compile on Windows

## Running

- cd app
- go build
- ./app

The app takes the following arguments
- -debug: enabled debug logging
- -exportServerBundle: Export the server bundle to a file called serverbundle
- -disableMetrics: Disable metrics reporting to serverMonitor.txt and associated tracking routines
- -dir [directory]: specify a directory to store server files (default is current directory) 

The app takes the following environment variables
- CWTCH_HOME: sets the config dir for the app
- DISABLE_METRICS: if set to any value ('1') it disables metrics reporting to serverMonitor.txt and associated tracking routines 

`env CONFIG_HOME=./conf ./app`

## Using the Server

When run the app will output standard log lines, one of which will contain the `serverbundle` in purple. This is the part you need to capture and import into a Cwtch client app so you can use the server for hosting groups

## Docker

Build by executing `docker build -f docker/Dockerfile .`

or run our prebuild ones with

`pull openpriv/cwtch-server`

and run it. It stores all Cwtch data in a Volume at `/var/lib/cwtch` so if you want the server data to persist you would run

`docker run -v /var/lib/cwtch/server01:/var/lib/cwtch openpriv/cwtch-server`

to create a persistent container you might try a command like:

`docker run --name cwtch -v /var/lib/cwtch/server01:/var/lib/cwtch --restart always openpriv/cwtch-server`
