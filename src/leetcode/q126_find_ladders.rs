/**
 * Given two words (beginWord and endWord), and a dictionary's word list, find all shortest transformation sequence(s) from beginWord to endWord, such that:
 *
 * Only one letter can be changed at a time
 * Each transformed word must exist in the word list. Note that beginWord is not a transformed word.
 * Note:
 *
 * Return an empty list if there is no such transformation sequence.
 * All words have the same length.
 * All words contain only lowercase alphabetic characters.
 * You may assume no duplicates in the word list.
 * You may assume beginWord and endWord are non-empty and are not the same.
 * Example 1:
 *
 * Input:
 * beginWord = "hit",
 * endWord = "cog",
 * wordList = ["hot","dot","dog","lot","log","cog"]
 *
 * Output:
 * [
 *   ["hit","hot","dot","dog","cog"],
 *   ["hit","hot","lot","log","cog"]
 * ]
 * Example 2:
 *
 * Input:
 * beginWord = "hit"
 * endWord = "cog"
 * wordList = ["hot","dot","dog","lot","log"]
 *
 * Output: []
 *
 * Explanation: The endWord "cog" is not in wordList, therefore no possible transformation.
 */

fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
  let mut temp: Vec<Vec<String>> = Vec::new();
  let mut min_size = 0;

  fn dfs(
    current: &String,
    input_list: Vec<String>,
    mut output_list: Vec<String>,
    end_word: &String,
    temp: &mut Vec<Vec<String>>,
    min_size: &mut usize,
  ) {
    if check_word(&current, end_word) {
      output_list.push(end_word.clone());
      let size = output_list.len();
      if *min_size == 0 || size < *min_size {
        *min_size = size;
      }
      temp.push(output_list);
      return;
    }
    for i in 0..input_list.len() {
      let next = &input_list[i];
      if check_word(&current, &next) {
        let (mut new_output_list, mut new_input_list) = (output_list.clone(), input_list.clone());
        new_output_list.push(next.clone());
        new_input_list.remove(i);
        dfs(
          next,
          new_input_list,
          new_output_list,
          end_word,
          temp,
          min_size,
        );
      }
    }
  }

  if word_list.contains(&end_word) {
    dfs(
      &begin_word,
      word_list,
      vec![begin_word.clone()],
      &end_word,
      &mut temp,
      &mut min_size,
    );
  }

  dbg!((min_size, &temp));

  temp
    .into_iter()
    .filter(|v| v.last().map(|s| s == &end_word).unwrap_or(false))
    .filter(|v| v.len() == min_size)
    .collect::<Vec<Vec<String>>>()
}

fn check_word(old_word: &String, new_word: &String) -> bool {
  let mut count = 0;
  let (old_chars, new_chars) = (
    old_word.chars().collect::<Vec<char>>(),
    new_word.chars().collect::<Vec<char>>(),
  );
  for i in 0..old_chars.len() {
    if old_chars[i] != new_chars[i] {
      count += 1;
      if count > 1 {
        return false;
      }
    }
  }
  count == 1
}

#[test]
fn test_find_ladders() {
  fn vec_string(str_array: &[&str]) -> Vec<String> {
    str_array
      .iter()
      .map(|v| v.to_string())
      .collect::<Vec<String>>()
  }

  fn is_same(input1: Vec<Vec<String>>, input2: Vec<Vec<String>>) -> bool {
    use std::collections::HashSet;
    fn vec_to_set(input: Vec<Vec<String>>) -> HashSet<Vec<String>> {
      let mut set = HashSet::new();
      for i in input {
        set.insert(i);
      }
      set
    }
    vec_to_set(input1) == vec_to_set(input2)
  }

  assert!(is_same(
    find_ladders(
      "hit".to_string(),
      "cog".to_string(),
      vec_string(&["hot", "dot", "dog", "lot", "log", "cog"])
    ),
    vec![
      vec_string(&["hit", "hot", "lot", "log", "cog"]),
      vec_string(&["hit", "hot", "dot", "dog", "cog"])
    ]
  ));

  assert_eq!(
    find_ladders(
      "hot".to_string(),
      "dog".to_string(),
      vec_string(&["hot", "dog", "dot"])
    ),
    vec![vec_string(&["hot", "dot", "dog"])]
  );

  assert_eq!(
    find_ladders(
      "a".to_string(),
      "c".to_string(),
      vec_string(&["a", "b", "c"])
    ),
    vec![vec_string(&["a", "c"])]
  );

  let v: Vec<Vec<String>> = vec![];
  assert_eq!(
    find_ladders(
      "hit".to_string(),
      "cog".to_string(),
      vec_string(&["hot", "dot", "dog", "lot", "log"])
    ),
    v
  );

  println!(
    "{:?}",
    find_ladders(
      "qa".to_string(),
      "sq".to_string(),
      vec_string(&[
        "si", "go", "se", "cm", "so", "ph", "mt", "db", "mb", "sb", "kr", "ln", "tm", "le", "av",
        "sm", "ar", "ci", "ca", "br", "ti", "ba", "to", "ra", "fa", "yo", "ow", "sn", "ya", "cr",
        "po", "fe", "ho", "ma", "re", "or", "rn", "au", "ur", "rh", "sr", "tc", "lt", "lo", "as",
        "fr", "nb", "yb", "if", "pb", "ge", "th", "pm", "rb", "sh", "co", "ga", "li", "ha", "hz",
        "no", "bi", "di", "hi", "qa", "pi", "os", "uh", "wm", "an", "me", "mo", "na", "la", "st",
        "er", "sc", "ne", "mn", "mi", "am", "ex", "pt", "io", "be", "fm", "ta", "tb", "ni", "mr",
        "pa", "he", "lr", "sq", "ye"
      ])
    )
  );
}
