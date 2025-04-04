package server

import (
	"cwtch.im/cwtch/protocol/groups"
	"encoding/json"
	"git.openprivacy.ca/cwtch.im/server/storage"
	"git.openprivacy.ca/cwtch.im/tapir"
	"git.openprivacy.ca/cwtch.im/tapir/applications"
	"git.openprivacy.ca/cwtch.im/tapir/primitives/privacypass"
	"git.openprivacy.ca/openprivacy/log"
)

// NewTokenBoardServer generates new Server for Token Board
func NewTokenBoardServer(store storage.MessageStoreInterface, tokenService *privacypass.TokenServer) tapir.Application {
	tba := new(TokenboardServer)
	tba.TokenService = tokenService
	tba.LegacyMessageStore = store
	return tba
}

// TokenboardServer defines the token board server
type TokenboardServer struct {
	applications.AuthApp
	connection         tapir.Connection
	TokenService       *privacypass.TokenServer
	LegacyMessageStore storage.MessageStoreInterface
}

// NewInstance creates a new TokenBoardApp
func (ta *TokenboardServer) NewInstance() tapir.Application {
	tba := new(TokenboardServer)
	tba.TokenService = ta.TokenService
	tba.LegacyMessageStore = ta.LegacyMessageStore
	return tba
}

// Init initializes the cryptographic TokenBoardApp
func (ta *TokenboardServer) Init(connection tapir.Connection) {
	ta.AuthApp.Init(connection)
	if connection.HasCapability(applications.AuthCapability) {
		ta.connection = connection
		go ta.Listen()
	} else {
		connection.Close()
	}
}

// Listen processes the messages for this application
func (ta *TokenboardServer) Listen() {
	for {
		data := ta.connection.Expect()
		if len(data) == 0 {
			log.Debugf("server Closing Connection")
			ta.connection.Close()
			return // connection is closed
		}

		var message groups.Message
		if err := json.Unmarshal(data, &message); err != nil {
			log.Debugf("server Closing Connection Because of Malformed Client Packet %v", err)
			ta.connection.Close()
			return // connection is closed
		}

		switch message.MessageType {
		case groups.PostRequestMessage:
			if message.PostRequest != nil {
				postrequest := *message.PostRequest
				log.Debugf("Received a Post Message Request: %v", ta.connection.Hostname())
				ta.postMessageRequest(postrequest)
			} else {
				log.Debugf("server Closing Connection Because of PostRequestMessage Client Packet")
				ta.connection.Close()
				return // connection is closed
			}
		case groups.ReplayRequestMessage:
			if message.ReplayRequest != nil {
				log.Debugf("Received Replay Request %v", message.ReplayRequest)
				messages := ta.LegacyMessageStore.FetchMessagesFrom(message.ReplayRequest.LastCommit)
				response, _ := json.Marshal(groups.Message{MessageType: groups.ReplayResultMessage, ReplayResult: &groups.ReplayResult{NumMessages: len(messages)}})
				log.Debugf("Sending Replay Response %v", groups.ReplayResult{NumMessages: len(messages)})
				ta.connection.Send(response)
				lastSignature := message.ReplayRequest.LastCommit
				for _, message := range messages {
					lastSignature = message.Signature
					data, _ = json.Marshal(message)
					ta.connection.Send(data)
				}
				log.Debugf("Finished Requested Sync")
				// Set sync and then send any new messages that might have happened while we were syncing
				ta.connection.SetCapability(groups.CwtchServerSyncedCapability)
				// Because we have set the sync capability any new messages that arrive after this point will just
				// need to do a basic lookup from the last seen message
				newMessages := ta.LegacyMessageStore.FetchMessagesFrom(lastSignature)
				for _, message := range newMessages {
					data, _ = json.Marshal(groups.Message{MessageType: groups.NewMessageMessage, NewMessage: &groups.NewMessage{EGM: *message}})
					ta.connection.Send(data)
				}
			} else {
				log.Debugf("server Closing Connection Because of Malformed ReplayRequestMessage Packet")
				ta.connection.Close()
				return // connection is closed
			}
		}
	}
}

func (ta *TokenboardServer) postMessageRequest(pr groups.PostRequest) {
	if err := ta.TokenService.SpendToken(pr.Token, append(pr.EGM.ToBytes(), ta.connection.ID().Hostname()...)); err == nil {

		// ignore messages with no signatures
		if len(pr.EGM.Signature) == 0 {
			return
		}

		log.Debugf("Token is valid")
		ta.LegacyMessageStore.AddMessage(pr.EGM)
		data, _ := json.Marshal(groups.Message{MessageType: groups.PostResultMessage, PostResult: &groups.PostResult{Success: true}})
		ta.connection.Send(data)
		data, _ = json.Marshal(groups.Message{MessageType: groups.NewMessageMessage, NewMessage: &groups.NewMessage{EGM: pr.EGM}})
		ta.connection.Broadcast(data, groups.CwtchServerSyncedCapability)
	} else {
		log.Debugf("Attempt to spend an invalid token: %v", err)
		data, _ := json.Marshal(groups.Message{MessageType: groups.PostResultMessage, PostResult: &groups.PostResult{Success: false}})
		ta.connection.Send(data)
	}
}
