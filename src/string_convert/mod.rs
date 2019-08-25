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
