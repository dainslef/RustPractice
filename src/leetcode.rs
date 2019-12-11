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

fn num_to_nodes(mut num: i32, reverse: bool) -> Option<Box<ListNode>> {
  let mut vec = vec![];
  while num / 10 > 0 {
    vec.push(num % 10);
    num /= 10;
  }
  vec.push(num % 10);
  // the sequence of the vec is opposite of the number
  num_vec_to_nodes(vec, !reverse)
}

fn num_vec_to_nodes(mut vec: Vec<i32>, reverse: bool) -> Option<Box<ListNode>> {
  let mut next = None;
  if !reverse {
    vec.reverse();
  }
  for val in vec {
    next = Some(Box::new(ListNode { val, next }));
  }
  next
}

// build the vector of the number from the number list
fn nodes_to_num_vec(node: Option<Box<ListNode>>) -> Vec<i32> {
  let (mut vec, mut temp) = (vec![], &node);
  while let Some(n) = temp {
    vec.push(n.val);
    temp = &n.next;
  }
  vec
}

// build the vector of the node from the node list
fn nodes_to_node_vec(node: Option<Box<ListNode>>) -> Vec<Option<Box<ListNode>>> {
  let (mut vec, mut current) = (vec![], node);
  while let Some(v) = current.as_mut() {
    let node = std::mem::replace(&mut v.next, None);
    vec.push(current);
    current = node;
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

// check element content equivalence without element order
fn check_element_eq<T>(v1: T, v2: T) -> bool
where
  T: IntoIterator,
  T::Item: Eq + std::hash::Hash + std::fmt::Debug,
{
  use std::collections::HashMap;

  let (mut length1, mut length2) = (0, 0);
  let (mut content1, mut content2) = (HashMap::new(), HashMap::new());

  for v in v1 {
    length1 += 1;
    *content1.entry(v).or_insert(0) += 1;
  }
  for v in v2 {
    length2 += 1;
    *content2.entry(v).or_insert(0) += 1;
  }

  let eq = content1 == content2 && length1 == length2;
  if !eq {
    println!(
      "Elements are different!\nLength 1: {}, Length 2: {}",
      length1, length2
    );
    println!("Content 1: {:?}\nContent 2: {:?}", content1, content2);
  }

  eq
}

/**
 * Unlike everything else in the languages, macros will remain visible in sub-modules.
 * Also, unlike everything else in the language, macros are only accessible after their definition.
 * Or use #[macro_export] to export the macro, then use macro with code "crate::xxx_macro_name!".
 */
macro_rules! string_vec {
  ($($content:expr),*) => {{
    let mut temp = Vec::new();
    $(temp.push($content.to_string());)*
    temp
  }}
}

mod q10_regular_expression_matching;
mod q11_container_with_most_water;
mod q126_word_ladder_ii;
mod q127_word_ladder;
mod q12_integer_to_roman;
mod q16_three_sum_closest;
mod q17_letter_combinations_of_a_phone_number;
mod q18_four_sum;
mod q19_remove_nth_node_from_end_of_list;
mod q22_generate_parentheses;
mod q23_merge_k_sorted_lists;
mod q24_swap_nodes_in_pairs;
mod q25_reverse_nodes_in_k_group;
mod q29_divide_two_integers;
mod q2_add_two_numbers;
mod q30_substring_with_concatenation_of_all_words;
mod q31_next_permutation;
mod q32_longest_valid_parentheses;
mod q33_search_in_rotated_sorted_array;
mod q34_find_first_and_last_position_of_element_in_sorted_array;
mod q35_valid_sudoku;
mod q37_sudoku_solver;
mod q39_combination_sum;
mod q407_trapping_rain_water_ii;
mod q40_combination_sum_ii;
mod q41_first_missing_positive;
mod q42_trapping_rain_water;
mod q43_multiply_strings;
mod q44_wildcard_matching;
mod q454_four_sum_ii;
mod q45_jump_game_ii;
mod q46_permutations;
mod q47_permutations_ii;
mod q48_rotate_image;
mod q49_group_anagrams;
mod q4_find_median_sorted_arrays;
mod q50_pow_x_n;
mod q51_n_queens;
mod q59_spiral_matrix_ii;
mod q5_longest_palindrome;
mod q6_zipzag_conversion;
mod q76_minimum_window_substring;
mod q7_reverse_integer;
mod q8_my_atoi;
mod q92_reverse_linked_list_ii;
mod q97_interleaving_string;

// mod q54_spiral_matrix;
// mod q55_jump_game;
// mod q143_recoder_list;
// mod q216_combination_sum_iii;
// mod q377_combination_sum_iv;
// mod q52_n_queens_ii;
// mod q60_permutation_sequence;
// mod q77_combinations;
// mod q81_search_in_rotated_sorted_array_ii;
