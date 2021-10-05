extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
    .add(b'<').add(b'>').add(b'`');

pub fn construct_twitter_url(query: &str) -> String {
    if query == "tw" {
        return String::from("https://twitter.com");
    } else if &query[..4] == "tw @" {
        return construct_twitter_profile_url(&query[4..])
    } else {
        return construct_twitter_search_url(&query[3..])
    }
}

pub fn construct_twitter_profile_url(profile: &str) -> String {
    format!("https://twitter.com/{}",profile)
}

pub fn construct_twitter_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT);
    let twitter_search_url = format!("https://twitter.com/search?q={}", encoded_query);

    return twitter_search_url;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_twitter_url() {
        let fake_query = "tw";
        assert_eq!(construct_twitter_url(fake_query),
                   "https://twitter.com");
    }

    #[test]
    fn test_construct_twitter_url_query() {
        let fake_query = "tw hello world";
        assert_eq!(construct_twitter_url(fake_query),
                   "https://twitter.com/search?q=hello%20world");
    }

    #[test]
    fn test_construct_twitter_url_profile() {
        let fake_query = "tw @fbOpenSource";
        assert_eq!(construct_twitter_url(fake_query),
                   "https://twitter.com/fbOpenSource");
    }

    #[test]
    fn test_construct_twitter_profile_url() {
        let fake_profile = "jsjoeio";
        assert_eq!(
            construct_twitter_profile_url(fake_profile),
            "https://twitter.com/jsjoeio"
        );
    }

    #[test]
    fn test_construct_twitter_search_url() {
        let fake_query = "hello world";
        assert_eq!(
            construct_twitter_search_url(fake_query),
            "https://twitter.com/search?q=hello%20world"
        );
    }
}
