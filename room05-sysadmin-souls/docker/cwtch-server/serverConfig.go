package server

import (
	"crypto/rand"
	"encoding/json"
	"git.openprivacy.ca/cwtch.im/server/storage"
	"git.openprivacy.ca/cwtch.im/tapir/primitives"
	"git.openprivacy.ca/openprivacy/connectivity/tor"
	"git.openprivacy.ca/openprivacy/log"
	"github.com/gtank/ristretto255"
	"golang.org/x/crypto/ed25519"
	"os"
	"path"
	"sync"
)

const (
	// AttrAutostart is the attribute key for autostart setting
	AttrAutostart = "autostart"

	// AttrDescription is the attribute key for a user set server description
	AttrDescription = "description"

	// AttrStorageType is used by clients that may need info about stored server config types/styles
	AttrStorageType = "storageType"
)

const (
	// StorageTypeDefaultPassword is a AttrStorageType that indicated a app default password was used
	StorageTypeDefaultPassword = "storage-default-password"

	// StorageTypePassword is a AttrStorageType that indicated a user password was used to protect the profile
	StorageTypePassword = "storage-password"

	// StoreageTypeNoPassword is a AttrStorageType that indicated a no password was used to protect the profile
	StoreageTypeNoPassword = "storage-no-password"
)

// Reporting is a struct for storing a the config a server needs to be a peer, and connect to a group to report
type Reporting struct {
	LogMetricsToFile bool `json:"logMetricsToFile"`
}

// messages are ~4kb of storage
const MessagesPerMB = 250

// Config is a struct for storing basic server configuration
type Config struct {
	ConfigDir string `json:"-"`
	FilePath  string `json:"-"`
	Encrypted bool   `json:"-"`
	key       [32]byte

	PublicKey  ed25519.PublicKey  `json:"publicKey"`
	PrivateKey ed25519.PrivateKey `json:"privateKey"`

	TokenServerPublicKey  ed25519.PublicKey  `json:"tokenServerPublicKey"`
	TokenServerPrivateKey ed25519.PrivateKey `json:"tokenServerPrivateKey"`

	TokenServiceK ristretto255.Scalar `json:"tokenServiceK"`

	ServerReporting Reporting `json:"serverReporting"`

	Attributes map[string]string `json:"attributes"`

	// messages are ~4kb of storage
	// -1 == infinite
	MaxStorageMBs int `json:"maxStorageMBs"`

	lock         sync.Mutex
	encFileStore storage.FileStore
}

// Identity returns an encapsulation of the servers keys
func (config *Config) Identity() primitives.Identity {
	return primitives.InitializeIdentity("", &config.PrivateKey, &config.PublicKey)
}

// TokenServiceIdentity returns an encapsulation of the servers token server (experimental)
func (config *Config) TokenServiceIdentity() primitives.Identity {
	return primitives.InitializeIdentity("", &config.TokenServerPrivateKey, &config.TokenServerPublicKey)
}

func initDefaultConfig(configDir, filename string, encrypted bool) *Config {
	config := &Config{Encrypted: encrypted, ConfigDir: configDir, FilePath: filename, Attributes: make(map[string]string)}

	id, pk := primitives.InitializeEphemeralIdentity()
	tid, tpk := primitives.InitializeEphemeralIdentity()
	config.PrivateKey = pk
	config.PublicKey = id.PublicKey()
	config.TokenServerPrivateKey = tpk
	config.TokenServerPublicKey = tid.PublicKey()
	config.ServerReporting = Reporting{
		LogMetricsToFile: false,
	}
	config.Attributes[AttrAutostart] = "false"
	config.MaxStorageMBs = -1

	k := new(ristretto255.Scalar)
	b := make([]byte, 64)
	_, err := rand.Read(b)
	if err != nil {
		// unable to generate secure random numbers
		panic("unable to generate secure random numbers")
	}
	k.SetUniformBytes(b)
	config.TokenServiceK = *k
	return config
}

// LoadCreateDefaultConfigFile loads a Config from or creates a default config and saves it to a json file specified by filename
// if the encrypted flag is true the config is store encrypted by password
func LoadCreateDefaultConfigFile(configDir, filename string, encrypted bool, password string, defaultLogToFile bool) (*Config, error) {
	if _, err := os.Stat(path.Join(configDir, filename)); os.IsNotExist(err) {
		return CreateConfig(configDir, filename, encrypted, password, defaultLogToFile)
	}
	return LoadConfig(configDir, filename, encrypted, password)
}

// CreateConfig creates a default config and saves it to a json file specified by filename
// if the encrypted flag is true the config is store encrypted by password
func CreateConfig(configDir, filename string, encrypted bool, password string, defaultLogToFile bool) (*Config, error) {
	log.Debugf("CreateConfig for server with configDir: %s\n", configDir)
	os.MkdirAll(configDir, 0700)
	config := initDefaultConfig(configDir, filename, encrypted)
	config.ServerReporting.LogMetricsToFile = defaultLogToFile
	if encrypted {
		key, _, err := storage.InitV1Directory(configDir, password)
		if err != nil {
			log.Errorf("could not create server directory: %s", err)
			return nil, err
		}
		config.key = key
		config.encFileStore = storage.NewFileStore(configDir, ServerConfigFile, key)
	}

	config.Save()
	return config, nil
}

// LoadConfig loads a Config from a json file specified by filename
func LoadConfig(configDir, filename string, encrypted bool, password string) (*Config, error) {
	config := initDefaultConfig(configDir, filename, encrypted)
	var raw []byte
	var err error
	if encrypted {
		salt, err := os.ReadFile(path.Join(configDir, storage.SaltFile))
		if err != nil {
			return nil, err
		}
		config.key = storage.CreateKey(password, salt)
		config.encFileStore = storage.NewFileStore(configDir, ServerConfigFile, config.key)
		raw, err = config.encFileStore.Read()
		if err != nil {
			// Not an error to log as load config is called blindly across all dirs with a password to see what it applies to
			log.Debugf("read enc bytes failed: %s\n", err)
			return nil, err
		}
	} else {
		raw, err = os.ReadFile(path.Join(configDir, filename))
		if err != nil {
			return nil, err
		}
	}

	if err = json.Unmarshal(raw, &config); err != nil {
		log.Errorf("reading config: %v", err)
		return nil, err
	}

	// Always save (first time generation, new version with new variables populated)
	config.Save()
	return config, nil
}

// Save dumps the latest version of the config to a json file given by filename
func (config *Config) Save() error {
	config.lock.Lock()
	defer config.lock.Unlock()
	bytes, _ := json.MarshalIndent(config, "", "\t")
	if config.Encrypted {
		return config.encFileStore.Write(bytes)
	}
	return os.WriteFile(path.Join(config.ConfigDir, config.FilePath), bytes, 0600)
}

// CheckPassword returns true if the given password produces the same key as the current stored key, otherwise false.
func (config *Config) CheckPassword(checkpass string) bool {
	config.lock.Lock()
	defer config.lock.Unlock()
	salt, err := os.ReadFile(path.Join(config.ConfigDir, storage.SaltFile))
	if err != nil {
		return false
	}
	oldkey := storage.CreateKey(checkpass, salt[:])
	return oldkey == config.key
}

// Onion returns the .onion url for the server
func (config *Config) Onion() string {
	config.lock.Lock()
	defer config.lock.Unlock()
	return tor.GetTorV3Hostname(config.PublicKey) + ".onion"
}

// SetAttribute sets a server attribute
func (config *Config) SetAttribute(key, val string) {
	config.lock.Lock()
	config.Attributes[key] = val
	config.lock.Unlock()
	config.Save()
}

// GetAttribute gets a server attribute
func (config *Config) GetAttribute(key string) string {
	config.lock.Lock()
	defer config.lock.Unlock()
	return config.Attributes[key]
}

// GetMaxMessages returns the config setting for Max messages converting from MaxMB to messages
// or -1 for infinite
func (config *Config) GetMaxMessages() int {
	config.lock.Lock()
	defer config.lock.Unlock()
	if config.MaxStorageMBs == -1 {
		return -1
	}
	return config.MaxStorageMBs * MessagesPerMB
}

func (config *Config) GetMaxMessageMBs() int {
	config.lock.Lock()
	defer config.lock.Unlock()
	return config.MaxStorageMBs
}

func (config *Config) SetMaxMessageMBs(newval int) {
	config.lock.Lock()
	defer config.lock.Unlock()
	config.MaxStorageMBs = newval
}
