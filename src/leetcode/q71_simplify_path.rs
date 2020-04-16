/**
 * 71. Simplify Path
 * https://leetcode.com/problems/simplify-path/
 *
 * Given an absolute path for a file (Unix-style), simplify it. Or in other words, convert it to the canonical path.
 *
 * In a UNIX-style file system, a period . refers to the current directory. Furthermore, a double period .. moves the directory up a level.
 *
 * Note that the returned canonical path must always begin with a slash /, and there must be only a single slash / between two directory names. The last directory name (if it exists) must not end with a trailing /. Also, the canonical path must be the shortest string representing the absolute path.
 *
 *
 *
 * Example 1:
 *
 * Input: "/home/"
 * Output: "/home"
 * Explanation: Note that there is no trailing slash after the last directory name.
 * Example 2:
 *
 * Input: "/../"
 * Output: "/"
 * Explanation: Going one level up from the root directory is a no-op, as the root level is the highest level you can go.
 * Example 3:
 *
 * Input: "/home//foo/"
 * Output: "/home/foo"
 * Explanation: In the canonical path, multiple consecutive slashes are replaced by a single one.
 * Example 4:
 *
 * Input: "/a/./b/../../c/"
 * Output: "/c"
 * Example 5:
 *
 * Input: "/a/../../b/../c//.//"
 * Output: "/c"
 * Example 6:
 *
 * Input: "/a//b////c/d//././/.."
 * Output: "/a/b/c"
 */

fn simplify_path(path: String) -> String {
  macro_rules! update {
    ($paths: expr, $dir: expr) => {
      use std::iter::FromIterator;
      match $dir.as_slice() {
        ['.', '.'] => drop($paths.pop()),
        ['.'] | [] => {}
        _ => $paths.push(String::from_iter($dir)),
      };
    };
  }

  let (mut paths, mut dir): (Vec<String>, Vec<char>) = (vec![], vec![]);
  for c in path.chars() {
    if c == '/' {
      update!(paths, dir);
      dir = vec![];
    } else {
      dir.push(c);
    }
  }

  update!(paths, dir);
  if paths.is_empty() {
    "/".into()
  } else {
    paths.iter().map(|v| "/".to_string() + v).collect()
  }
}

#[test]
fn q71_test() {
  assert_eq!(simplify_path("/home///".into()), "/home");
  assert_eq!(simplify_path("/../".into()), "/");
  assert_eq!(simplify_path("/a/./b/../../c/".into()), "/c");
  assert_eq!(simplify_path("/a/./b/../../c".into()), "/c");
  assert_eq!(simplify_path("/a/../../b/../c//.//".into()), "/c");
  assert_eq!(simplify_path("/a//b////c/d//././/..".into()), "/a/b/c");
}
