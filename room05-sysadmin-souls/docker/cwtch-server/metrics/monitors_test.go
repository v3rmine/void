package metrics

import (
	tor2 "git.openprivacy.ca/cwtch.im/tapir/networks/tor"
	"git.openprivacy.ca/openprivacy/log"
	"os"
	"path/filepath"
	"testing"
	"time"
)

func TestMonitors(t *testing.T) {
	log.SetLevel(log.LevelInfo)
	os.RemoveAll("testLog")
	os.Mkdir("testLog", 0700)
	service := new(tor2.BaseOnionService)
	mp := Monitors{}
	mp.Start(service, func() int { return 1 }, "testLog", true)
	mp.MessageCounter.Add(1)
	log.Infof("sleeping for minute to give to for monitors to trigger...")
	// wait a minute for it to trigger
	time.Sleep(62 * time.Second)

	// it didn't segfault? that's good, did it create a log file?
	if _, err := os.Stat(filepath.Join("testLog", "serverMonitorReport.txt")); err != nil {
		t.Errorf("serverMonitorReport.txt not generated")
	}

	mp.Stop()
	os.RemoveAll("testLog")
}
