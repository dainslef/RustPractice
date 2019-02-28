/**
 * Use "#[cfg(test)]" to define a test module,
 * the function in test module will only compile and run when user execute "cargo test" command.
 * Functions used by tests but not used by other modules will not receive
 * warnning message like "warning: function is never used: `...`".
 */
mod q2_add_two_numbers;
mod q3_length_of_longest_substring;
mod q4_find_median_sorted_arrays;
mod q5_longest_palindrome;
mod q6_zipzag_conversion;
mod q126_word_ladder_two;
mod q127_word_ladder;
// mod q11_container_with_most_water;
mod q15_three_sum;
mod q16_three_sum_closest;
mod q18_four_sum;
mod q445_add_two_numbers_two;
mod q454_four_sum_two;
mod q7_reverse_integer;
mod q8_my_atoi;
mod q97_interleaving_string;

#[derive(PartialEq, Eq, Debug)]
struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

fn build_nodes(mut num: i32, reserve: bool) -> Option<Box<ListNode>> {
  let (mut next, mut vec) = (None, vec![]);
  while num / 10 > 0 {
    vec.push(num % 10);
    num /= 10;
  }
  vec.push(num % 10);
  if reserve {
    vec.reverse();
  }
  for val in vec {
    next = Some(Box::new(ListNode { val, next }));
  }
  next
}

// for q15 and q18, check if the target "vec" is equal to some element in "vec_list"
fn check_duplicate(vec_list: &Vec<Vec<i32>>, vec: &Vec<i32>) -> bool {
  let mut is_duplicate = false;
  for old_vec in vec_list {
    let mut new_vec = vec.clone();
    for old_val in old_vec {
      for i in 0..new_vec.len() {
        // check target vec if have equal element in old_vec
        if old_val == &new_vec[i] {
          new_vec.remove(i);
          break;
        }
      }
    }
    // if all elemnets have been removed, mean the vec is duplicate
    if new_vec.is_empty() {
      is_duplicate = true;
      break;
    }
  }
  is_duplicate
}

// for q126 and q127, check if two words differ by only one character
fn check_word(old_word: &String, new_word: &String) -> bool {
  let mut count = 0;
  let (old_u8s, new_u8s): (&[u8], &[u8]) = (old_word.as_ref(), new_word.as_ref());
  for i in 0..old_u8s.len() {
    if old_u8s[i] != new_u8s[i] {
      count += 1;
      if count > 1 {
        return false;
      }
    }
  }
  count == 1
}
