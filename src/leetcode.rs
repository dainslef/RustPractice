/**
 * The definition of ListNode, used by many problems
 */
#[derive(PartialEq, Eq, Debug)]
struct ListNode {
  pub val: i32,
  pub next: Option<Box<Self>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    Self { next: None, val }
  }
}

// convert a number to the list of every bit of the number
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

// build list node from the vector of the numbers
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

// build the vector of the numbers from the a list node
fn nodes_to_num_vec(node: Option<Box<ListNode>>) -> Vec<i32> {
  let (mut vec, mut temp) = (vec![], &node);
  while let Some(n) = temp {
    vec.push(n.val);
    temp = &n.next;
  }
  vec
}

// build the vector of the node from the a list node
fn nodes_to_node_vec(node: Option<Box<ListNode>>) -> Vec<Option<Box<ListNode>>> {
  let (mut vec, mut current) = (vec![], node);
  while let Some(v) = current.as_mut() {
    let node = std::mem::replace(&mut v.next, None);
    vec.push(current);
    current = node;
  }
  vec
}

use std::{cell::RefCell, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<Self>>>,
  pub right: Option<Rc<RefCell<Self>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None,
    }
  }

  #[inline]
  pub fn new_option(val: Option<i32>) -> Option<Rc<RefCell<Self>>> {
    val.map(|v| Rc::new(RefCell::new(Self::new(v))))
  }

  /**
   * Building binary tree from Vec<Option<i32>>, Some means valued node, None means empty node.
   * for example:
   *
   * [Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)] will be transformed to:
   *      1
   *    /   \
   *   2     3
   *  / \   /
   * 4   5 6
   *
   * [Some(1), Some(2), Some(3), Some(4), None, Some(5), None, Some(6)] will be transformed to:
   *        1
   *      /   \
   *     2     3
   *    / \   / \
   *   4     5
   *  /
   * 6
   */
  pub fn from(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<Self>>> {
    use std::collections::VecDeque;

    let mut root = None; // save the root node
    let mut nodes: VecDeque<*mut Option<Rc<RefCell<Self>>>> = Default::default(); // save the pointer to child nodes

    for v in vec {
      // use the macro to deal with child node
      macro_rules! deal {
        ($node: expr) => {
          if let Some(n) = &*$node {
            // add the pointer of child node, use raw pointer to avoid the ownership check
            // save the raw pointer of child node of new tree node dosn't need UNSAFE
            nodes.push_back(&mut n.borrow_mut().left);
            nodes.push_back(&mut n.borrow_mut().right);
          }
        };
      }
      let node = Self::new_option(v); // new tree node
      if root.is_none() {
        root = node;
        deal!(&root);
      } else if let Some(current) = nodes.pop_front() {
        unsafe {
          // only dereference raw pointer should under UNSAFE
          *current = node;
          deal!(current);
        }
      }
    }

    root
  }
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

// normal problems
mod q1008_construct_binary_search_tree_from_preorder_traversal;
mod q10_regular_expression_matching;
mod q11_container_with_most_water;
mod q126_word_ladder_ii;
mod q127_word_ladder;
mod q12_integer_to_roman;
mod q16_three_sum_closest;
mod q17_letter_combinations_of_a_phone_number;
mod q18_four_sum;
mod q19_remove_nth_node_from_end_of_list;
mod q200_number_of_islands;
mod q208_implement_trie;
mod q212_word_search_ii;
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
mod q525_contiguous_array;
mod q52_n_queens_ii;
mod q53_maximum_subarray;
mod q543_diameter_of_binary_tree;
mod q54_spiral_matrix;
mod q55_jump_game;
mod q56_merge_intervals;
mod q57_insert_interval;
mod q59_spiral_matrix_ii;
mod q5_longest_palindrome;
mod q60_permutation_sequence;
mod q61_rotate_list;
mod q62_unique_paths;
mod q63_unique_paths_ii;
mod q64_minimum_path_sum;
mod q65_valid_number;
mod q68_text_justification;
mod q6_zipzag_conversion;
mod q71_simplify_path;
mod q72_edit_distance;
mod q73_set_matrix_zeroes;
mod q74_search_a_2d_matrix;
mod q75_sort_colors;
mod q76_minimum_window_substring;
mod q77_combinations;
mod q78_subsets;
mod q79_word_search;
mod q7_reverse_integer;
mod q82_remove_duplicates_from_sorted_list_ii;
mod q844_backspace_string_compare;
mod q84_largest_rectangle_in_histogram;
mod q85_maximal_rectangle;
mod q8_my_atoi;
mod q92_reverse_linked_list_ii;
mod q97_interleaving_string;

// some extra problems can only be found in "30-Day LeetCoding Challenge"
mod day_30_leetcoding_challenge;

// mod q80_remove_duplicates_from_sorted_array_ii;
// mod q124_binary_tree_maximum_path_sum;
// mod q221_maximal_square;
// mod q1143_longest_common_subsequence;
// mod q146_lru_cache;
// mod q201_bitwise_and_of_numbers_range;
// mod q560_subarray_sum_equals_k;
// mod q678_valid_parenthesis_string;
// mod q238_product_of_array_except_self;
// mod q1046_last_stone_weight;
// mod q155_min_stack;
// mod q876_middle_of_the_linked_list;
// mod q122_best_time_to_buy_and_sell_stock_ii;
// mod q283_move_zeroes;
// mod q136_single_number;
// mod q202_happy_number;
// mod q328_odd_even_linked_list;
// mod q725_split_linked_list;
// mod q885_spiral_matrix_iii;
// mod q143_recoder_list;
// mod q216_combination_sum_iii;
// mod q377_combination_sum_iv;
// mod q81_search_in_rotated_sorted_array_ii;
