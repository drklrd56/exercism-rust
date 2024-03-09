use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let result = input.graphemes(true).rev().collect();
    return result;
}