/// return a pig_latin word
///
/// #Arguments
///
/// * `word` - A string slice you want to convert to a pig_latin word
///
/// # Example
///
/// ```
/// // The first consonant of each word is moved to the end of the word and “a
/// // is added, so “first” becomes “irst-fay.” Words that start with a vowel have
/// //“hay” added to the end instead (“apple” becomes “apple-hay”).
/// ```
pub fn convert_to_pig_latin(word: &str) -> String {
    let pig_latin;

    if !is_start_with_vowel(word) {
        let mut end_word = String::from(word[..1].to_string());
        end_word.push_str("ay");
        pig_latin = format!("{}-{}", word[1..].to_string(), end_word);
    } else {
        pig_latin = format!("{}-{}", word, "hay");
    }

    pig_latin
}

/// Returns true if the word start with a vowel, other wise return false
///
/// # Arguments
///
/// * `word` - The word you want to determine
///
/// # Example
///
/// If we put "apple" as argument this function will return true.
fn is_start_with_vowel(word: &str) -> bool {
    let mut lower_case_word = String::from(word);
    lower_case_word.make_ascii_lowercase();

    match &lower_case_word[..1] {
        "a" => true,
        "e" => true,
        "i" => true,
        "o" => true,
        "u" => true,
        _ => false,
    }
}
