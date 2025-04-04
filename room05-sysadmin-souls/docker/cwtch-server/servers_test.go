package server

import (
	"git.openprivacy.ca/openprivacy/connectivity"
	"git.openprivacy.ca/openprivacy/log"
	"os"
	"testing"
)

const TestDir = "./serversTest"
const DefaultPassword = "be gay do crime"

const TestServerDesc = "a test Server"

func TestServers(t *testing.T) {
	log.SetLevel(log.LevelDebug)
	log.Infof("clean up / setup...")
	os.RemoveAll(TestDir)
	os.Mkdir(TestDir, 0700)

	acn := connectivity.NewLocalACN()
	log.Infof("NewServers()...")
	servers := NewServers(acn, TestDir)
	s, err := servers.CreateServer(DefaultPassword)
	if err != nil {
		t.Errorf("could not create server: %s", err)
		return
	}
	s.SetAttribute(AttrDescription, TestServerDesc)
	serverOnion := s.Onion()

	s.Destroy()

	log.Infof("NewServers()...")
	servers2 := NewServers(acn, TestDir)
	log.Infof("LoadServers()...")
	list, err := servers2.LoadServers(DefaultPassword)
	log.Infof("Loaded!")
	if err != nil {
		t.Errorf("clould not load server: %s", err)
		return
	}
	if len(list) != 1 {
		t.Errorf("expected to load 1 server, got %d", len(list))
		return
	}

	if list[0] != serverOnion {
		t.Errorf("expected loaded server to have onion: %s but got %s", serverOnion, list[0])
	}

	s1 := servers.GetServer(list[0])
	if s1.GetAttribute(AttrDescription) != TestServerDesc {
		t.Errorf("expected server description of '%s' but got '%s'", TestServerDesc, s1.GetAttribute(AttrDescription))
	}

	servers2.Destroy()
	os.RemoveAll(TestDir)
}
