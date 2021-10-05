extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
    .add(b'<').add(b'>').add(b'`');

pub fn construct_youtube_url(query: &str) -> String {
    if query == "yt" {
        return String::from("https://youtube.com");
    } else if &query[..4] == "yt /" {
        return construct_youtube_user_url(&query[4..])
    } else {
        return construct_youtube_search_url(&query[3..])
    }
}

pub fn construct_youtube_user_url(user: &str) -> String {
    format!("https://youtube.com/{}/videos",user)
}

pub fn construct_youtube_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT);
    let youtube_search_url = format!("https://youtube.com/results?search_query={}", encoded_query);

    return youtube_search_url;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_youtube_search_url() {
        let fake_query = "yt hello";
        assert_eq!(
            construct_youtube_url(fake_query),
            "https://youtube.com/results?search_query=hello"
        );
    }

    #[test]
    fn test_construct_youtube_search_url_with_encoding() {
        let fake_query = "yt hello world";
        assert_eq!(
            construct_youtube_url(fake_query),
            "https://youtube.com/results?search_query=hello%20world"
        );
    }

    #[test]
    fn test_construct_youtube_user_url() {
        let fake_query = "yt /taylorswift";
        assert_eq!(
            construct_youtube_url(fake_query),
            "https://youtube.com/taylorswift/videos"
        );
    }
}