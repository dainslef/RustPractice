/**
 * 72. Edit Distance
 * https://leetcode.com/problems/edit-distance/
 *
 * Given two words word1 and word2, find the minimum number of operations required to convert word1 to word2.
 *
 * You have the following 3 operations permitted on a word:
 *
 * Insert a character
 * Delete a character
 * Replace a character
 * Example 1:
 *
 * Input: word1 = "horse", word2 = "ros"
 * Output: 3
 * Explanation:
 * horse -> rorse (replace 'h' with 'r')
 * rorse -> rose (remove 'r')
 * rose -> ros (remove 'e')
 * Example 2:
 *
 * Input: word1 = "intention", word2 = "execution"
 * Output: 5
 * Explanation:
 * intention -> inention (remove 't')
 * inention -> enention (replace 'i' with 'e')
 * enention -> exention (replace 'n' with 'x')
 * exention -> exection (replace 'n' with 'c')
 * exection -> execution (insert 'u')
 */

// solution: dynamic programming
fn min_distance(word1: String, word2: String) -> i32 {
  let (length1, length2) = (word1.len(), word2.len());
  let (chars1, chars2): (Vec<char>, Vec<char>) = (word1.chars().collect(), word2.chars().collect());
  let mut temp: Vec<Vec<i32>> = (0..=length1)
    .map(|y| {
      let init = |x| if y == 0 { x } else if x == 0 { y } else { 0 } as i32;
      (0..=length2).map(init).collect()
    })
    .collect();

  for y in 1..=length1 {
    for x in 1..=length2 {
      temp[y][x] = if chars1[y - 1] == chars2[x - 1] {
        temp[y - 1][x - 1] // if the chars are matched, use the last value
      } else {
        temp[y - 1][x - 1].min(temp[y][x - 1]).min(temp[y - 1][x]) + 1 // if the chars aren't match, compare and get the minimum value then plus the value
      }
    }
  }

  temp[length1][length2]
}

#[test]
fn q72_test() {
  assert_eq!(min_distance("abcdbcedcdf".into(), "abcdcd".into()), 5); // need to avoid duplicate count the same char match
  assert_eq!(min_distance("".into(), "".into()), 0);
  assert_eq!(min_distance("horse".into(), "".into()), 5);
  assert_eq!(min_distance("ros".into(), "horse".into()), 3);
  assert_eq!(min_distance("horse".into(), "ros".into()), 3);
  assert_eq!(min_distance("intention".into(), "execution".into()), 5);
  assert_eq!(
    min_distance("zoologicoarchaeologist".into(), "zoopathologist".into()),
    10
  );
  assert_eq!(
    min_distance(
      "pneumonoultramicroscopicsilicovolcanoconiosis".into(),
      "ultramicroscopically".into()
    ),
    27
  );
}
