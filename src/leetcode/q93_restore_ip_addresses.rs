/**
 * 93. Restore IP Addresses
 * https://leetcode.com/problems/restore-ip-addresses/
 *
 * Given a string s containing only digits, return all possible valid IP addresses that can be obtained from s. You can return them in any order.
 *
 * A valid IP address consists of exactly four integers, each integer is between 0 and 255, separated by single dots and cannot have leading zeros. For example, "0.1.2.201" and "192.168.1.1" are valid IP addresses and "0.011.255.245", "192.168.1.312" and "192.168@1.1" are invalid IP addresses.
 *
 *
 *
 * Example 1:
 *
 * Input: s = "25525511135"
 * Output: ["255.255.11.135","255.255.111.35"]
 * Example 2:
 *
 * Input: s = "0000"
 * Output: ["0.0.0.0"]
 * Example 3:
 *
 * Input: s = "1111"
 * Output: ["1.1.1.1"]
 * Example 4:
 *
 * Input: s = "010010"
 * Output: ["0.10.0.10","0.100.1.0"]
 * Example 5:
 *
 * Input: s = "101023"
 * Output: ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
 *
 *
 * Constraints:
 *
 * 0 <= s.length <= 3000
 * s consists of digits only.
 */

/**
 * Runtime: 0 ms, faster than 100.00% of Rust online submissions for Restore IP Addresses.
 * Memory Usage: 2.1 MB, less than 80.00% of Rust online submissions for Restore IP Addresses.
 */
fn restore_ip_addresses(s: String) -> Vec<String> {
  if s.len() < 4 || s.len() > 12 {
    return vec![];
  }

  let mut temp: Vec<Vec<usize>> = vec![];
  let chars: Vec<char> = s.chars().collect();

  let check = |r| {
    s.get(r)
      .map(|v: &str| {
        let n = v.parse::<i32>().unwrap_or(-1);
        n >= 0 && n <= 255
      })
      .unwrap_or(false)
  };

  for _ in 0..4 {
    if temp.is_empty() {
      temp.push(vec![0]);
      if chars[0] == '0' {
        continue;
      }
      temp.push(vec![1]);
      if check(0..=2) {
        temp.push(vec![2]);
      }
    } else {
      let mut next = vec![];
      for v in temp {
        if v.len() <= 3 {
          if let Some(last) = v.last() {
            if last + 1 < s.len() && (v.len() == 3 && s.len() - last == 2 || v.len() < 3) {
              let mut v = v.clone();
              v.push(last + 1);
              next.push(v);
            }
            if chars.get(last + 1) == Some(&'0') {
              continue;
            }
            if last + 2 < s.len() && (v.len() == 3 && s.len() - last == 3 || v.len() < 3) {
              let mut v = v.clone();
              v.push(last + 2);
              next.push(v);
            }
            if last + 3 < s.len()
              && check(last + 1..=last + 3)
              && (v.len() == 3 && s.len() - last == 4 || v.len() < 3)
            {
              let mut v = v.clone();
              v.push(last + 3);
              next.push(v);
            }
          }
        }
      }
      temp = next;
    }
  }

  temp
    .into_iter()
    .filter(|v| v.len() == 4)
    .map(|v| {
      let (i1, i2, i3, i4) = (v[0], v[1], v[2], v[3]);
      let get = |i| s.get(i).unwrap_or_default();
      let (s1, s2, s3, s4) = (
        get(0..=i1),
        get(i1 + 1..=i2),
        get(i2 + 1..=i3),
        get(i3 + 1..=i4),
      );
      "".to_string() + s1 + "." + s2 + "." + s3 + "." + s4
    })
    .collect()
}

#[test]
fn q93_test() {
  assert!(restore_ip_addresses("".into()).is_empty());
  assert!(restore_ip_addresses("1".into()).is_empty());
  assert_eq!(
    restore_ip_addresses("25525511135".into()),
    ["255.255.11.135", "255.255.111.35"]
  );
  assert_eq!(restore_ip_addresses("0000".into()), ["0.0.0.0"]);
  assert_eq!(restore_ip_addresses("1111".into()), ["1.1.1.1"]);
  assert_eq!(
    restore_ip_addresses("010010".into()),
    ["0.10.0.10", "0.100.1.0"]
  );
  assert_eq!(
    restore_ip_addresses("101023".into()),
    [
      "1.0.10.23",
      "1.0.102.3",
      "10.1.0.23",
      "10.10.2.3",
      "101.0.2.3"
    ]
  );
}
