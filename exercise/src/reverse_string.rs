pub fn reverse(input: &str) -> String {
    if input == "" {
        return "".to_string();
    }

    let chars = input.chars();
    let reversed = chars.into_iter().rev();

    let mut rev_chars: Vec<char> = vec![];
    reversed.for_each(|x| rev_chars.push(x));
    String::from_iter(rev_chars)
}
