// // Copyright (c) quickfixengine.org  All rights reserved.
// //
// // This file may be distributed under the terms of the quickfixengine.org
// // license as defined by quickfixengine.org and appearing in the file
// // LICENSE included in the packaging of this file.
// //
// // This file is provided AS IS with NO WARRANTY OF ANY KIND, INCLUDING
// // THE WARRANTY OF DESIGN, MERCHANTABILITY AND FITNESS FOR A
// // PARTICULAR PURPOSE.
// //
// // See http://www.quickfixengine.org/LICENSE for licensing information.
// //
// // Contact ask@quickfixengine.org if any conditions of this licensing
// // are not clear to you.

// package quickfix

// import "github.com/quickfixgo/quickfix/internal"

// type resendState struct {
// 	loggedOn
// 	messageStash          map[int]*Message
// 	currentResendRangeEnd int
// 	resendRangeEnd        int
// }

// func (s resendState) String() string { return "Resend" }

// func (s resendState) Timeout(session *session, event internal.Event) (nextState sessionState) {
// 	nextState = inSession{}.Timeout(session, event)
// 	switch nextState.(type) {
// 	case inSession:
// 		nextState = s
// 	case pendingTimeout:
// 		// Wrap pendingTimeout in resend. prevents us falling back to inSession if recovering
// 		// from pendingTimeout.
// 		nextState = pendingTimeout{s}
// 	}

// 	return
// }

// func (s resendState) FixMsgIn(session *session, msg *Message) (nextState sessionState) {
// 	nextState = inSession{}.FixMsgIn(session, msg)

// 	if !nextState.IsLoggedOn() {
// 		return
// 	}

// 	if s.currentResendRangeEnd != 0 && s.currentResendRangeEnd < session.store.NextTargetMsgSeqNum() {
// 		nextResendState, err := session.sendResendRequest(session.store.NextTargetMsgSeqNum(), s.resendRangeEnd)
// 		if err != nil {
// 			return handleStateError(session, err)
// 		}
// 		nextResendState.messageStash = s.messageStash
// 		return nextResendState
// 	}

// 	if s.resendRangeEnd >= session.store.NextTargetMsgSeqNum() {
// 		return s
// 	}

// 	for len(s.messageStash) > 0 {
// 		targetSeqNum := session.store.NextTargetMsgSeqNum()
// 		msg, ok := s.messageStash[targetSeqNum]
// 		if !ok {
// 			break
// 		}

// 		delete(s.messageStash, targetSeqNum)

// 		nextState = inSession{}.FixMsgIn(session, msg)
// 		if !nextState.IsLoggedOn() {
// 			return
// 		}
// 	}

// 	return
// }

#[cfg(test)]
mod tests {

    // import (
    // 	"testing"

    // 	"github.com/stretchr/testify/suite"

    // 	"github.com/quickfixgo/quickfix/internal"
    // )

    // type resendStateTestSuite struct {
    // 	SessionSuiteRig
    // }

    // func TestResendStateTestSuite(t *testing.T) {
    // 	suite.Run(t, new(resendStateTestSuite))
    // }

    // func (s *resendStateTestSuite) SetupTest() {
    // 	s.Init()
    // 	s.session.State = resendState{}
    // }

    // func (s *resendStateTestSuite) TestIsLoggedOn() {
    // 	s.True(s.session.IsLoggedOn())
    // }

    // func (s *resendStateTestSuite) TestIsConnected() {
    // 	s.True(s.session.IsConnected())
    // }

    // func (s *resendStateTestSuite) TestIsSessionTime() {
    // 	s.True(s.session.IsSessionTime())
    // }

    // func (s *resendStateTestSuite) TestTimeoutPeerTimeout() {
    // 	s.MockApp.On("ToAdmin")
    // 	s.session.Timeout(s.session, internal.PeerTimeout)

    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.State(pendingTimeout{resendState{}})
    // }

    // func (s *resendStateTestSuite) TestTimeoutUnchangedIgnoreLogonLogoutTimeout() {
    // 	tests := []internal.Event{internal.LogonTimeout, internal.LogoutTimeout}

    // 	for _, event := range tests {
    // 		s.session.Timeout(s.session, event)
    // 		s.State(resendState{})
    // 	}
    // }

    // func (s *resendStateTestSuite) TestTimeoutUnchangedNeedHeartbeat() {
    // 	s.MockApp.On("ToAdmin")
    // 	s.session.Timeout(s.session, internal.NeedHeartbeat)

    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.State(resendState{})
    // }

    // func (s *resendStateTestSuite) TestFixMsgIn() {
    // 	s.session.State = inSession{}

    // 	// In session expects seq number 1, send too high.
    // 	s.MessageFactory.SetNextSeqNum(2)
    // 	s.MockApp.On("ToAdmin")

    // 	msgSeqNum2 := s.NewOrderSingle()
    // 	s.fixMsgIn(s.session, msgSeqNum2)

    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.State(resendState{})
    // 	s.LastToAdminMessageSent()
    // 	s.MessageType(string(msgTypeResendRequest), s.MockApp.lastToAdmin)
    // 	s.FieldEquals(tagBeginSeqNo, 1, s.MockApp.lastToAdmin.Body)
    // 	s.NextTargetMsgSeqNum(1)

    // 	msgSeqNum3 := s.NewOrderSingle()
    // 	s.fixMsgIn(s.session, msgSeqNum3)
    // 	s.State(resendState{})
    // 	s.NextTargetMsgSeqNum(1)

    // 	msgSeqNum4 := s.NewOrderSingle()
    // 	s.fixMsgIn(s.session, msgSeqNum4)

    // 	s.State(resendState{})
    // 	s.NextTargetMsgSeqNum(1)

    // 	s.MessageFactory.SetNextSeqNum(1)
    // 	s.MockApp.On("FromApp").Return(nil)
    // 	s.fixMsgIn(s.session, s.NewOrderSingle())

    // 	s.MockApp.AssertNumberOfCalls(s.T(), "FromApp", 4)
    // 	s.State(inSession{})
    // 	s.NextTargetMsgSeqNum(5)
    // }

    // func (s *resendStateTestSuite) TestFixMsgInSequenceReset() {
    // 	s.session.State = inSession{}

    // 	// In session expects seq number 1, send too high.
    // 	s.MessageFactory.SetNextSeqNum(3)
    // 	s.MockApp.On("ToAdmin")

    // 	msgSeqNum3 := s.NewOrderSingle()
    // 	s.fixMsgIn(s.session, msgSeqNum3)

    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.State(resendState{})
    // 	s.LastToAdminMessageSent()
    // 	s.MessageType(string(msgTypeResendRequest), s.MockApp.lastToAdmin)
    // 	s.FieldEquals(tagBeginSeqNo, 1, s.MockApp.lastToAdmin.Body)
    // 	s.NextTargetMsgSeqNum(1)

    // 	s.MessageFactory.SetNextSeqNum(1)
    // 	s.MockApp.On("FromAdmin").Return(nil)
    // 	s.fixMsgIn(s.session, s.SequenceReset(2))
    // 	s.NextTargetMsgSeqNum(2)
    // 	s.State(resendState{})

    // 	s.MockApp.On("FromApp").Return(nil)
    // 	s.fixMsgIn(s.session, s.NewOrderSingle())

    // 	s.MockApp.AssertNumberOfCalls(s.T(), "FromApp", 2)
    // 	s.NextTargetMsgSeqNum(4)
    // 	s.State(inSession{})
    // }

    // func (s *resendStateTestSuite) TestFixMsgInResendChunk() {
    // 	s.session.State = inSession{}
    // 	s.ResendRequestChunkSize = 2

    // 	// In session expects seq number 1, send too high.
    // 	s.MessageFactory.SetNextSeqNum(4)
    // 	s.MockApp.On("ToAdmin")

    // 	msgSeqNum4 := s.NewOrderSingle()
    // 	s.fixMsgIn(s.session, msgSeqNum4)

    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.State(resendState{})
    // 	s.LastToAdminMessageSent()
    // 	s.MessageType(string(msgTypeResendRequest), s.MockApp.lastToAdmin)
    // 	s.FieldEquals(tagBeginSeqNo, 1, s.MockApp.lastToAdmin.Body)
    // 	s.FieldEquals(tagEndSeqNo, 2, s.MockApp.lastToAdmin.Body)
    // 	s.NextTargetMsgSeqNum(1)

    // 	msgSeqNum5 := s.NewOrderSingle()
    // 	s.fixMsgIn(s.session, msgSeqNum5)
    // 	s.State(resendState{})
    // 	s.NextTargetMsgSeqNum(1)

    // 	msgSeqNum6 := s.NewOrderSingle()
    // 	s.fixMsgIn(s.session, msgSeqNum6)

    // 	s.State(resendState{})
    // 	s.NextTargetMsgSeqNum(1)

    // 	s.MessageFactory.SetNextSeqNum(1)
    // 	s.MockApp.On("FromApp").Return(nil)
    // 	s.fixMsgIn(s.session, s.NewOrderSingle())

    // 	s.MockApp.AssertNumberOfCalls(s.T(), "FromApp", 1)
    // 	s.State(resendState{})
    // 	s.NextTargetMsgSeqNum(2)

    // 	s.fixMsgIn(s.session, s.NewOrderSingle())
    // 	s.MockApp.AssertNumberOfCalls(s.T(), "FromApp", 2)
    // 	s.State(resendState{})
    // 	s.NextTargetMsgSeqNum(3)

    // 	s.LastToAdminMessageSent()
    // 	s.MessageType(string(msgTypeResendRequest), s.MockApp.lastToAdmin)
    // 	s.FieldEquals(tagBeginSeqNo, 3, s.MockApp.lastToAdmin.Body)
    // 	s.FieldEquals(tagEndSeqNo, 0, s.MockApp.lastToAdmin.Body)
    // }
}