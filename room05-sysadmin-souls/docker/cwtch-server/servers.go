package server

import (
	"errors"
	"fmt"
	"git.openprivacy.ca/cwtch.im/server/storage"
	"git.openprivacy.ca/openprivacy/connectivity"
	"git.openprivacy.ca/openprivacy/log"
	"os"
	"path"
	"sync"
)

// Servers is an interface to manage multiple Cwtch servers
// Unlike a standalone server, server's dirs will be under one "$CwtchDir/servers" and use a cwtch style localID to obscure
// what servers are hosted. Users are of course free to use a default password. This means Config file will be encrypted
// with cwtch/storage/v1/file_enc and monitor files will not be generated
type Servers interface {
	LoadServers(password string) ([]string, error)
	CreateServer(password string) (Server, error)

	GetServer(onion string) Server
	ListServers() []string
	DeleteServer(onion string, currentPassword string) error

	LaunchServer(string)
	StopServer(string)
	Stop()
	Destroy()
}

type servers struct {
	lock      sync.Mutex
	servers   map[string]Server
	directory string
	acn       connectivity.ACN
}

// NewServers returns a Servers interface to manage a collection of servers
// expecting directory: $CWTCH_HOME/servers
func NewServers(acn connectivity.ACN, directory string) Servers {
	return &servers{acn: acn, directory: directory, servers: make(map[string]Server)}
}

// LoadServers will attempt to load any servers in the servers directory that are encrypted with the supplied password
// returns a list of onions identifiers for servers loaded or an error
func (s *servers) LoadServers(password string) ([]string, error) {
	s.lock.Lock()
	defer s.lock.Unlock()
	dirs, err := os.ReadDir(s.directory)
	if err != nil {
		return nil, fmt.Errorf("error: cannot read server directory: %v", err)
	}
	loadedServers := []string{}
	for _, dir := range dirs {
		newConfig, err := LoadConfig(path.Join(s.directory, dir.Name()), ServerConfigFile, true, password)
		if err == nil {
			if _, exists := s.servers[newConfig.Onion()]; !exists {
				log.Debugf("Loaded config, building server for %s\n", newConfig.Onion())
				server := NewServer(newConfig)
				s.servers[server.Onion()] = server
				loadedServers = append(loadedServers, server.Onion())
			}
		}
	}
	return loadedServers, nil
}

// CreateServer creates a new server and stores it, also returns an interface to it
func (s *servers) CreateServer(password string) (Server, error) {
	newLocalID := storage.GenerateRandomID()
	directory := path.Join(s.directory, newLocalID)
	config, err := CreateConfig(directory, ServerConfigFile, true, password, false)
	if err != nil {
		return nil, err
	}
	server := NewServer(config)
	s.lock.Lock()
	defer s.lock.Unlock()
	s.servers[server.Onion()] = server
	return server, nil
}

// GetServer returns a server interface for the supplied onion
func (s *servers) GetServer(onion string) Server {
	s.lock.Lock()
	defer s.lock.Unlock()
	return s.servers[onion]
}

// ListServers returns a list of server onion identifies this servers struct is managing
func (s *servers) ListServers() []string {
	s.lock.Lock()
	defer s.lock.Unlock()
	list := []string{}
	for onion := range s.servers {
		list = append(list, onion)
	}
	return list
}

// DeleteServer delete's the requested server (assuming the passwords match
func (s *servers) DeleteServer(onion string, password string) error {
	s.lock.Lock()
	defer s.lock.Unlock()
	server := s.servers[onion]
	if server != nil {
		err := server.Delete(password)
		if err == nil {
			delete(s.servers, onion)
		}
		return err
	}
	return errors.New("server not found")
}

// LaunchServer Run() the specified server
func (s *servers) LaunchServer(onion string) {
	s.lock.Lock()
	defer s.lock.Unlock()
	if server, exists := s.servers[onion]; exists {
		server.Run(s.acn)
	}
}

// StopServer stops the specified server
func (s *servers) StopServer(onion string) {
	s.lock.Lock()
	defer s.lock.Unlock()
	if server, exists := s.servers[onion]; exists {
		server.Stop()
	}
}

// Stop stops all the servers
func (s *servers) Stop() {
	s.lock.Lock()
	defer s.lock.Unlock()
	for _, server := range s.servers {
		server.Stop()
	}
}

// Destroy destroys all the servers
func (s *servers) Destroy() {
	s.lock.Lock()
	defer s.lock.Unlock()
	for _, server := range s.servers {
		server.Destroy()
	}
}
