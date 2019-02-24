/**
 * Given s1, s2, s3, find whether s3 is formed by the interleaving of s1 and s2.
 *
 * Example 1:
 *
 * Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
 * Output: true
 * Example 2:
 *
 * Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
 * Output: false
 */

fn is_interleave(s1: String, s2: String, s3: String) -> bool {
  let (len1, len2, len3) = (s1.len(), s2.len(), s3.len());

  if len1 + len2 == len3 {
    let (u1s, u2s, u3s): (&[u8], &[u8], &[u8]) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
    let line = (0..=len2).map(|_| false).collect::<Vec<bool>>();
    let mut temp = (0..=len1).map(|_| line.clone()).collect::<Vec<Vec<bool>>>();

    for i1 in 0..=len1 {
      for i2 in 0..=len2 {
        temp[i1][i2] = match (i1, i2) {
          (0, 0) => true,
          (0, _) => temp[0][i2 - 1] && u2s[i2 - 1] == u3s[i1 + i2 - 1],
          (_, 0) => temp[i1 - 1][0] && u1s[i1 - 1] == u3s[i1 + i2 - 1],
          _ => {
            (temp[i1 - 1][i2] && u1s[i1 - 1] == u3s[i1 + i2 - 1])
              || (temp[i1][i2 - 1] && u2s[i2 - 1] == u3s[i1 + i2 - 1])
          }
        }
      }
    }

    temp[len1][len2]
  } else {
    false
  }
}

#[test]
fn test_is_interleave() {
  assert_eq!(
    is_interleave(
      "aabcc".to_string(),
      "dbbca".to_string(),
      "aadbbcbcac".to_string()
    ),
    true
  );
  assert_eq!(
    is_interleave(
      "aaan".to_string(),
      "naaa".to_string(),
      "aaanaaa".to_string()
    ),
    false
  );
  assert_eq!(
    is_interleave(
      "aaan".to_string(),
      "naaa".to_string(),
      "aaannaaa".to_string()
    ),
    true
  );
  assert_eq!(
    is_interleave(
      "aabcc".to_string(),
      "dbsca".to_string(),
      "aadbbcbcac".to_string()
    ),
    false
  );
  assert_eq!(
    is_interleave(
      "ssss".to_string(),
      "ssss".to_string(),
      "ssssssss".to_string()
    ),
    true
  );
  assert_eq!(
    is_interleave("".to_string(), "".to_string(), "".to_string()),
    true
  );
  assert_eq!(
    is_interleave("a".to_string(), "".to_string(), "c".to_string()),
    false
  );
  assert_eq!(
    is_interleave(
      "aabc".to_string(),
      "abad".to_string(),
      "aabcabad".to_string()
    ),
    true
  );
  assert_eq!(
    is_interleave(
      "aabc".to_string(),
      "abad".to_string(),
      "aabacbad".to_string()
    ),
    true
  );
  assert_eq!(
    is_interleave("".to_string(), "aaaa".to_string(), "aaaa".to_string()),
    true
  );
}
