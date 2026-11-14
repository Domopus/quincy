//! Finds music links in a message and says which service and kind they point to.

use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MusicLink {
    pub url: String,
    pub service: &'static str,
    /// `track`, `album`, `playlist`, `artist`, or `unresolved` for short links
    /// that need a redirect to say what they are.
    pub kind: &'static str,
    pub external_id: Option<String>,
}

/// Every music link in `text`, in order, without duplicates.
pub fn find(text: &str) -> Vec<MusicLink> {
    let mut links: Vec<MusicLink> = Vec::new();
    for word in text.split_whitespace() {
        let word = word.trim_matches(|c: char| {
            matches!(
                c,
                '(' | ')' | '<' | '>' | '"' | '\'' | ',' | '.' | '!' | '?'
            )
        });
        if let Some(link) = parse(word)
            && !links.iter().any(|l| l.url == link.url)
        {
            links.push(link);
        }
    }
    links
}

/// One URL, if it is a music link.
pub fn parse(candidate: &str) -> Option<MusicLink> {
    if !candidate.starts_with("http://") && !candidate.starts_with("https://") {
        return None;
    }
    let url = Url::parse(candidate).ok()?;
    let host = url.host_str()?.trim_start_matches("www.");
    let segments: Vec<&str> = url.path_segments()?.filter(|s| !s.is_empty()).collect();

    let (service, kind, external_id) = match host {
        "open.spotify.com" => {
            // Optional locale prefix: /intl-fr/track/{id}
            let rest = match segments.first() {
                Some(s) if s.starts_with("intl-") => &segments[1..],
                _ => &segments[..],
            };
            let (kind, id) = kind_and_id(rest)?;
            ("spotify", kind, Some(id))
        }
        "spotify.link" => ("spotify", "unresolved", None),
        "deezer.com" => {
            // Optional locale prefix: /fr/track/{id}
            let rest = match segments.first() {
                Some(s) if s.len() == 2 => &segments[1..],
                _ => &segments[..],
            };
            let (kind, id) = kind_and_id(rest)?;
            ("deezer", kind, Some(id))
        }
        "link.deezer.com" | "deezer.page.link" => ("deezer", "unresolved", None),
        _ => return None,
    };

    Some(MusicLink {
        url: candidate.to_owned(),
        service,
        kind,
        external_id,
    })
}

fn kind_and_id(segments: &[&str]) -> Option<(&'static str, String)> {
    let kind = match *segments.first()? {
        "track" => "track",
        "album" => "album",
        "playlist" => "playlist",
        "artist" => "artist",
        _ => return None,
    };
    Some((kind, (*segments.get(1)?).to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spotify_track_with_locale_and_query() {
        let link =
            parse("https://open.spotify.com/intl-fr/track/2EqlS6tkEnglzr7tkKAAYD?si=abc").unwrap();
        assert_eq!(link.service, "spotify");
        assert_eq!(link.kind, "track");
        assert_eq!(link.external_id.as_deref(), Some("2EqlS6tkEnglzr7tkKAAYD"));
    }

    #[test]
    fn deezer_track_with_locale() {
        let link = parse("https://www.deezer.com/fr/track/916424").unwrap();
        assert_eq!((link.service, link.kind), ("deezer", "track"));
        assert_eq!(link.external_id.as_deref(), Some("916424"));
    }

    #[test]
    fn short_links_are_unresolved() {
        assert_eq!(
            parse("https://spotify.link/AbCd").unwrap().kind,
            "unresolved"
        );
        assert_eq!(
            parse("https://link.deezer.com/s/31abc").unwrap().kind,
            "unresolved"
        );
    }

    #[test]
    fn other_links_and_pages_are_ignored() {
        assert!(parse("https://youtube.com/watch?v=x").is_none());
        assert!(parse("https://open.spotify.com/user/someone").is_none());
        assert!(parse("open.spotify.com/track/x").is_none());
    }

    #[test]
    fn finds_links_inside_text_once() {
        let text = "écoute ça (https://www.deezer.com/track/1) et https://www.deezer.com/track/1 !";
        let links = find(text);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].url, "https://www.deezer.com/track/1");
    }
}
