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
  let temp = vec![vec![]];

  let get_exist_indexes = |vec: &Vec<String>| {

  };

  if word_list.contains(&end_word) {
    for i in 0..end_word.len() {

    }
  }

  temp
}

#[test]
fn test_find_ladders() {
  fn vec_string(str_array: &[&str]) -> Vec<String> {
    str_array
      .iter()
      .map(|v| v.to_string())
      .collect::<Vec<String>>()
  }

  assert_eq!(
    find_ladders(
      "hit".to_string(),
      "cog".to_string(),
      vec_string(&["hot", "dot", "dog", "lot", "log", "cog"])
    ),
    vec![
      vec_string(&["hit", "hot", "dot", "dog", "cog"]),
      vec_string(&["hit", "hot", "lot", "log", "cog"])
    ]
  );
}
