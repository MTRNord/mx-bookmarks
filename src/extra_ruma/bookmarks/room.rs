//! Types for the *dev.nordgedanken.bookmarks.room* event.
//! https://github.com/MTRNord/matrix-doc/blob/bookmarks-spec/proposals/2771-bookmarks.md

use matrix_sdk::events::macros::BasicEventContent;
use serde::{Deserialize, Serialize};

use matrix_sdk::events::BasicEvent;
use crate::extra_ruma::bookmarks::Bookmark;

/// Informs the client of bookmarks in a room.
pub type BookmarksEvent = BasicEvent<BookmarksEventContent>;

/// Map of Bookmarks.
pub type Bookmarks = Vec<Bookmark>;

/// The payload for `BookmarksEvent`.
#[derive(Clone, Debug, Deserialize, Serialize, BasicEventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "dev.nordgedanken.bookmarks.room")]
pub struct BookmarksEventContent {
    /// Map of Bookmarks.
    pub bookmarks: Bookmarks,
}

impl BookmarksEventContent {
    /// Creates a new `BookmarksEventContent` with the given `Bookmarks.
    pub fn new(bookmarks: Bookmarks) -> Self {
        Self { bookmarks }
    }
}

impl From<Bookmarks> for BookmarksEventContent {
    fn from(bookmarks: Bookmarks) -> Self {
        Self::new(bookmarks)
    }
}
