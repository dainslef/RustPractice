/**
 * Use "#[cfg(test)]" to define a test module,
 * the function in test module will only compile and run when user execute "cargo test" command.
 * Functions used by tests but not used by other modules will not receive
 * warnning message like "warning: function is never used: `...`".
 */
mod q11_container_with_most_water;
mod q126_word_ladder_two;
mod q127_word_ladder;
mod q12_integer_to_roman;
mod q15_three_sum;
mod q16_three_sum_closest;
mod q17_letter_combinations_of_a_phone_number;
mod q18_four_sum;
mod q19_remove_nth_node_from_end_of_list;
// mod q22_generate_parentheses;
// mod q24_swap_nodes_in_pairs;
// mod q23_merge_k_sorted_lists;
// mod q25_reverse_nodes_in_k_group;
mod q2_add_two_numbers;
// mod q29_divide_two_integers;
mod q3_length_of_longest_substring;
// mod q42_trapping_rain_water;
mod q445_add_two_numbers_two;
mod q454_four_sum_two;
mod q4_find_median_sorted_arrays;
mod q5_longest_palindrome;
mod q6_zipzag_conversion;
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

fn num_to_nodes(mut num: i32, reserve: bool) -> Option<Box<ListNode>> {
  let mut vec = vec![];
  while num / 10 > 0 {
    vec.push(num % 10);
    num /= 10;
  }
  vec.push(num % 10);
  // the sequence of the vec is opposite of the number
  vec_to_nodes(vec, !reserve)
}

fn vec_to_nodes(mut vec: Vec<i32>, reserve: bool) -> Option<Box<ListNode>> {
  let mut next = None;
  if !reserve {
    vec.reverse();
  }
  for val in vec {
    next = Some(Box::new(ListNode { val, next }));
  }
  next
}

// build the vector of the number from the number list
fn nodes_to_vec(node: Option<Box<ListNode>>) -> Vec<i32> {
  let (mut vec, mut temp) = (vec![], &node);
  while let Some(n) = temp {
    vec.push(n.val);
    temp = &n.next;
  }
  vec
}

// for q15 and q18, check if the target is included in the "vec_list"
fn check_vecs_contain_target(vec_list: &Vec<Vec<i32>>, target: &Vec<i32>) -> bool {
  for old_vec in vec_list {
    let mut new_vec = target.clone();
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
      return true;
    }
  }
  false
}

// for q126 and q127, check if two words differ by only one character
fn check_diff_one_char(old_word: &String, new_word: &String) -> bool {
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

fn check_vec_val_eq<T: Eq + std::hash::Hash + std::fmt::Debug>(
  vec1: &Vec<T>,
  vec2: &Vec<T>,
) -> bool {
  use std::collections::HashSet;
  use std::iter::FromIterator;

  let set1: HashSet<&T> = HashSet::from_iter(vec1);
  let set2 = HashSet::from_iter(vec2);

  let eq = set1 == set2;
  if !eq {
    dbg!((&set1, &set2));
  }

  eq
}

fn strs_to_vec(str_array: &[&str]) -> Vec<String> {
  str_array.iter().map(|v| v.to_string()).collect()
}
