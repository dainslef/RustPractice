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
 *
 * Tips: Don't try to split the input string, just save the index.
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
    let mut next = vec![];
    if temp.is_empty() {
      // init content
      next.push(vec![0]);
      if chars[0] != '0' {
        next.push(vec![1]);
        if check(0..=2) {
          next.push(vec![2]);
        }
      }
    }
    for v in temp {
      let last = v.last().unwrap();
      let mut add = |offset: usize| {
        let current = last + offset;
        if current < s.len()
              && (offset < 2 || chars.get(last + 1) != Some(&'0')) // each integer of ip can't start with 0
              && (offset < 3 || check(last + 1..=last + 3)) // each integer of ip should between 0 ~ 255
              && (v.len() < 3 || v.len() == 3 && s.len() - current == 1)
        // last index should match the size of input string
        {
          let mut v = v.clone();
          v.push(current);
          next.push(v);
        }
      };
      add(1);
      add(2);
      add(3); // add the next ip integer with different offset
    }
    temp = next;
  }

  temp
    .into_iter()
    .map(|v| {
      let get = |i| s.get(i).unwrap_or_default();
      get(0..=v[0]).to_string()
        + "."
        + get(v[0] + 1..=v[1])
        + "."
        + get(v[1] + 1..=v[2])
        + "."
        + get(v[2] + 1..=v[3])
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
