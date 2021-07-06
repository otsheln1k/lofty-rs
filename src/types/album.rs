use super::picture::Picture;

/// A struct for representing an album for convenience.
#[derive(Debug, Clone)]
pub struct Album<'a> {
	/// The title of the album
	pub title: Option<&'a str>,
	/// A `Vec` of the album artists
	pub artist: Option<&'a str>,
	/// The album's covers (Front, Back)
	pub covers: (Option<Picture>, Option<Picture>),
}

impl<'a> Default for Album<'a> {
	fn default() -> Self {
		Self {
			title: None,
			artist: None,
			covers: (None, None),
		}
	}
}

impl<'a> Album<'a> {
	/// Create a new `Album`
	pub fn new(
		title: Option<&'a str>,
		artist: Option<&'a str>,
		covers: (Option<Picture>, Option<Picture>),
	) -> Self {
		Self {
			title,
			artist,
			covers,
		}
	}
	/// Create a new album with the specified title
	pub fn with_title(title: &'a str) -> Self {
		Self {
			title: Some(title),
			artist: None,
			covers: (None, None),
		}
	}
	/// Set the album artists
	pub fn set_artists(&mut self, artist_str: &'a str) {
		self.artist = Some(artist_str);
	}
	/// Clears the `artists` field
	pub fn remove_artist(mut self) {
		self.artist = None
	}
	/// Set the album cover
	pub fn set_covers(&mut self, covers: (Option<Picture>, Option<Picture>)) {
		self.covers = covers
	}
	/// Clears the `covers` field
	pub fn remove_covers(mut self) {
		self.covers = (None, None)
	}
}
