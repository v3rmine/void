package server

import (
	"crypto/ed25519"
	"cwtch.im/cwtch/model"
	"encoding/base64"
	"errors"
	"fmt"
	"git.openprivacy.ca/cwtch.im/server/metrics"
	"git.openprivacy.ca/cwtch.im/server/storage"
	"git.openprivacy.ca/cwtch.im/tapir"
	"git.openprivacy.ca/cwtch.im/tapir/applications"
	tor2 "git.openprivacy.ca/cwtch.im/tapir/networks/tor"
	"git.openprivacy.ca/cwtch.im/tapir/persistence"
	"git.openprivacy.ca/cwtch.im/tapir/primitives"
	"git.openprivacy.ca/cwtch.im/tapir/primitives/privacypass"
	"git.openprivacy.ca/openprivacy/connectivity"
	"git.openprivacy.ca/openprivacy/connectivity/tor"
	"git.openprivacy.ca/openprivacy/log"
	"os"
	"path"
	"sync"
)

const (
	// ServerConfigFile is the standard filename for a server's config to be written to in a directory
	ServerConfigFile = "serverConfig.json"
)

// Server encapsulates a complete, compliant Cwtch server.
type Server interface {
	Identity() primitives.Identity
	Run(acn connectivity.ACN) error
	KeyBundle() *model.KeyBundle
	CheckStatus() (bool, error)
	Stop()
	Destroy()
	GetStatistics() Statistics
	Delete(password string) error
	Onion() string
	ServerBundle() string
	TofuBundle() string
	GetAttribute(string) string
	SetAttribute(string, string)
	SetMonitorLogging(bool)
}

type server struct {
	config              *Config
	service             tapir.Service
	messageStore        storage.MessageStoreInterface
	metricsPack         metrics.Monitors
	tokenTapirService   tapir.Service
	tokenServer         *privacypass.TokenServer
	tokenService        primitives.Identity
	tokenServicePrivKey ed25519.PrivateKey
	tokenServiceStopped bool
	onionServiceStopped bool
	running             bool
	lock                sync.RWMutex
}

// NewServer creates and configures a new server based on the supplied configuration
func NewServer(serverConfig *Config) Server {
	server := new(server)
	server.running = false
	server.config = serverConfig
	server.tokenService = server.config.TokenServiceIdentity()
	server.tokenServicePrivKey = server.config.TokenServerPrivateKey
	bs := new(persistence.BoltPersistence)
	bs.Open(path.Join(serverConfig.ConfigDir, "tokens.db"))
	server.tokenServer = privacypass.NewTokenServerFromStore(&serverConfig.TokenServiceK, bs)
	log.Infof("Y: %v", server.tokenServer.Y)
	return server
}

// Identity returns the main onion identity of the server
func (s *server) Identity() primitives.Identity {
	return s.config.Identity()
}

// helper fn to pass to metrics
func (s *server) getStorageTotalMessageCount() int {
	if s.messageStore != nil {
		return s.messageStore.MessagesCount()
	}
	return 0
}

// helper fn to pass to storage
func (s *server) incMessageCount() {
	if s.metricsPack.MessageCounter != nil {
		s.metricsPack.MessageCounter.Add(1)
	}
}

// Run starts a server with the given privateKey
func (s *server) Run(acn connectivity.ACN) error {
	s.lock.Lock()
	defer s.lock.Unlock()
	if s.running {
		return nil
	}

	identity := primitives.InitializeIdentity("", &s.config.PrivateKey, &s.config.PublicKey)
	service := new(tor2.BaseOnionService)
	service.Init(acn, s.config.PrivateKey, &identity)
	s.service = service
	log.Infof("cwtch server running on cwtch:%s\n", s.Onion())

	if s.config.ServerReporting.LogMetricsToFile {
		s.metricsPack.Start(service, s.getStorageTotalMessageCount, s.config.ConfigDir, s.config.ServerReporting.LogMetricsToFile)
	}

	var err error
	s.messageStore, err = storage.InitializeSqliteMessageStore(path.Join(s.config.ConfigDir, "cwtch.messages"), s.config.GetMaxMessages(), s.incMessageCount)
	if err != nil {
		return fmt.Errorf("could not open database: %v", err)
	}

	s.tokenTapirService = new(tor2.BaseOnionService)
	s.tokenTapirService.Init(acn, s.tokenServicePrivKey, &s.tokenService)
	tokenApplication := new(applications.TokenApplication)
	tokenApplication.TokenService = s.tokenServer
	powTokenApp := new(applications.ApplicationChain).
		ChainApplication(new(applications.ProofOfWorkApplication), applications.SuccessfulProofOfWorkCapability).
		ChainApplication(tokenApplication, applications.HasTokensCapability)
	go func() {
		s.tokenTapirService.Listen(powTokenApp)
		s.tokenServiceStopped = true
	}()
	go func() {
		s.service.Listen(NewTokenBoardServer(s.messageStore, s.tokenServer))
		s.onionServiceStopped = true
	}()

	s.running = true
	return nil
}

// KeyBundle provides the signed keybundle of the server
func (s *server) KeyBundle() *model.KeyBundle {
	kb := model.NewKeyBundle()
	identity := s.config.Identity()
	kb.Keys[model.KeyTypeServerOnion] = model.Key(identity.Hostname())
	kb.Keys[model.KeyTypeTokenOnion] = model.Key(s.tokenService.Hostname())
	kb.Keys[model.KeyTypePrivacyPass] = model.Key(s.tokenServer.Y.String())
	kb.Sign(identity)
	return kb
}

// CheckStatus returns true if the server is running and/or an error if any part of the server needs to be restarted.
func (s *server) CheckStatus() (bool, error) {
	s.lock.RLock()
	defer s.lock.RUnlock()
	if s.onionServiceStopped || s.tokenServiceStopped {
		return s.running, fmt.Errorf("one of more server components are down: onion:%v token service: %v", s.onionServiceStopped, s.tokenServiceStopped)
	}
	return s.running, nil
}

// Stop turns off the server so it cannot receive connections and frees most resourses.
// The server is still in a reRunable state and tokenServer still has an active persistence
func (s *server) Stop() {
	log.Infof("Shutting down server")
	s.lock.Lock()
	defer s.lock.Unlock()
	if s.running {
		s.service.Shutdown()
		s.messageStore.Close()
		s.tokenTapirService.Shutdown()
		log.Infof("Closing Token server Database...")

		s.metricsPack.Stop()
		s.running = false
	}
}

// Destroy frees the last of the resources the server has active (tokenServer persistence) leaving it un-re-runable and completely shutdown
func (s *server) Destroy() {
	s.Stop()
	s.lock.Lock()
	defer s.lock.Unlock()
	s.tokenServer.Close()
}

// Statistics is an encapsulation of information about the server that an operator might want to know at a glance.
type Statistics struct {
	TotalMessages    int
	TotalConnections int
}

// GetStatistics is a stub method for providing some high level information about
// the server operation to bundling applications (e.g. the UI)
func (s *server) GetStatistics() Statistics {
	if s.running {
		return Statistics{
			TotalMessages:    s.messageStore.MessagesCount(),
			TotalConnections: s.service.Metrics().ConnectionCount,
		}
	}
	return Statistics{}
}

func (s *server) Delete(password string) error {
	s.lock.Lock()
	if s.config.Encrypted && !s.config.CheckPassword(password) {
		s.lock.Unlock()
		log.Errorf("encryped and checkpassword failed")
		return errors.New("cannot delete server, passwords do not match")
	}
	s.lock.Unlock()
	s.Destroy()
	os.RemoveAll(s.config.ConfigDir)
	return nil
}

func (s *server) Onion() string {
	return s.config.Onion()
}

// ServerBundle returns a bundle of the server keys required to access it (torv3 keys are addresses)
func (s *server) ServerBundle() string {
	bundle := s.KeyBundle().Serialize()
	return fmt.Sprintf("server:%s", base64.StdEncoding.EncodeToString(bundle))
}

// TofuBundle returns a Server Bundle + a newly created group invite
func (s *server) TofuBundle() string {
	group, _ := model.NewGroup(tor.GetTorV3Hostname(s.config.PublicKey))
	invite, err := group.Invite()
	if err != nil {
		panic(err)
	}
	bundle := s.KeyBundle().Serialize()
	return fmt.Sprintf("tofubundle:server:%s||%s", base64.StdEncoding.EncodeToString(bundle), invite)
}

// GetAttribute gets a server attribute
func (s *server) GetAttribute(key string) string {
	return s.config.GetAttribute(key)
}

// SetAttribute sets a server attribute
func (s *server) SetAttribute(key, val string) {
	s.config.SetAttribute(key, val)
}

// GetMessageCap gets a server's MaxStorageMBs value
func (s *server) GetMaxStorageMBs() int {
	return s.config.GetMaxMessageMBs()
}

// SetMaxStorageMBs sets a server's MaxStorageMBs and sets MaxMessages for storage (which can trigger a prune)
func (s *server) SetMaxStorageMBs(val int) {
	s.config.SetMaxMessageMBs(val)
	s.messageStore.SetMessageCap(s.config.GetMaxMessages())
}

// SetMonitorLogging turns on or off the monitor logging suite, and logging to a file in the server dir
func (s *server) SetMonitorLogging(do bool) {
	s.config.ServerReporting.LogMetricsToFile = do
	s.config.Save()
	if do {
		s.metricsPack.Start(s.service, s.getStorageTotalMessageCount, s.config.ConfigDir, s.config.ServerReporting.LogMetricsToFile)
	} else {
		s.metricsPack.Stop()
	}
}
