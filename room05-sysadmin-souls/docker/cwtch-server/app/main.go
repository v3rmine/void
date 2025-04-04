package main

import (
	"crypto/rand"
	"encoding/base64"
	"flag"
	cwtchserver "git.openprivacy.ca/cwtch.im/server"
	"git.openprivacy.ca/cwtch.im/tapir/primitives"
	"git.openprivacy.ca/openprivacy/connectivity/tor"
	"git.openprivacy.ca/openprivacy/log"
	_ "github.com/mattn/go-sqlite3" // sqlite3 driver
	mrand "math/rand"
	"os"
	"os/signal"
	"path"
	"syscall"
	"time"
)

func main() {
	flagDebug := flag.Bool("debug", false, "Enable debug logging")
	flagExportServer := flag.Bool("exportServerBundle", false, "Export the server bundle to a file called serverbundle")
	flagDir := flag.String("dir", ".", "Directory to store server files in (config, encrypted messages, metrics)")
	flagDisableMetrics := flag.Bool("disableMetrics", false, "Disable metrics reporting")
	flag.Parse()

	log.AddEverythingFromPattern("server/app/main")
	log.AddEverythingFromPattern("server/server")
	log.ExcludeFromPattern("service.go")
	log.SetLevel(log.LevelInfo)
	if *flagDebug {
		log.Infoln("enableing Debug logging")
		log.SetLevel(log.LevelDebug)
	}
	configDir := os.Getenv("CWTCH_HOME")
	if configDir == "" {
		configDir = *flagDir
	}
	if len(os.Args) == 2 && os.Args[1] == "gen1" {
		config := new(cwtchserver.Config)
		id, pk := primitives.InitializeEphemeralIdentity()
		tid, tpk := primitives.InitializeEphemeralIdentity()
		config.PrivateKey = pk
		config.PublicKey = id.PublicKey()
		config.TokenServerPrivateKey = tpk
		config.TokenServerPublicKey = tid.PublicKey()
		config.ServerReporting = cwtchserver.Reporting{
			LogMetricsToFile: true,
		}
		config.ConfigDir = "."
		config.FilePath = cwtchserver.ServerConfigFile
		config.Encrypted = false
		config.Save()
		return
	}

	disableMetrics := *flagDisableMetrics
	if os.Getenv("DISABLE_METRICS") != "" {
		disableMetrics = true
	}
	serverConfig, err := cwtchserver.LoadCreateDefaultConfigFile(configDir, cwtchserver.ServerConfigFile, false, "", !disableMetrics)
	if err != nil {
		log.Errorf("Could not load/create config file: %s\n", err)
		return
	}
	serverConfig.ServerReporting.LogMetricsToFile = !disableMetrics
	// we don't need real randomness for the port, just to avoid a possible conflict...
	r := mrand.New(mrand.NewSource(int64(time.Now().Nanosecond())))
	controlPort := r.Intn(1000) + 9052

	// generate a random password
	key := make([]byte, 64)
	_, err = rand.Read(key)
	if err != nil {
		panic(err)
	}

	os.MkdirAll("tordir/tor", 0700)
	tor.NewTorrc().WithHashedPassword(base64.StdEncoding.EncodeToString(key)).WithControlPort(controlPort).WithSocksPort(controlPort + 1).Build("./tordir/tor/torrc")
	acn, err := tor.NewTorACNWithAuth("tordir", "", "tordir/tor", controlPort, tor.HashedPasswordAuthenticator{Password: base64.StdEncoding.EncodeToString(key)})
	if err != nil {
		log.Errorf("\nError connecting to Tor: %v\n", err)
		os.Exit(1)
	}
	defer acn.Close()

	server := cwtchserver.NewServer(serverConfig)
	log.Infoln("starting cwtch server...")
	log.Infof("Server %s\n", server.Onion())

	log.Infof("Server bundle (import into client to use server): %s\n", log.Magenta(server.ServerBundle()))

	if *flagExportServer {
		// Todo: change all to server export
		os.WriteFile(path.Join(serverConfig.ConfigDir, "serverbundle"), []byte(server.ServerBundle()), 0600)
	}

	// Graceful Stop
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt, syscall.SIGTERM)
	go func() {
		<-c
		server.Destroy()
		acn.Close()
		os.Exit(1)
	}()

	running := false
	lastStatus := -2
	for {
		status, msg := acn.GetBootstrapStatus()
		if status == 100 && !running {
			log.Infoln("ACN is online, Running Server")
			server.Run(acn)
			running = true
		}
		if status != 100 {
			if running {
				log.Infoln("ACN is offline, Stopping Server")
				server.Stop()
				running = false
			} else {
				if lastStatus != status {
					log.Infof("ACN booting... Status %v%%: %v\n", status, msg)
					lastStatus = status
				}
			}
		}
		time.Sleep(time.Second)
	}
}
