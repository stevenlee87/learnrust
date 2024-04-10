/*
字符串 slice（string slice）是 String 中一部分值的引用，它看起来像这样：
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
 */
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn str_code() {
    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），整体或全部
    let word = first_word(&my_string[0..6]);
    println!("The value of word1:{}", word);

    let word = first_word(&my_string[..]);
    println!("The value of word2:{}", word);
}
