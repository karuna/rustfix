use crate::session::session_state::LoggedOn;
use delegate::delegate;

#[derive(Default)]
pub struct InSession {
    pub logged_on: LoggedOn,
}

impl ToString for InSession {
    fn to_string(&self) -> String {
        String::from("In Session")
    }
}

impl InSession {
    delegate! {
        to self.logged_on {
            pub fn is_connected(&self) -> bool;
            pub fn is_session_time(&self) -> bool;
            pub fn is_logged_on(&self) -> bool;

        }
    }
}

#[cfg(test)]
mod tests {

    // import (
    // 	"testing"
    // 	"time"

    // 	"github.com/stretchr/testify/suite"

    // 	"github.com/quickfixgo/quickfix/internal"
    // )

    // type InSessionTestSuite struct {
    // 	SessionSuiteRig
    // }

    // func TestInSessionTestSuite(t *testing.T) {
    // 	suite.Run(t, new(InSessionTestSuite))
    // }

    // func (s *InSessionTestSuite) SetupTest() {
    // 	s.Init()
    // 	s.session.State = inSession{}
    // }

    // func (s *InSessionTestSuite) TestPreliminary() {
    // 	s.True(s.session.is_logged_on())
    // 	s.True(s.session.IsConnected())
    // 	s.True(s.session.IsSessionTime())
    // }

    // func (s *InSessionTestSuite) TestLogout() {
    // 	s.MockApp.On("FromAdmin").Return(nil)
    // 	s.MockApp.On("ToAdmin")
    // 	s.MockApp.On("OnLogout")
    // 	s.session.fixMsgIn(s.session, s.Logout())

    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.State(latentState{})

    // 	s.LastToAdminMessageSent()
    // 	s.MessageType(string(msgTypeLogout), s.MockApp.lastToAdmin)
    // 	s.NextTargetMsgSeqNum(2)
    // 	s.NextSenderMsgSeqNum(2)
    // }

    // func (s *InSessionTestSuite) TestLogoutEnableLastMsgSeqNumProcessed() {
    // 	s.session.EnableLastMsgSeqNumProcessed = true

    // 	s.MockApp.On("FromAdmin").Return(nil)
    // 	s.MockApp.On("ToAdmin")
    // 	s.MockApp.On("OnLogout")
    // 	s.session.fixMsgIn(s.session, s.Logout())

    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.LastToAdminMessageSent()

    // 	s.MessageType(string(msgTypeLogout), s.MockApp.lastToAdmin)
    // 	s.FieldEquals(tagLastMsgSeqNumProcessed, 1, s.MockApp.lastToAdmin.Header)
    // }

    // func (s *InSessionTestSuite) TestLogoutResetOnLogout() {
    // 	s.session.ResetOnLogout = true

    // 	s.MockApp.On("ToApp").Return(nil)
    // 	s.Nil(s.queueForSend(s.NewOrderSingle()))
    // 	s.MockApp.AssertExpectations(s.T())

    // 	s.MockApp.On("FromAdmin").Return(nil)
    // 	s.MockApp.On("ToAdmin")
    // 	s.MockApp.On("OnLogout")
    // 	s.session.fixMsgIn(s.session, s.Logout())

    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.State(latentState{})
    // 	s.LastToAppMessageSent()
    // 	s.LastToAdminMessageSent()
    // 	s.MessageType(string(msgTypeLogout), s.MockApp.lastToAdmin)

    // 	s.NextTargetMsgSeqNum(1)
    // 	s.NextSenderMsgSeqNum(1)
    // 	s.NoMessageQueued()
    // }

    // func (s *InSessionTestSuite) TestTimeoutNeedHeartbeat() {
    // 	s.MockApp.On("ToAdmin").Return(nil)
    // 	s.session.Timeout(s.session, internal.NeedHeartbeat)

    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.State(inSession{})
    // 	s.LastToAdminMessageSent()
    // 	s.MessageType(string(msgTypeHeartbeat), s.MockApp.lastToAdmin)
    // 	s.NextSenderMsgSeqNum(2)
    // }

    // func (s *InSessionTestSuite) TestTimeoutPeerTimeout() {
    // 	s.MockApp.On("ToAdmin").Return(nil)
    // 	s.session.Timeout(s.session, internal.PeerTimeout)

    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.State(pendingTimeout{inSession{}})
    // 	s.LastToAdminMessageSent()
    // 	s.MessageType(string(msgTypeTestRequest), s.MockApp.lastToAdmin)
    // 	s.NextSenderMsgSeqNum(2)
    // }

    // func (s *InSessionTestSuite) TestDisconnected() {
    // 	s.MockApp.On("OnLogout").Return(nil)
    // 	s.session.Disconnected(s.session)
    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.State(latentState{})
    // }

    // func (s *InSessionTestSuite) TestStop() {
    // 	s.MockApp.On("ToAdmin")
    // 	s.session.Stop(s.session)

    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.State(logoutState{})
    // 	s.LastToAdminMessageSent()
    // 	s.MessageType(string(msgTypeLogout), s.MockApp.lastToAdmin)

    // 	s.MockApp.On("OnLogout")
    // 	s.session.Timeout(s.session, <-s.sessionEvent)
    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.Stopped()
    // 	s.Disconnected()
    // }

    // func (s *InSessionTestSuite) TestFIXMsgInTargetTooHighEnableLastMsgSeqNumProcessed() {
    // 	s.session.EnableLastMsgSeqNumProcessed = true
    // 	s.MessageFactory.seqNum = 5

    // 	s.MockApp.On("ToAdmin")
    // 	msgSeqNumTooHigh = s.NewOrderSingle()
    // 	s.fixMsgIn(s.session, msgSeqNumTooHigh)

    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.LastToAdminMessageSent()
    // 	s.MessageType(string(msgTypeResendRequest), s.MockApp.lastToAdmin)
    // 	s.FieldEquals(tagLastMsgSeqNumProcessed, 0, s.MockApp.lastToAdmin.Header)
    // }

    // func (s *InSessionTestSuite) TestFIXMsgInTargetTooHigh() {
    // 	s.MessageFactory.seqNum = 5

    // 	s.MockApp.On("ToAdmin")
    // 	msgSeqNumTooHigh = s.NewOrderSingle()
    // 	s.fixMsgIn(s.session, msgSeqNumTooHigh)

    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.LastToAdminMessageSent()
    // 	s.MessageType(string(msgTypeResendRequest), s.MockApp.lastToAdmin)
    // 	s.FieldEquals(TAG_BEGIN_SEQ_NO, 1, s.MockApp.lastToAdmin.Body)
    // 	s.FieldEquals(TAG_END_SEQ_NO, 0, s.MockApp.lastToAdmin.Body)

    // 	resendState, ok = s.session.State.(resendState)
    // 	s.True(ok)
    // 	s.NextTargetMsgSeqNum(1)

    // 	stashedMsg, ok = resendState.messageStash[6]
    // 	s.True(ok)

    // 	rawMsg = msgSeqNumTooHigh.build()
    // 	stashedRawMsg = stashedMsg.build()
    // 	s.Equal(string(rawMsg), string(stashedRawMsg))
    // }
    // func (s *InSessionTestSuite) TestFIXMsgInTargetTooHighResendRequestChunkSize() {
    // 	var tests = []struct {
    // 		chunkSize        int
    // 		expected_end_seq_no int
    // 	}{
    // 		{0, 0},
    // 		{10, 0},
    // 		{5, 0},
    // 		{2, 2},
    // 		{3, 3},
    // 	}

    // 	for _, test = range tests {
    // 		s.SetupTest()
    // 		s.MessageFactory.seqNum = 5
    // 		s.session.ResendRequestChunkSize = test.chunkSize

    // 		s.MockApp.On("ToAdmin")
    // 		msgSeqNumTooHigh = s.NewOrderSingle()
    // 		s.fixMsgIn(s.session, msgSeqNumTooHigh)

    // 		s.MockApp.AssertExpectations(s.T())
    // 		s.LastToAdminMessageSent()
    // 		s.MessageType(string(msgTypeResendRequest), s.MockApp.lastToAdmin)
    // 		s.FieldEquals(TAG_BEGIN_SEQ_NO, 1, s.MockApp.lastToAdmin.Body)
    // 		s.FieldEquals(TAG_END_SEQ_NO, test.expected_end_seq_no, s.MockApp.lastToAdmin.Body)

    // 		resendState, ok = s.session.State.(resendState)
    // 		s.True(ok)
    // 		s.NextTargetMsgSeqNum(1)

    // 		stashedMsg, ok = resendState.messageStash[6]
    // 		s.True(ok)

    // 		rawMsg = msgSeqNumTooHigh.build()
    // 		stashedRawMsg = stashedMsg.build()
    // 		s.Equal(string(rawMsg), string(stashedRawMsg))
    // 	}
    // }

    // func (s *InSessionTestSuite) TestFIXMsgInResendRequestAllAdminExpectGapFill() {
    // 	s.MockApp.On("ToAdmin")
    // 	s.session.Timeout(s.session, internal.NeedHeartbeat)
    // 	s.LastToAdminMessageSent()

    // 	s.session.Timeout(s.session, internal.NeedHeartbeat)
    // 	s.LastToAdminMessageSent()

    // 	s.session.Timeout(s.session, internal.NeedHeartbeat)
    // 	s.LastToAdminMessageSent()

    // 	s.MockApp.AssertNumberOfCalls(s.T(), "ToAdmin", 3)
    // 	s.NextSenderMsgSeqNum(4)

    // 	s.MockApp.On("FromAdmin").Return(nil)
    // 	s.MockApp.On("ToAdmin")
    // 	s.fixMsgIn(s.session, s.ResendRequest(1))

    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.LastToAdminMessageSent()
    // 	s.MessageType(string(msgTypeSequenceReset), s.MockApp.lastToAdmin)
    // 	s.FieldEquals(tagMsgSeqNum, 1, s.MockApp.lastToAdmin.Header)
    // 	s.FieldEquals(tagPossDupFlag, true, s.MockApp.lastToAdmin.Header)
    // 	s.FieldEquals(tagNewSeqNo, 4, s.MockApp.lastToAdmin.Body)
    // 	s.FieldEquals(tagGapFillFlag, true, s.MockApp.lastToAdmin.Body)

    // 	s.NextSenderMsgSeqNum(4)
    // 	s.State(inSession{})
    // }

    // func (s *InSessionTestSuite) TestFIXMsgInResendRequestAllAdminThenApp() {
    // 	s.MockApp.On("ToAdmin")
    // 	s.session.Timeout(s.session, internal.NeedHeartbeat)
    // 	s.LastToAdminMessageSent()

    // 	s.session.Timeout(s.session, internal.NeedHeartbeat)
    // 	s.LastToAdminMessageSent()

    // 	s.MockApp.On("ToApp").Return(nil)
    // 	s.Require().Nil(s.session.send(s.NewOrderSingle()))
    // 	s.LastToAppMessageSent()

    // 	s.MockApp.AssertNumberOfCalls(s.T(), "ToAdmin", 2)
    // 	s.MockApp.AssertNumberOfCalls(s.T(), "ToApp", 1)
    // 	s.NextSenderMsgSeqNum(4)

    // 	s.MockApp.On("FromAdmin").Return(nil)
    // 	s.MockApp.On("ToAdmin")
    // 	s.MockApp.On("ToApp").Return(nil)
    // 	s.fixMsgIn(s.session, s.ResendRequest(1))

    // 	s.MockApp.AssertNumberOfCalls(s.T(), "ToAdmin", 3)
    // 	s.MockApp.AssertNumberOfCalls(s.T(), "ToApp", 2)

    // 	s.LastToAdminMessageSent()
    // 	s.MessageType(string(msgTypeSequenceReset), s.MockApp.lastToAdmin)
    // 	s.FieldEquals(tagMsgSeqNum, 1, s.MockApp.lastToAdmin.Header)
    // 	s.FieldEquals(tagPossDupFlag, true, s.MockApp.lastToAdmin.Header)
    // 	s.FieldEquals(tagNewSeqNo, 3, s.MockApp.lastToAdmin.Body)
    // 	s.FieldEquals(tagGapFillFlag, true, s.MockApp.lastToAdmin.Body)

    // 	s.LastToAppMessageSent()
    // 	s.MessageType("D", s.MockApp.lastToApp)
    // 	s.FieldEquals(tagMsgSeqNum, 3, s.MockApp.lastToApp.Header)
    // 	s.FieldEquals(tagPossDupFlag, true, s.MockApp.lastToApp.Header)

    // 	s.NextSenderMsgSeqNum(4)
    // 	s.State(inSession{})
    // }

    // func (s *InSessionTestSuite) TestFIXMsgInResendRequestNoMessagePersist() {
    // 	s.session.DisableMessagePersist = true

    // 	s.MockApp.On("ToApp").Return(nil)
    // 	s.Require().Nil(s.session.send(s.NewOrderSingle()))
    // 	s.LastToAppMessageSent()

    // 	s.MockApp.AssertNumberOfCalls(s.T(), "ToApp", 1)
    // 	s.NextSenderMsgSeqNum(2)

    // 	s.MockApp.On("FromAdmin").Return(nil)
    // 	s.MockApp.On("ToAdmin")
    // 	s.fixMsgIn(s.session, s.ResendRequest(1))

    // 	s.MockApp.AssertNumberOfCalls(s.T(), "ToAdmin", 1)
    // 	s.MockApp.AssertNumberOfCalls(s.T(), "ToApp", 1)

    // 	s.LastToAdminMessageSent()
    // 	s.MessageType(string(msgTypeSequenceReset), s.MockApp.lastToAdmin)
    // 	s.FieldEquals(tagMsgSeqNum, 1, s.MockApp.lastToAdmin.Header)
    // 	s.FieldEquals(tagPossDupFlag, true, s.MockApp.lastToAdmin.Header)
    // 	s.FieldEquals(tagNewSeqNo, 2, s.MockApp.lastToAdmin.Body)
    // 	s.FieldEquals(tagGapFillFlag, true, s.MockApp.lastToAdmin.Body)

    // 	s.NextSenderMsgSeqNum(2)
    // 	s.State(inSession{})
    // }

    // func (s *InSessionTestSuite) TestFIXMsgInResendRequestDoNotSendApp() {
    // 	s.MockApp.On("ToAdmin")
    // 	s.session.Timeout(s.session, internal.NeedHeartbeat)
    // 	s.LastToAdminMessageSent()

    // 	s.MockApp.On("ToApp").Return(nil)
    // 	s.Require().Nil(s.session.send(s.NewOrderSingle()))
    // 	s.LastToAppMessageSent()

    // 	s.session.Timeout(s.session, internal.NeedHeartbeat)
    // 	s.LastToAdminMessageSent()

    // 	s.MockApp.AssertNumberOfCalls(s.T(), "ToAdmin", 2)
    // 	s.MockApp.AssertNumberOfCalls(s.T(), "ToApp", 1)
    // 	s.NextSenderMsgSeqNum(4)

    // 	// NOTE: a cheat here, need to reset mock.
    // 	s.MockApp = MockApp{}
    // 	s.MockApp.On("FromAdmin").Return(nil)
    // 	s.MockApp.On("ToApp").Return(ErrDoNotSend)
    // 	s.MockApp.On("ToAdmin")
    // 	s.fixMsgIn(s.session, s.ResendRequest(1))

    // 	s.MockApp.AssertNumberOfCalls(s.T(), "ToAdmin", 1)
    // 	s.MockApp.AssertNumberOfCalls(s.T(), "ToApp", 1)

    // 	s.LastToAdminMessageSent()
    // 	s.MessageType(string(msgTypeSequenceReset), s.MockApp.lastToAdmin)
    // 	s.FieldEquals(tagMsgSeqNum, 1, s.MockApp.lastToAdmin.Header)
    // 	s.FieldEquals(tagPossDupFlag, true, s.MockApp.lastToAdmin.Header)
    // 	s.FieldEquals(tagNewSeqNo, 4, s.MockApp.lastToAdmin.Body)
    // 	s.FieldEquals(tagGapFillFlag, true, s.MockApp.lastToAdmin.Body)

    // 	s.NoMessageSent()

    // 	s.NextSenderMsgSeqNum(4)
    // 	s.State(inSession{})
    // }

    // func (s *InSessionTestSuite) TestFIXMsgInTargetTooLow() {
    // 	s.incr_next_target_msg_seq_num()

    // 	s.MockApp.On("ToAdmin")
    // 	s.fixMsgIn(s.session, s.NewOrderSingle())
    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.LastToAdminMessageSent()
    // 	s.MessageType(string(msgTypeLogout), s.MockApp.lastToAdmin)
    // 	s.FieldEquals(tagText, "MsgSeqNum too low, expecting 2 but received 1", s.MockApp.lastToAdmin.Body)
    // 	s.State(logoutState{})
    // }

    // func (s *InSessionTestSuite) TestFIXMsgInTargetTooLowPossDup() {
    // 	s.incr_next_target_msg_seq_num()

    // 	s.MockApp.On("ToAdmin")
    // 	nos = s.NewOrderSingle()
    // 	nos.header.set_field(tagPossDupFlag, FIXBoolean(true))

    // 	s.fixMsgIn(s.session, nos)
    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.LastToAdminMessageSent()
    // 	s.MessageType(string(msgTypeReject), s.MockApp.lastToAdmin)
    // 	s.FieldEquals(tagText, "Required tag missing", s.MockApp.lastToAdmin.Body)
    // 	s.FieldEquals(tagRefTagID, int(tagOrigSendingTime), s.MockApp.lastToAdmin.Body)
    // 	s.State(inSession{})

    // 	nos.header.set_field(tagOrigSendingTime, FIXUTCTimestamp{Time: time.Now().Add(time.Duration(-1) * time.Minute)})
    // 	nos.header.set_field(tagSendingTime, FIXUTCTimestamp{Time: time.Now()})
    // 	s.fixMsgIn(s.session, nos)
    // 	s.MockApp.AssertExpectations(s.T())
    // 	s.NoMessageSent()
    // 	s.State(inSession{})
    // 	s.NextTargetMsgSeqNum(2)
    // }
}
