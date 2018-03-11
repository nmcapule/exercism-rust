use regex::Regex;
extern crate regex;


pub fn reply(message: &str) -> &str {
    let whitespace_re = Regex::new(r"^\s*$").unwrap();
    let has_upper_re = Regex::new(r"^.*[A-Z].*$").unwrap();
    let has_lower_re = Regex::new(r"^.*[a-z].*$").unwrap();
    let question_re = Regex::new(r"^.*\?\s*$").unwrap();
    match message {
        s if has_upper_re.is_match(s) && !has_lower_re.is_match(s) && question_re.is_match(s) => "Calm down, I know what I\'m doing!", 
        s if has_upper_re.is_match(s) && !has_lower_re.is_match(s) => "Whoa, chill out!", 
        s if question_re.is_match(s) => "Sure.",
        s if whitespace_re.is_match(s) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
