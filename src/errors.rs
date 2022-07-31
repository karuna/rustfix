use crate::tag::Tag;
use simple_error::SimpleError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

lazy_static! {
    // ERR_DO_NOT_SEND is a convenience error to indicate a DoNotSend in ToApp
    static ref ERR_DO_NOT_SEND: SimpleError = simple_error!("Do Not Send");
}

const REJECT_REASON_INVALID_TAG_NUMBER: isize = 0;
const REJECT_REASON_REQUIRED_TAG_MISSING: isize = 1;
const REJECT_REASON_TAG_NOT_DEFINED_FOR_THIS_MESSAGE_TYPE: isize = 2;
const REJECT_REASON_UNSUPPORTED_MESSAGE_TYPE: isize = 3;
const REJECT_REASON_TAG_SPECIFIED_WITHOUT_A_VALUE: isize = 4;
const REJECT_REASON_VALUE_IS_INCORRECT: isize = 5;
const REJECT_REASON_CONDITIONALLY_REQUIRED_FIELD_MISSING: isize = 5;
const REJECT_REASON_INCORRECT_DATA_FORMAT_FOR_VALUE: isize = 6;
const REJECT_REASON_COMP_ID_PROBLEM: isize = 9;
const REJECT_REASON_SENDING_TIME_ACCURACY_PROBLEM: isize = 10;
const REJECT_REASON_INVALID_MSG_TYPE: isize = 11;
const REJECT_REASON_TAG_APPEARS_MORE_THAN_ONCE: isize = 13;
const REJECT_REASON_TAG_SPECIFIED_OUT_OF_REQUIRED_ORDER: isize = 14;
const REJECT_REASON_REPEATING_GROUP_FIELDS_OUT_OF_ORDER: isize = 15;
const REJECT_REASON_INCORRECT_NUM_IN_GROUP_COUNT_FOR_REPEATING_GROUP: isize = 16;

// MessageRejectError is a type of error that can correlate to a message reject.
pub trait MessageRejectErrorTrait: Error {
    // reject_reason, tag 373 for session rejects, tag 380 for business rejects.
    fn reject_reason(&self) -> isize;
    fn business_reject_ref_id(&self) -> &str;
    fn ref_tag_id(&self) -> Option<Tag>;
    fn is_business_reject(&self) -> bool;
}

// RejectLogon indicates the application is rejecting permission to logon. Implements MessageRejectError
#[derive(Debug)]
pub struct RejectLogon {
    pub text: String,
}

impl Display for RejectLogon {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.text)
    }
}

impl Error for RejectLogon {
    fn description(&self) -> &str {
        &self.text
    }
}

impl MessageRejectErrorTrait for RejectLogon {
    // reject_reason implements MessageRejectError
    fn reject_reason(&self) -> isize {
        0
    }

    // business_reject_ref_id implements MessageRejectError
    fn business_reject_ref_id(&self) -> &str {
        ""
    }

    // ref_tag_id implements MessageRejectError
    fn ref_tag_id(&self) -> Option<Tag> {
        None
    }

    // is_business_reject implements MessageRejectError
    fn is_business_reject(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct MessageRejectError {
    pub reject_reason: isize,
    pub text: String,
    pub business_reject_ref_id: String,
    pub ref_tag_id: Option<Tag>,
    pub is_business_reject: bool,
}

impl Display for MessageRejectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.text)
    }
}

impl Error for MessageRejectError {
    fn description(&self) -> &str {
        &self.text
    }
}

impl MessageRejectErrorTrait for MessageRejectError {
    fn reject_reason(&self) -> isize {
        self.reject_reason
    }

    fn business_reject_ref_id(&self) -> &str {
        &self.business_reject_ref_id
    }

    // ref_tag_id implements MessageRejectError
    fn ref_tag_id(&self) -> Option<Tag> {
        self.ref_tag_id
    }

    // is_business_reject implements MessageRejectError
    fn is_business_reject(&self) -> bool {
        self.is_business_reject
    }
}

// new_message_reject_error returns a MessageRejectError with the given error message, reject reason, and optional reftagid
pub fn new_message_reject_error(
    err: String,
    reject_reason: isize,
    ref_tag_id: Option<Tag>,
) -> impl MessageRejectErrorTrait {
    MessageRejectError {
        text: err,
        reject_reason,
        ref_tag_id,
        business_reject_ref_id: String::new(),
        is_business_reject: false,
    }
}

// new_business_message_reject_error returns a MessageRejectError with the given error mesage, reject reason, and optional ref_tag_id.
// Reject is treated as a business level reject
pub fn new_business_message_reject_error(
    err: String,
    reject_reason: isize,
    ref_tag_id: Option<Tag>,
) -> impl MessageRejectErrorTrait {
    MessageRejectError {
        text: err,
        reject_reason,
        ref_tag_id,
        business_reject_ref_id: String::new(),
        is_business_reject: true,
    }
}

// new_business_message_reject_error_with_ref_id returns a MessageRejectError with the given error mesage, reject reason, ref_id, and optional ref_tag_id.
// Reject is treated as a business level reject
pub fn new_business_message_reject_error_with_ref_id(
    err: String,
    reject_reason: isize,
    business_reject_ref_id: String,
    ref_tag_id: Option<Tag>,
) -> impl MessageRejectErrorTrait {
    MessageRejectError {
        text: err,
        reject_reason,
        ref_tag_id,
        business_reject_ref_id,
        is_business_reject: true,
    }
}

// incorrect_data_format_for_value returns an error indicating a field that cannot be parsed as the type required.
pub fn incorrect_data_format_for_value(tag: Tag) -> impl MessageRejectErrorTrait {
    new_message_reject_error(
        String::from("Incorrect data format for value"),
        REJECT_REASON_INCORRECT_DATA_FORMAT_FOR_VALUE,
        Some(tag),
    )
}

// repeating_group_fields_out_of_order returns an error indicating a problem parsing repeating groups fields
fn repeating_group_fields_out_of_order(
    tag: Tag,
    mut reason: String,
) -> impl MessageRejectErrorTrait {
    if !reason.is_empty() {
        reason = format!("Repeating group fields out of order ({})", reason)
    } else {
        reason = String::from("Repeating group fields out of order")
    }
    new_message_reject_error(
        reason,
        REJECT_REASON_REPEATING_GROUP_FIELDS_OUT_OF_ORDER,
        Some(tag),
    )
}

// value_is_incorrect returns an error indicating a field with value that is not valid.
pub fn value_is_incorrect(tag: Tag) -> impl MessageRejectErrorTrait {
    new_message_reject_error(
        String::from("Value is incorrect (out of range) for this tag"),
        REJECT_REASON_VALUE_IS_INCORRECT,
        Some(tag),
    )
}

// conditionally_required_field_missing indicates that the requested field could not be found in the FIX message.
pub fn conditionally_required_field_missing(tag: Tag) -> impl MessageRejectErrorTrait {
    new_business_message_reject_error(
        format!("Conditionally Required Field Missing ({})", tag),
        REJECT_REASON_CONDITIONALLY_REQUIRED_FIELD_MISSING,
        Some(tag),
    )
}

// value_is_incorrect_no_tag returns an error indicating a field with value that is not valid.
// FIXME: to be compliant with legacy tests, for certain value issues, do not include ref_tag? (11c_NewSeqNoLess)
fn value_is_incorrect_no_tag() -> impl MessageRejectErrorTrait {
    new_message_reject_error(
        String::from("Value is incorrect (out of range) for this tag"),
        REJECT_REASON_VALUE_IS_INCORRECT,
        None,
    )
}

// invalid_message_type returns an error to indicate an invalid message type
pub fn invalid_message_type() -> impl MessageRejectErrorTrait {
    new_message_reject_error(
        String::from("Invalid MsgType"),
        REJECT_REASON_INVALID_MSG_TYPE,
        None,
    )
}

// unsupported_message_type returns an error to indicate an unhandled message.
pub fn unsupported_message_type() -> impl MessageRejectErrorTrait {
    new_business_message_reject_error(
        String::from("Unsupported Message Type"),
        REJECT_REASON_UNSUPPORTED_MESSAGE_TYPE,
        None,
    )
}

// tag_not_defined_for_this_message_type returns an error for an invalid tag appearing in a message.
pub fn tag_not_defined_for_this_message_type(tag: Tag) -> impl MessageRejectErrorTrait {
    new_message_reject_error(
        String::from("Tag not defined for this message type"),
        REJECT_REASON_TAG_NOT_DEFINED_FOR_THIS_MESSAGE_TYPE,
        Some(tag),
    )
}

// tag_appears_more_than_once return an error for multiple tags in a message not detected as a repeating group.
fn tag_appears_more_than_once(tag: Tag) -> impl MessageRejectErrorTrait {
    new_message_reject_error(
        String::from("Tag appears more than once"),
        REJECT_REASON_TAG_APPEARS_MORE_THAN_ONCE,
        Some(tag),
    )
}

// required_tag_missing returns a validation error when a required field cannot be found in a message.
pub fn required_tag_missing(tag: Tag) -> impl MessageRejectErrorTrait {
    new_message_reject_error(
        String::from("Required tag missing"),
        REJECT_REASON_REQUIRED_TAG_MISSING,
        Some(tag),
    )
}

// incorrect_num_in_group_count_for_repeating_group returns a validation error when the num in group value for a group does not match actual size.
pub fn incorrect_num_in_group_count_for_repeating_group(tag: Tag) -> impl MessageRejectErrorTrait {
    return new_message_reject_error(
        String::from("Incorrect NumInGroup count for repeating group"),
        REJECT_REASON_INCORRECT_NUM_IN_GROUP_COUNT_FOR_REPEATING_GROUP,
        Some(tag),
    );
}

// tag_specified_out_of_required_order returns validation error when the group order does not match the spec.
fn tag_specified_out_of_required_order(tag: Tag) -> impl MessageRejectErrorTrait {
    new_message_reject_error(
        String::from("Tag specified out of required order"),
        REJECT_REASON_TAG_SPECIFIED_OUT_OF_REQUIRED_ORDER,
        Some(tag),
    )
}

// tag_specified_without_a_value returns a validation error for when a field has no value.
pub fn tag_specified_without_a_value(tag: Tag) -> impl MessageRejectErrorTrait {
    new_message_reject_error(
        String::from("Tag specified without a value"),
        REJECT_REASON_TAG_SPECIFIED_WITHOUT_A_VALUE,
        Some(tag),
    )
}

// invalid_tag_number returns a validation error for messages with invalid tags.
pub fn invalid_tag_number(tag: Tag) -> impl MessageRejectErrorTrait {
    new_message_reject_error(
        String::from("Invalid tag number"),
        REJECT_REASON_INVALID_TAG_NUMBER,
        Some(tag),
    )
}

// comp_id_problem creates a reject for msg where msg has invalid comp id values.
fn comp_id_problem() -> impl MessageRejectErrorTrait {
    new_message_reject_error(
        String::from("CompID problem"),
        REJECT_REASON_COMP_ID_PROBLEM,
        None,
    )
}

// sending_time_accuracy_problem creates a reject for a msg with stale or invalid sending time.
fn sending_time_accuracy_problem() -> impl MessageRejectErrorTrait {
    new_message_reject_error(
        String::from("SendingTime accuracy problem"),
        REJECT_REASON_SENDING_TIME_ACCURACY_PROBLEM,
        None,
    )
}

mod tests {
    use super::*;

    #[test]
    fn test_new_message_reject_error() {
        let expected_error_string = "Custom error";
        let expected_reject_reason = 5;
        let expected_ref_tag_id: Tag = 44;
        let expected_is_business_reject = false;

        let msg_rej = new_message_reject_error(
            String::from(expected_error_string),
            expected_reject_reason,
            Some(expected_ref_tag_id),
        );

        assert_eq!(
            msg_rej.to_string(),
            expected_error_string,
            "expected: {}, got: {}\n",
            expected_error_string,
            msg_rej.to_string(),
        );

        assert_eq!(
            msg_rej.reject_reason(),
            expected_reject_reason,
            "expected: {}, got: {}\n",
            expected_reject_reason,
            msg_rej.reject_reason(),
        );

        assert_eq!(
            msg_rej.ref_tag_id(),
            Some(expected_ref_tag_id),
            "expected: {}, got: {}\n",
            expected_ref_tag_id,
            msg_rej.ref_tag_id().unwrap(),
        );

        assert_eq!(
            msg_rej.is_business_reject(),
            expected_is_business_reject,
            "Expected is_business_reject to be false\n",
        )
    }

    #[test]
    fn test_new_business_message_reject_error() {
        let expected_error_string = "Custom error";
        let expected_reject_reason = 5;
        let expected_ref_tag_id: Tag = 44;
        let expected_is_business_reject = true;

        let msg_rej = new_business_message_reject_error(
            String::from(expected_error_string),
            expected_reject_reason,
            Some(expected_ref_tag_id),
        );

        assert_eq!(
            msg_rej.to_string(),
            expected_error_string,
            "expected: {}, got: {}\n",
            expected_error_string,
            msg_rej.to_string(),
        );
        assert_eq!(
            msg_rej.reject_reason(),
            expected_reject_reason,
            "expected: {}, got: {}\n",
            expected_reject_reason,
            msg_rej.reject_reason(),
        );
        assert_eq!(
            msg_rej.ref_tag_id(),
            Some(expected_ref_tag_id),
            "expected: {}, got: {}\n",
            expected_ref_tag_id,
            msg_rej.ref_tag_id().unwrap(),
        );
        assert_eq!(
            msg_rej.is_business_reject(),
            expected_is_business_reject,
            "Expected is_business_reject to be true\n",
        );
    }

    #[test]
    fn test_new_business_message_reject_error_with_ref_id() {
        let expected_error_string = "Custom error";
        let expected_reject_reason = 5;
        let expected_business_reject_ref_id = "1";
        let expected_ref_tag_id: Tag = 44;
        let expected_is_business_reject = true;

        let msg_rej = new_business_message_reject_error_with_ref_id(
            String::from(expected_error_string),
            expected_reject_reason,
            String::from(expected_business_reject_ref_id),
            Some(expected_ref_tag_id),
        );

        assert_eq!(
            msg_rej.to_string(),
            expected_error_string,
            "expected: {}, got: {}\n",
            expected_error_string,
            msg_rej.to_string(),
        );
        assert_eq!(
            msg_rej.reject_reason(),
            expected_reject_reason,
            "expected: {}, got: {}\n",
            expected_reject_reason,
            msg_rej.reject_reason(),
        );
        assert_eq!(
            msg_rej.business_reject_ref_id(),
            expected_business_reject_ref_id,
            "expected: {}, got: {}\n",
            expected_business_reject_ref_id,
            msg_rej.business_reject_ref_id(),
        );
        assert_eq!(
            msg_rej.ref_tag_id(),
            Some(expected_ref_tag_id),
            "expected: {}, got: {}\n",
            expected_ref_tag_id,
            msg_rej.ref_tag_id().unwrap(),
        );
        assert_eq!(
            msg_rej.is_business_reject(),
            expected_is_business_reject,
            "Expected is_business_reject to be true\n",
        );
    }

    #[test]
    fn test_incorrect_data_format_for_value() {
        let expected_error_string = "Incorrect data format for value";
        let expected_reject_reason = 6;
        let expected_ref_tag_id: Tag = 44;
        let expected_is_business_reject = false;

        let msg_rej = incorrect_data_format_for_value(expected_ref_tag_id);

        assert_eq!(
            msg_rej.to_string(),
            expected_error_string,
            "expected: {}, got: {}\n",
            expected_error_string,
            msg_rej.to_string(),
        );
        assert_eq!(
            msg_rej.reject_reason(),
            expected_reject_reason,
            "expected: {}, got: {}\n",
            expected_reject_reason,
            msg_rej.reject_reason(),
        );
        assert_eq!(
            msg_rej.ref_tag_id(),
            Some(expected_ref_tag_id),
            "expected: {}, got: {}\n",
            expected_ref_tag_id,
            msg_rej.ref_tag_id().unwrap(),
        );
        assert_eq!(
            msg_rej.is_business_reject(),
            expected_is_business_reject,
            "Expected is_business_reject to be false\n",
        );
    }

    #[test]
    fn test_value_is_incorrect() {
        let expected_error_string = "Value is incorrect (out of range) for this tag";
        let expected_reject_reason = 5;
        let expected_ref_tag_id: Tag = 44;
        let expected_is_business_reject = false;

        let msg_rej = value_is_incorrect(expected_ref_tag_id);

        assert_eq!(
            msg_rej.ref_tag_id(),
            Some(expected_ref_tag_id),
            "expected: {}, got: {}\n",
            expected_ref_tag_id,
            msg_rej.ref_tag_id().unwrap(),
        );
        assert_eq!(
            msg_rej.to_string(),
            expected_error_string,
            "expected: {}, got: {}\n",
            expected_error_string,
            msg_rej.to_string(),
        );
        assert_eq!(
            msg_rej.reject_reason(),
            expected_reject_reason,
            "expected: {}, got: {}\n",
            expected_reject_reason,
            msg_rej.reject_reason(),
        );
        assert_eq!(
            msg_rej.is_business_reject(),
            expected_is_business_reject,
            "Expected is_business_reject to be false\n",
        );
    }

    #[test]
    fn test_conditionally_required_field_missing() {
        let expected_reject_reason = 5;
        let expected_ref_tag_id: Tag = 44;
        let expected_error_string = format!(
            "Conditionally Required Field Missing ({})",
            expected_ref_tag_id,
        );
        let expected_is_business_reject = true;

        let msg_rej = conditionally_required_field_missing(expected_ref_tag_id);

        assert_eq!(
            msg_rej.ref_tag_id(),
            Some(expected_ref_tag_id),
            "expected: {}, got: {}\n",
            expected_ref_tag_id,
            msg_rej.ref_tag_id().unwrap(),
        );
        assert_eq!(
            msg_rej.to_string(),
            expected_error_string,
            "expected: {}, got: {}\n",
            expected_error_string,
            msg_rej.to_string(),
        );
        assert_eq!(
            msg_rej.reject_reason(),
            expected_reject_reason,
            "expected: {}, got: {}\n",
            expected_reject_reason,
            msg_rej.reject_reason(),
        );
        assert_eq!(
            msg_rej.is_business_reject(),
            expected_is_business_reject,
            "Expected is_business_reject to be true\n",
        );
    }

    #[test]
    fn test_invalid_message_type() {
        let expected_error_string = "Invalid MsgType";
        let expected_reject_reason = 11;
        let expected_is_business_reject = false;

        let msg_rej = invalid_message_type();

        assert_eq!(
            msg_rej.to_string(),
            expected_error_string,
            "expected: {}, got: {}\n",
            expected_error_string,
            msg_rej.to_string(),
        );
        assert_eq!(
            msg_rej.reject_reason(),
            expected_reject_reason,
            "expected: {}, got: {}\n",
            expected_reject_reason,
            msg_rej.reject_reason(),
        );
        assert_eq!(
            msg_rej.ref_tag_id(),
            None,
            "expected: None, got: {}\n",
            msg_rej.ref_tag_id().unwrap(),
        );
        assert_eq!(
            msg_rej.is_business_reject(),
            expected_is_business_reject,
            "Expected is_business_reject to be false\n",
        );
    }

    #[test]
    fn test_unsupported_message_type() {
        let expected_reject_reason = 3;
        let expected_error_string = "Unsupported Message Type";
        let expected_is_business_reject = true;

        let msg_rej = unsupported_message_type();

        assert_eq!(
            msg_rej.ref_tag_id(),
            None,
            "expected: None, got: {}\n",
            msg_rej.ref_tag_id().unwrap(),
        );
        assert_eq!(
            msg_rej.to_string(),
            expected_error_string,
            "expected: {}, got: {}\n",
            expected_error_string,
            msg_rej.to_string(),
        );
        assert_eq!(
            msg_rej.reject_reason(),
            expected_reject_reason,
            "expected: {}, got: {}\n",
            expected_reject_reason,
            msg_rej.reject_reason(),
        );
        assert_eq!(
            msg_rej.is_business_reject(),
            expected_is_business_reject,
            "Expected is_business_reject to be true\n",
        );
    }

    #[test]
    fn test_tag_not_defined_for_this_message_type() {
        let expected_error_string = "Tag not defined for this message type";
        let expected_reject_reason = 2;
        let expected_ref_tag_id: Tag = 44;
        let expected_is_business_reject = false;

        let msg_rej = tag_not_defined_for_this_message_type(expected_ref_tag_id);

        assert_eq!(
            msg_rej.to_string(),
            expected_error_string,
            "expected: {}, got: {}\n",
            expected_error_string,
            msg_rej.to_string(),
        );
        assert_eq!(
            msg_rej.reject_reason(),
            expected_reject_reason,
            "expected: {}, got: {}\n",
            expected_reject_reason,
            msg_rej.reject_reason(),
        );
        assert_eq!(
            msg_rej.ref_tag_id(),
            Some(expected_ref_tag_id),
            "expected: {}, got: {}\n",
            expected_ref_tag_id,
            msg_rej.ref_tag_id().unwrap(),
        );
        assert_eq!(
            msg_rej.is_business_reject(),
            expected_is_business_reject,
            "Expected is_business_reject to be false\n",
        );
    }

    #[test]
    fn test_required_tag_missing() {
        let expected_error_string = "Required tag missing";
        let expected_reject_reason = 1;
        let expected_ref_tag_id: Tag = 44;
        let expected_is_business_reject = false;

        let msg_rej = required_tag_missing(expected_ref_tag_id);

        assert_eq!(
            msg_rej.to_string(),
            expected_error_string,
            "expected: {}, got: {}\n",
            expected_error_string,
            msg_rej.to_string(),
        );
        assert_eq!(
            msg_rej.reject_reason(),
            expected_reject_reason,
            "expected: {}, got: {}\n",
            expected_reject_reason,
            msg_rej.reject_reason(),
        );
        assert_eq!(
            msg_rej.ref_tag_id(),
            Some(expected_ref_tag_id),
            "expected: {}, got: {}\n",
            expected_ref_tag_id,
            msg_rej.ref_tag_id().unwrap(),
        );
        assert_eq!(
            msg_rej.is_business_reject(),
            expected_is_business_reject,
            "Expected is_business_reject to be false\n",
        );
    }

    #[test]
    fn test_tag_specified_without_a_value() {
        let expected_error_string = "Tag specified without a value";
        let expected_reject_reason = 4;
        let expected_ref_tag_id: Tag = 44;
        let expected_is_business_reject = false;

        let msg_rej = tag_specified_without_a_value(expected_ref_tag_id);

        assert_eq!(
            msg_rej.to_string(),
            expected_error_string,
            "expected: {}, got: {}\n",
            expected_error_string,
            msg_rej.to_string(),
        );
        assert_eq!(
            msg_rej.reject_reason(),
            expected_reject_reason,
            "expected: {}, got: {}\n",
            expected_reject_reason,
            msg_rej.reject_reason(),
        );
        assert_eq!(
            msg_rej.ref_tag_id(),
            Some(expected_ref_tag_id),
            "expected: {}, got: {}\n",
            expected_ref_tag_id,
            msg_rej.ref_tag_id().unwrap(),
        );
        assert_eq!(
            msg_rej.is_business_reject(),
            expected_is_business_reject,
            "Expected is_business_reject to be false\n",
        );
    }

    #[test]
    fn test_invalid_tag_number() {
        let expected_error_string = "Invalid tag number";
        let expected_reject_reason = 0;
        let expected_ref_tag_id: Tag = 44;
        let expected_is_business_reject = false;

        let msg_rej = invalid_tag_number(expected_ref_tag_id);

        assert_eq!(
            msg_rej.to_string(),
            expected_error_string,
            "expected: {}, got: {}\n",
            expected_error_string,
            msg_rej.to_string(),
        );
        assert_eq!(
            msg_rej.reject_reason(),
            expected_reject_reason,
            "expected: {}, got: {}\n",
            expected_reject_reason,
            msg_rej.reject_reason(),
        );
        assert_eq!(
            msg_rej.ref_tag_id(),
            Some(expected_ref_tag_id),
            "expected: {}, got: {}\n",
            expected_ref_tag_id,
            msg_rej.ref_tag_id().unwrap(),
        );
        assert_eq!(
            msg_rej.is_business_reject(),
            expected_is_business_reject,
            "Expected is_business_reject to be false\n",
        );
    }
}
