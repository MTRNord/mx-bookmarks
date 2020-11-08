//! Types for the *dev.nordgedanken.bookmarks.global* event.
//! https://github.com/MTRNord/matrix-doc/blob/bookmarks-spec/proposals/2771-bookmarks.md#alternatives

use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use matrix_sdk::events::macros::BasicEventContent;
use matrix_sdk::identifiers::{RoomId};
use serde::{Deserialize, Serialize};
use crate::extra_ruma::bookmarks::Bookmark;

/// Informs the client about the events that are bookmarked by the user.
pub type BookmarksEvent = matrix_sdk::events::BasicEvent<BookmarksEventContent>;

/// The payload for `BookmarksEvent`.
///
/// A mapping of `RoomId`s to a list of `EventId`s which are considered *bookmarked* for that
/// particular room.
#[derive(Clone, Debug, Deserialize, Serialize, BasicEventContent)]
#[ruma_event(type = "dev.nordgedanken.bookmarks.global")]
pub struct BookmarksEventContent(pub BTreeMap<RoomId, Vec<Bookmark>>);

impl Deref for BookmarksEventContent {
    type Target = BTreeMap<RoomId, Vec<Bookmark>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BookmarksEventContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, convert::TryFrom};

    use matrix_sdk::Raw;
    use matrix_sdk::identifiers::{RoomId,  EventId};
    use serde_json::{from_value as from_json_value, json, to_value as to_json_value};

    use super::{BookmarksEvent, BookmarksEventContent};
    use crate::extra_ruma::bookmarks::Bookmark;
    use std::borrow::Cow;

    #[test]
    fn serialization() {
        let mut content = BookmarksEventContent(BTreeMap::new());
        let event = EventId::try_from("$h29iv0s8:example.com").unwrap();
        let bookmark = Bookmark {
            event_id: event.clone(),
            title: "test".into(),
            comment: "test".into()        };
        let events = vec![bookmark.clone()];
        let room = RoomId::try_from("!123:ruma.io").unwrap();

        content.insert(room.clone(),events.clone() );

        let event = BookmarksEvent { content };
        let json_data = json!({
            "content": {
                room.to_string(): vec![
                    json!({
                        "event_id": events[0].event_id,
                        "title": events[0].title,
                        "comment": events[0].comment,
                    })
               ]
            },
            "type": "dev.nordgedanken.bookmarks.global"
        });

        assert_eq!(to_json_value(&event).unwrap(), json_data);
    }

    #[test]
    fn deserialization() {
        let room = RoomId::try_from("!123:ruma.io").unwrap();
        let bookmark_one = Bookmark {
            event_id: EventId::try_from("$h29iv0s8:example.com").unwrap(),
            title: Default::default(),
            comment: Default::default()
        };
        let bookmark_two = Bookmark {
            event_id: EventId::try_from("$h29iv0s9:example.com").unwrap(),
            title: Default::default(),
            comment: Default::default()
        };
        let events = vec![bookmark_one, bookmark_two];

        let json_data = json!({
            "content": {
                room.to_string(): vec![
                        json!({
                            "event_id": events[0].event_id,
                            "title": events[0].title,
                            "comment": events[0].comment,
                        }),
                        json!({
                            "event_id": events[1].event_id,
                            "title": events[1].title,
                            "comment": events[1].comment,
                        }),
                    ],
            },
            "type": "dev.nordgedanken.bookmarks.global"
        });

        let event: BookmarksEvent =
            from_json_value::<Raw<_>>(json_data).unwrap().deserialize().unwrap();
        let bookmarks = event.content.get(&room).unwrap();

        assert!(bookmarks.contains(&events[0]));
        assert!(bookmarks.contains(&events[1]));
    }
}
