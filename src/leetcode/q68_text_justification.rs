/**
 * 68. Text Justification
 * https://leetcode.com/problems/text-justification/
 *
 * Given an array of words and a width maxWidth, format the text such that each line has exactly maxWidth characters and is fully (left and right) justified.
 *
 * You should pack your words in a greedy approach; that is, pack as many words as you can in each line. Pad extra spaces ' ' when necessary so that each line has exactly maxWidth characters.
 *
 * Extra spaces between words should be distributed as evenly as possible. If the number of spaces on a line do not divide evenly between words, the empty slots on the left will be assigned more spaces than the slots on the right.
 *
 * For the last line of text, it should be left justified and no extra space is inserted between words.
 *
 * Note:
 *
 * A word is defined as a character sequence consisting of non-space characters only.
 * Each word's length is guaranteed to be greater than 0 and not exceed maxWidth.
 * The input array words contains at least one word.
 * Example 1:
 *
 * Input:
 * words = ["This", "is", "an", "example", "of", "text", "justification."]
 * maxWidth = 16
 * Output:
 * [
 *    "This    is    an",
 *    "example  of text",
 *    "justification.  "
 * ]
 * Example 2:
 *
 * Input:
 * words = ["What","must","be","acknowledgment","shall","be"]
 * maxWidth = 16
 * Output:
 * [
 *   "What   must   be",
 *   "acknowledgment  ",
 *   "shall be        "
 * ]
 * Explanation: Note that the last line is "shall be    " instead of "shall     be",
 *              because the last line must be left-justified instead of fully-justified.
 *              Note that the second line is also left-justified becase it contains only one word.
 * Example 3:
 *
 * Input:
 * words = ["Science","is","what","we","understand","well","enough","to","explain",
 *          "to","a","computer.","Art","is","everything","else","we","do"]
 * maxWidth = 20
 * Output:
 * [
 *   "Science  is  what we",
 *   "understand      well",
 *   "enough to explain to",
 *   "a  computer.  Art is",
 *   "everything  else  we",
 *   "do                  "
 * ]
 */

fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
  let (mut out, mut temp): (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());
  let max_width = max_width as usize;

  for word in words {
    let word_size: usize = temp.iter().map(|v| v.len()).sum();
    if word_size + word.len() + temp.len() > max_width {
      let size = if temp.len() > 2 { temp.len() - 1 } else { 1 };
      let space_size = max_width - word_size;
      let (space_width, extra) = (space_size / size, space_size % size);
      let mut temp_s = "".to_string();
      for i in 0..size {
        temp_s += &temp[i];
        temp_s += &(0..space_width).map(|_| ' ').collect::<String>();
        if i < extra {
          temp_s += &" ";
        }
      }
      if temp.len() > 1 {
        temp_s += temp.last().unwrap();
      }
      out.push(temp_s);
      temp.clear();
    }
    temp.push(word);
  }

  if !temp.is_empty() {
    let mut s = "".to_string();
    for word in temp {
      if !s.is_empty() {
        s.push(' ');
      }
      s += &word;
    }
    for _ in s.len()..max_width {
      s.push(' ');
    }
    out.push(s);
  }

  out
}

#[test]
fn q68_test() {
  assert_eq!(
    full_justify(string_vec!["test"], 4),
    ["test"]
  );
  assert_eq!(
    full_justify(
      string_vec!["What", "must", "be", "acknowledgment", "shall", "be"],
      16
    ),
    ["What   must   be", "acknowledgment  ", "shall be        "]
  );
  assert_eq!(
    full_justify(
      string_vec![
        "Science",
        "is",
        "what",
        "we",
        "understand",
        "well",
        "enough",
        "to",
        "explain",
        "to",
        "a",
        "computer.",
        "Art",
        "is",
        "everything",
        "else",
        "we",
        "do"
      ],
      20
    ),
    [
      "Science  is  what we",
      "understand      well",
      "enough to explain to",
      "a  computer.  Art is",
      "everything  else  we",
      "do                  "
    ]
  );
}
