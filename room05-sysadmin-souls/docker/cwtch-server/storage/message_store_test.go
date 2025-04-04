package storage

import (
	"cwtch.im/cwtch/protocol/groups"
	"encoding/binary"
	"git.openprivacy.ca/cwtch.im/server/metrics"
	"git.openprivacy.ca/openprivacy/log"
	_ "github.com/mattn/go-sqlite3" // sqlite3 driver
	"os"
	"testing"
	"time"
)

func TestMessageStore(t *testing.T) {
	filename := "../testcwtchmessages.db"
	os.Remove(filename)
	log.SetLevel(log.LevelDebug)
	counter := metrics.NewCounter()
	db, err := InitializeSqliteMessageStore(filename, -1, func() { counter.Add(1) })
	if err != nil {
		t.Fatalf("Error: %v", err)
	}

	numMessages := 100

	t.Logf("Generating Data...")
	var messages []groups.EncryptedGroupMessage
	for i := 0; i < numMessages; i++ {
		buf := make([]byte, 4)
		binary.PutUvarint(buf, uint64(i))
		messages = append(messages, groups.EncryptedGroupMessage{
			Signature:  append([]byte("Hello world"), buf...),
			Ciphertext: []byte("Hello world"),
		})
	}

	t.Logf("Populating Database")
	start := time.Now()
	for _, message := range messages {
		db.AddMessage(message)
	}
	t.Logf("Time to Insert: %v", time.Since(start))
	if counter.Count() != numMessages {
		t.Errorf("Counter should be at %v was %v", numMessages, counter.Count())
	}

	// Wait for inserts to complete..
	fetchedMessages := db.FetchMessages()
	//for _, message := range fetchedMessages {
	//t.Logf("Message: %v", message)
	//}
	if len(fetchedMessages) != numMessages {
		t.Fatalf("Incorrect number of messages returned")
	}

	t.Logf("Testing FetchMessagesFrom...")

	numToFetch := numMessages / 2

	buf := make([]byte, 4)
	binary.PutUvarint(buf, uint64(numToFetch))
	sig := append([]byte("Hello world"), buf...)
	fetchedMessages = db.FetchMessagesFrom(sig)
	//for _, message := range fetchedMessages {
	//	t.Logf("Message: %v", message)
	//}
	if len(fetchedMessages) != numToFetch {
		t.Fatalf("Incorrect number of messages returned : %v", len(messages))
	}

	db.Close()
}
