/**
 * 208. Implement Trie (Prefix Tree)
 * https://leetcode.com/problems/implement-trie-prefix-tree/
 *
 * Implement a trie with insert, search, and startsWith methods.
 *
 * Example:
 *
 * Trie trie = new Trie();
 *
 * trie.insert("apple");
 * trie.search("apple");   // returns true
 * trie.search("app");     // returns false
 * trie.startsWith("app"); // returns true
 * trie.insert("app");
 * trie.search("app");     // returns true
 * Note:
 *
 * You may assume that all inputs are consist of lowercase letters a-z.
 * All inputs are guaranteed to be non-empty strings.
 */

struct Trie {
  is_end: bool,             // mark if this node can be used as the end of a word
  chars: Vec<Option<Self>>, // save the info of next chars, use Vec index to represent the ascii of char
}

// check the word with custom operate
macro_rules! check_word {
  ($self: ident, $word: expr, $operate: expr) => {{
    let mut current: &Trie = &$self;
    for b in $word.bytes() {
      let index = b as usize - Trie::OFFSET;
      if let Some(c) = &current.chars[index] {
        current = c; // check if the current char has been record
      } else {
        return false;
      }
    }
    $operate(current)
  }};
}

/**
 * Runtime: 32 ms, faster than 32.56% of Rust online submissions for Implement Trie (Prefix Tree).
 * Memory Usage: 32.1 MB, less than 6.98% of Rust online submissions for Implement Trie (Prefix Tree).
 */
impl Trie {
  const OFFSET: usize = 97;

  /** Initialize your data structure here. */
  fn new() -> Self {
    Trie {
      is_end: false,
      chars: (0..26).map(|_| None).collect(),
    }
  }

  /** Inserts a word into the trie. */
  fn insert(&mut self, word: String) {
    let mut current: &mut Trie = self;
    for b in word.bytes() {
      let index = b as usize - Trie::OFFSET;
      current = current.chars[index].get_or_insert(Self::new());
    }
    current.is_end = true; // mark this node as be used as whole word
  }

  /** Returns if the word is in the trie. */
  fn search(&self, word: String) -> bool {
    check_word!(self, word, |v: &Trie| v.is_end)
  }

  /** Returns if there is any word in the trie that starts with the given prefix. */
  fn starts_with(&self, prefix: String) -> bool {
    check_word!(self, prefix, |_| true)
  }
}

#[test]
fn q208_test() {
  let mut trie = Trie::new();

  trie.insert("apple".into());
  assert!(trie.search("apple".into())); // returns true
  assert!(!trie.search("app".into())); // returns false
  assert!(!trie.search("appc".into())); // returns false
  assert!(trie.starts_with("app".into())); // returns true

  trie.insert("app".into());
  assert!(trie.search("app".into())); // returns true
}
