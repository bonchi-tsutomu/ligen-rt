use rand::random_range;

const ASCII_LETTERS_DIGIT: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub fn generate_id(length: usize) -> String {
    let mut out = String::new();
    let chars: Vec<char> = ASCII_LETTERS_DIGIT.chars().collect();

    for _ in 0..length {
        let idx = random_range(0..chars.len());
        if let Some(char) = chars.get(idx) {
            out.push(*char);
        }
    }

    out
}