/**
 * 79. Word Search
 * https://leetcode.com/problems/word-search/
 *
 * Given a 2D board and a word, find if the word exists in the grid.
 *
 * The word can be constructed from letters of sequentially adjacent cell, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once.
 *
 * Example:
 *
 * board =
 * [
 *   ['A','B','C','E'],
 *   ['S','F','C','S'],
 *   ['A','D','E','E']
 * ]
 *
 * Given word = "ABCCED", return true.
 * Given word = "SEE", return true.
 * Given word = "ABCB", return false.
 */

/**
 * Runtime: 44 ms
 * Memory Usage: 3.5 MB
 */
fn exist(board: Vec<Vec<char>>, word: String) -> bool {
  use std::collections::{HashSet, VecDeque};

  if board.is_empty() || board[0].is_empty() {
    return false;
  }

  fn recurse(
    board: &Vec<Vec<char>>,
    word: &mut VecDeque<char>,
    index: Option<(usize, usize)>,
    indexes: &mut HashSet<(usize, usize)>,
  ) -> bool {
    word
      .pop_front()
      .map(|head| {
        macro_rules! check {
          ($y: expr, $x: expr) => {
            check!($y, $x, true);
          };
          ($y: expr, $x: expr, $condition: expr) => {
            if $condition && !indexes.contains(&($y, $x)) && head == board[$y][$x] {
              indexes.insert(($y, $x));
              if recurse(board, word, Some(($y, $x)), indexes) {
                return true; // find the target and break right now (avoid useless time cost)
              }
              indexes.remove(&($y, $x));
            }
          };
        }

        let (row, column) = (board.len(), board[0].len());
        if let Some((y, x)) = index {
          check!(y - 1, x, y > 0);
          check!(y, x - 1, x > 0);
          check!(y + 1, x, y + 1 < row);
          check!(y, x + 1, x + 1 < column);
        } else {
          for y in 0..row {
            for x in 0..column {
              check!(y, x);
            }
          }
        }

        word.push_front(head);
        false
      })
      .unwrap_or(true)
  }

  recurse(
    &board,
    &mut word.chars().collect(),
    None,
    &mut HashSet::new(),
  )
}

/**
 * Runtime: 48 ms, faster than 10.81% of Rust online submissions for Word Search.
 * Memory Usage: 3.5 MB, less than 52.94% of Rust online submissions for Word Search.
 */
fn exist_loop(board: Vec<Vec<char>>, word: String) -> bool {
  if board.is_empty() || board[0].is_empty() {
    return false;
  }

  use std::collections::HashMap;

  let (row, column) = (board.len(), board[0].len());
  let chars: Vec<char> = word.chars().collect();
  let mut cache: HashMap<usize, Vec<(usize, usize)>> = vec![(
    0,
    (0..row)
      .flat_map(|y| (0..column).map(move |x| (y, x)))
      .filter(|(y, x)| board[*y][*x] == chars[0])
      .collect(),
  )]
  .into_iter()
  .collect();

  let mut index = 0;
  let mut indexes: Vec<(usize, usize)> = vec![];

  loop {
    if (word.len() > 1 && index + 1 == word.len())
      || word
        == indexes
          .iter()
          .map(|(y, x)| board[*y][*x])
          .collect::<String>()
    {
      return true;
    } else if let Some((y, x)) = cache.entry(index).or_default().pop() {
      indexes.push((y, x));
      let next = cache.entry(index + 1).or_default();

      macro_rules! check {
        ($y: expr, $x: expr, $condition: expr) => {
          if $condition
            && chars.get(index + 1) == Some(&board[$y][$x])
            && !indexes.contains(&($y, $x))
          {
            next.push(($y, $x));
          }
        };
      }
      check!(y - 1, x, y > 0);
      check!(y, x - 1, x > 0);
      check!(y + 1, x, y + 1 < row);
      check!(y, x + 1, x + 1 < column);

      if !next.is_empty() {
        index += 1; // update index only if there are valid chars in next position
      } else if word.len() > 1 {
        indexes.pop();
      }
    } else if index > 0 {
      indexes.pop();
      index -= 1;
    } else {
      return false; // index is 0, means no other substring, match failed
    }
  }
}

#[test]
fn q79_test() {
  fn test(exist: impl Fn(Vec<Vec<char>>, String) -> bool) {
    assert_eq!(exist(vec![vec!['A'], vec!['A']], "A".into()), true);
    assert_eq!(exist(vec![vec!['A'], vec!['A']], "C".into()), false);
    assert_eq!(exist(vec![vec!['A']], "B".into()), false);
    assert_eq!(
      exist(
        vec![
          vec!['A', 'B', 'C', 'E'],
          vec!['S', 'F', 'E', 'S'],
          vec!['A', 'D', 'E', 'E']
        ],
        "ABCEFSADEESE".into()
      ),
      true
    );
    assert_eq!(
      exist(
        vec![
          vec!['A', 'B', 'C', 'E'],
          vec!['S', 'F', 'C', 'S'],
          vec!['A', 'D', 'E', 'E']
        ],
        "ABCCED".into()
      ),
      true
    );
    assert_eq!(
      exist(
        vec![
          vec!['A', 'B', 'C', 'E'],
          vec!['S', 'F', 'C', 'S'],
          vec!['A', 'D', 'E', 'E']
        ],
        "EECCBA".into()
      ),
      true
    );
    assert_eq!(
      exist(
        vec![
          vec!['A', 'B', 'C', 'E'],
          vec!['S', 'F', 'C', 'S'],
          vec!['A', 'D', 'E', 'E']
        ],
        "SEE".into()
      ),
      true
    );
    assert_eq!(
      exist(
        vec![
          vec!['A', 'B', 'C', 'E'],
          vec!['S', 'F', 'C', 'S'],
          vec!['A', 'D', 'E', 'E']
        ],
        "ABCB".into()
      ),
      false
    );
    assert_eq!(exist(
      vec![
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'
        ],
        vec![
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a',
          'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'b'
        ]
      ],
      "baaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into()
    ), true);
  }

  test(exist);
  test(exist_loop);
}
