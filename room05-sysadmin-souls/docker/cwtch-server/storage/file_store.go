package storage

import (
	"crypto/rand"
	"encoding/hex"
	"errors"
	"git.openprivacy.ca/openprivacy/log"
	"golang.org/x/crypto/nacl/secretbox"
	"golang.org/x/crypto/pbkdf2"
	"golang.org/x/crypto/sha3"
	"io"
	"os"
	"path"
)

// SaltFile is the standard filename to store an encrypted config's SALT under beside it
const SaltFile = "SALT"

const version = "1"
const versionFile = "VERSION"

// GenerateRandomID generates a random 16 byte hex id code
func GenerateRandomID() string {
	randBytes := make([]byte, 16)
	rand.Read(randBytes)
	return hex.EncodeToString(randBytes)
}

// InitV1Directory generates a key and salt from a password, writes a SALT and VERSION file and returns the key and salt
func InitV1Directory(directory, password string) ([32]byte, [128]byte, error) {
	os.Mkdir(directory, 0700)

	key, salt, err := CreateKeySalt(password)
	if err != nil {
		log.Errorf("Could not create key for profile store from password: %v\n", err)
		return [32]byte{}, [128]byte{}, err
	}

	if err = os.WriteFile(path.Join(directory, versionFile), []byte(version), 0600); err != nil {
		log.Errorf("Could not write version file: %v", err)
		return [32]byte{}, [128]byte{}, err
	}

	if err = os.WriteFile(path.Join(directory, SaltFile), salt[:], 0600); err != nil {
		log.Errorf("Could not write salt file: %v", err)
		return [32]byte{}, [128]byte{}, err
	}

	return key, salt, nil
}

// fileStore stores a cwtchPeer in an encrypted file
type fileStore struct {
	directory string
	filename  string
	key       [32]byte
}

// FileStore is a primitive around storing encrypted files
type FileStore interface {
	Write([]byte) error
	Read() ([]byte, error)
	Delete()
	ChangeKey(newkey [32]byte)
}

// NewFileStore instantiates a fileStore given a filename and a password
func NewFileStore(directory string, filename string, key [32]byte) FileStore {
	filestore := new(fileStore)
	filestore.key = key
	filestore.filename = filename
	filestore.directory = directory
	return filestore
}

// CreateKeySalt derives a key and salt from a password: returns key, salt, err
func CreateKeySalt(password string) ([32]byte, [128]byte, error) {
	var salt [128]byte
	if _, err := io.ReadFull(rand.Reader, salt[:]); err != nil {
		log.Errorf("Cannot read from random: %v\n", err)
		return [32]byte{}, salt, err
	}
	dk := pbkdf2.Key([]byte(password), salt[:], 4096, 32, sha3.New512)

	var dkr [32]byte
	copy(dkr[:], dk)
	return dkr, salt, nil
}

// CreateKey derives a key from a password and salt
func CreateKey(password string, salt []byte) [32]byte {
	dk := pbkdf2.Key([]byte(password), salt, 4096, 32, sha3.New512)

	var dkr [32]byte
	copy(dkr[:], dk)
	return dkr
}

// EncryptFileData encrypts the data with the supplied key
func EncryptFileData(data []byte, key [32]byte) ([]byte, error) {
	var nonce [24]byte

	if _, err := io.ReadFull(rand.Reader, nonce[:]); err != nil {
		log.Errorf("Cannot read from random: %v\n", err)
		return nil, err
	}

	encrypted := secretbox.Seal(nonce[:], data, &nonce, &key)
	return encrypted, nil
}

// DecryptFile decrypts the passed ciphertext with the supplied key.
func DecryptFile(ciphertext []byte, key [32]byte) ([]byte, error) {
	var decryptNonce [24]byte
	copy(decryptNonce[:], ciphertext[:24])
	decrypted, ok := secretbox.Open(nil, ciphertext[24:], &decryptNonce, &key)
	if ok {
		return decrypted, nil
	}
	return nil, errors.New("failed to decrypt")
}

// ReadEncryptedFile reads data from an encrypted file in directory with key
func ReadEncryptedFile(directory, filename string, key [32]byte) ([]byte, error) {
	encryptedbytes, err := os.ReadFile(path.Join(directory, filename))
	if err == nil {
		return DecryptFile(encryptedbytes, key)
	}
	return nil, err
}

// write serializes a cwtchPeer to a file
func (fps *fileStore) Write(data []byte) error {
	encryptedbytes, err := EncryptFileData(data, fps.key)
	if err != nil {
		return err
	}

	err = os.WriteFile(path.Join(fps.directory, fps.filename), encryptedbytes, 0600)
	return err
}

func (fps *fileStore) Read() ([]byte, error) {
	return ReadEncryptedFile(fps.directory, fps.filename, fps.key)
}

func (fps *fileStore) Delete() {
	err := os.Remove(path.Join(fps.directory, fps.filename))
	if err != nil {
		log.Errorf("Deleting file %v\n", err)
	}
}

func (fps *fileStore) ChangeKey(newkey [32]byte) {
	fps.key = newkey
}
