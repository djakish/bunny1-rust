extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<')
    .add(b'>').add(b'`');

pub fn construct_reddit_url(query: &str) -> String {
    if query == "rd" {
        return String::from("https://reddit.com");
    }
    else if &query[..5] == "rd r/" {
	let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
        return format!("https://reddit.com/{}", encoded_query)
    }
    else{
	let encoded_query = utf8_percent_encode(query, FRAGMENT);
    	let reddit_search_url = format!("https://www.reddit.com/search/?q={}", encoded_query);
    	return reddit_search_url;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_reddit_url() {
        let fake_query = "rd";
        assert_eq!(
            construct_reddit_url(fake_query),
            "https://reddit.com"
        );
    }

    #[test]
    fn test_construct_reddit_subreddit_url() {
        let fake_query = "rd r/all";
        assert_eq!(
            construct_reddit_url(fake_query),
            "https://reddit.com/r/all"
        );
    }
}
