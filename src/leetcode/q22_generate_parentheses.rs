/**
 * Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
 *
 * For example, given n = 3, a solution set is:
 *
 * [
 *   "((()))",
 *   "(()())",
 *   "(())()",
 *   "()(())",
 *   "()()()"
 * ]
 */

fn generate_parenthesis(n: i32) -> Vec<String> {
  let mut temp = vec![(vec![], 0, 0)];

  for i in 0..2 * n {
    let mut next = vec![];
    for v in &temp {
      let (left, right) = (v.1, v.2);
      if left > right {
        if left < n || i < n {
          let mut val = v.clone();
          val.0.push('(');
          val.1 += 1;
          next.push(val);
        }
        let mut val = v.clone();
        val.0.push(')');
        val.2 += 1;
        next.push(val);
      } else {
        let mut val = v.clone();
        val.0.push('(');
        val.1 += 1;
        next.push(val);
      }
    }
    temp = next;
  }

  use std::iter::FromIterator;
  temp.into_iter().map(|v| String::from_iter(v.0)).collect()
}

#[test]
fn test_generate_parenthesis() {
  assert_eq!(generate_parenthesis(0), vec![""]);
  assert_eq!(generate_parenthesis(1), vec!["()"]);
  assert_eq!(generate_parenthesis(2), vec!["(())", "()()"]);
  assert_eq!(
    generate_parenthesis(3),
    vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
  );
  assert_eq!(
    generate_parenthesis(4),
    vec![
      "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
      "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()"
    ]
  );
  assert_eq!(
    generate_parenthesis(5),
    vec![
      "((((()))))",
      "(((()())))",
      "(((())()))",
      "(((()))())",
      "(((())))()",
      "((()(())))",
      "((()()()))",
      "((()())())",
      "((()()))()",
      "((())(()))",
      "((())()())",
      "((())())()",
      "((()))(())",
      "((()))()()",
      "(()((())))",
      "(()(()()))",
      "(()(())())",
      "(()(()))()",
      "(()()(()))",
      "(()()()())",
      "(()()())()",
      "(()())(())",
      "(()())()()",
      "(())((()))",
      "(())(()())",
      "(())(())()",
      "(())()(())",
      "(())()()()",
      "()(((())))",
      "()((()()))",
      "()((())())",
      "()((()))()",
      "()(()(()))",
      "()(()()())",
      "()(()())()",
      "()(())(())",
      "()(())()()",
      "()()((()))",
      "()()(()())",
      "()()(())()",
      "()()()(())",
      "()()()()()"
    ]
  );
}
