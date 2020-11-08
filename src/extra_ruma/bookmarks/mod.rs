use matrix_sdk::identifiers::EventId;
use serde::{Serialize, Deserialize};

pub mod global;
pub mod room;


/// Information about a tag.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct Bookmark {
    pub event_id: EventId,
    pub title: String,
    pub comment: String,
}
