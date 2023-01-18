use std::collections::BTreeMap;

use crate::extension::Extension;
use crate::extension::util::remove_extension_value;

/// An Podcast channel element extension.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct PodcastChannelExtension {

    /// This element is used to declare a unique, global identifier for a podcast.
    /// The value is a UUIDv5, and is easily generated from the RSS feed url,
    ///  with the protocol scheme and trailing slashes stripped off,
    ///  combined with a unique "podcast" namespace which has a UUID of ead4c236-bf58-58c6-a2c6-a6b28d128cb6
    pub guid: Option<String>,
}

impl PodcastChannelExtension {
    /// Create an `ITunesChannelExtension` from a `BTreeMap`.
    pub fn from_map(mut map: BTreeMap<String, Vec<Extension>>) -> Self {
        Self {
            guid: remove_extension_value(&mut map, "guid"),
        }
    }
}