/* *
 * Given two words (beginWord and endWord), and a dictionary's word list, find all shortest transformation sequence(s) from beginWord to endWord, such that:
 *
 * Only one letter can be changed at a time
 * Each transformed word must exist in the word list. Note that beginWord is not a transformed word.
 * Note:
 *
 * Return an empty list if there is no such transformation sequence.
 * All words have the same length.
 * All words contain only lowercase alphabetic characters.
 * You may assume no duplicates in the word list.
 * You may assume beginWord and endWord are non-empty and are not the same.
 * Example 1:
 *
 * Input:
 * beginWord = "hit",
 * endWord = "cog",
 * wordList = ["hot","dot","dog","lot","log","cog"]
 *
 * Output:
 * [
 *   ["hit","hot","dot","dog","cog"],
 *   ["hit","hot","lot","log","cog"]
 * ]
 * Example 2:
 *
 * Input:
 * beginWord = "hit"
 * endWord = "cog"
 * wordList = ["hot","dot","dog","lot","log"]
 *
 * Output: []
 *
 * Explanation: The endWord "cog" is not in wordList, therefore no possible transformation.
 */
use super::*;

fn find_ladders_dfs(
  begin_word: String,
  end_word: String,
  word_list: Vec<String>,
) -> Vec<Vec<String>> {
  let mut temp: Vec<Vec<String>> = vec![];
  let mut min_size = 0;

  // use recursion to find out all path
  fn dfs(
    current: &String,
    input_list: Vec<String>,
    mut output_list: Vec<String>,
    end_word: &String,
    temp: &mut Vec<Vec<String>>,
    min_size: &mut usize,
  ) {
    // check if find out the target path
    if check_diff_one_char(&current, end_word) {
      output_list.push(end_word.clone());
      let size = output_list.len();
      // check and update the path size
      if *min_size == 0 || size < *min_size {
        *min_size = size;
      }
      // add the path to the temp
      temp.push(output_list);
      // find the target, finish this recursion
      return;
    }
    for i in 0..input_list.len() {
      // get the next word
      let next = &input_list[i];
      // check the next word if match the condition
      if check_diff_one_char(&current, &next) {
        // copy and update the input/ouput list
        let (mut new_output_list, mut new_input_list) = (output_list.clone(), input_list.clone());
        new_output_list.push(next.clone());
        new_input_list.remove(i);
        // recursively check the next word until find all the paths
        dfs(
          next,
          new_input_list,
          new_output_list,
          end_word,
          temp,
          min_size,
        );
      }
    }
  }

  if word_list.contains(&end_word) {
    dfs(
      &begin_word,
      word_list,
      vec![begin_word.clone()],
      &end_word,
      &mut temp,
      &mut min_size,
    );
  }

  // filter results, only return the list which have least size
  temp
    .into_iter()
    .filter(|v| v.len() == min_size && v.last() == Some(&end_word))
    .collect()
}

fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
  use std::collections::{HashSet, VecDeque};
  use std::iter::FromIterator;

  // save all of the target paths which match the condition
  let mut result: Vec<Vec<String>> = vec![];
  // save the words which last level has used
  let mut currents: HashSet<String> = HashSet::new();
  // use hash set to save the all input word list, check if the word in the list under the time complexity of O(1)
  let mut word_set: HashSet<String> = HashSet::from_iter(word_list);

  // save the temp paths
  let mut paths = VecDeque::new();
  paths.push_back(vec![begin_word]);

  if word_set.contains(&end_word) {
    let (mut level, mut min_level) = (1, word_set.len());
    while let Some(path) = paths.pop_front() {
      // if the length of temp list bigger than the value of level, it means reaching a deeper level
      if path.len() > level {
        for word in &currents {
          // clean up the words which have been checked, the target path won't have the duplicate words
          word_set.remove(word);
        }
        // clean up the old words and update the value of level
        currents.clear();
        level = path.len();
        if level > min_level {
          // break out if current level is deeper than the min level
          break;
        }
      }
      if let Some(current) = path.last() {
        let bytes = current.clone().into_bytes();
        for i in 0..bytes.len() {
          // 'a' .. 'z' ASCII number
          for u in 97_u8..=122_u8 {
            // change a character in the word and check if the new word is contained by the word set
            let mut bytes = bytes.clone();
            bytes[i] = u;
            if let Ok(next) = String::from_utf8(bytes) {
              if word_set.contains(&next) {
                currents.insert(next.clone());
                let mut new_path = path.clone();
                new_path.push(next.clone());
                if next == end_word {
                  min_level = new_path.len();
                  result.push(new_path);
                } else {
                  paths.push_back(new_path);
                }
              }
            }
          }
        }
      }
    }
  }

  result
}

#[test]
fn test_find_ladders() {
  let empty: Vec<Vec<String>> = vec![];

  assert_eq!(
    find_ladders(
      "hot".to_string(),
      "dog".to_string(),
      strs_to_vec(&["hot", "dog", "dot"])
    ),
    vec![strs_to_vec(&["hot", "dot", "dog"])]
  );

  assert_eq!(
    find_ladders(
      "hot".to_string(),
      "dog".to_string(),
      strs_to_vec(&["hot", "dog"])
    ),
    empty
  );

  assert_eq!(
    find_ladders(
      "a".to_string(),
      "c".to_string(),
      strs_to_vec(&["a", "b", "c"])
    ),
    vec![strs_to_vec(&["a", "c"])]
  );

  assert_eq!(
    find_ladders(
      "hit".to_string(),
      "cog".to_string(),
      strs_to_vec(&["hot", "dot", "dog", "lot", "log"])
    ),
    empty
  );

  assert!(check_element_eq(
    find_ladders(
      "hit".to_string(),
      "cog".to_string(),
      strs_to_vec(&["hot", "dot", "dog", "lot", "log", "cog"])
    )
    .iter(),
    vec![
      strs_to_vec(&["hit", "hot", "lot", "log", "cog"]),
      strs_to_vec(&["hit", "hot", "dot", "dog", "cog"])
    ]
    .iter()
  ));

  assert!(check_element_eq(
    find_ladders(
      "qa".to_string(),
      "sq".to_string(),
      strs_to_vec(&[
        "si", "go", "se", "cm", "so", "ph", "mt", "db", "mb", "sb", "kr", "ln", "tm", "le", "av",
        "sm", "ar", "ci", "ca", "br", "ti", "ba", "to", "ra", "fa", "yo", "ow", "sn", "ya", "cr",
        "po", "fe", "ho", "ma", "re", "or", "rn", "au", "ur", "rh", "sr", "tc", "lt", "lo", "as",
        "fr", "nb", "yb", "if", "pb", "ge", "th", "pm", "rb", "sh", "co", "ga", "li", "ha", "hz",
        "no", "bi", "di", "hi", "qa", "pi", "os", "uh", "wm", "an", "me", "mo", "na", "la", "st",
        "er", "sc", "ne", "mn", "mi", "am", "ex", "pt", "io", "be", "fm", "ta", "tb", "ni", "mr",
        "pa", "he", "lr", "sq", "ye"
      ])
    )
    .iter(),
    vec![
      strs_to_vec(&["qa", "ba", "be", "se", "sq"]),
      strs_to_vec(&["qa", "ba", "bi", "si", "sq"]),
      strs_to_vec(&["qa", "ba", "br", "sr", "sq"]),
      strs_to_vec(&["qa", "ca", "ci", "si", "sq"]),
      strs_to_vec(&["qa", "ca", "cm", "sm", "sq"]),
      strs_to_vec(&["qa", "ca", "co", "so", "sq"]),
      strs_to_vec(&["qa", "ca", "cr", "sr", "sq"]),
      strs_to_vec(&["qa", "fa", "fe", "se", "sq"]),
      strs_to_vec(&["qa", "fa", "fm", "sm", "sq"]),
      strs_to_vec(&["qa", "fa", "fr", "sr", "sq"]),
      strs_to_vec(&["qa", "ga", "ge", "se", "sq"]),
      strs_to_vec(&["qa", "ga", "go", "so", "sq"]),
      strs_to_vec(&["qa", "ha", "he", "se", "sq"]),
      strs_to_vec(&["qa", "ha", "hi", "si", "sq"]),
      strs_to_vec(&["qa", "ha", "ho", "so", "sq"]),
      strs_to_vec(&["qa", "la", "le", "se", "sq"]),
      strs_to_vec(&["qa", "la", "li", "si", "sq"]),
      strs_to_vec(&["qa", "la", "ln", "sn", "sq"]),
      strs_to_vec(&["qa", "la", "lo", "so", "sq"]),
      strs_to_vec(&["qa", "la", "lr", "sr", "sq"]),
      strs_to_vec(&["qa", "la", "lt", "st", "sq"]),
      strs_to_vec(&["qa", "ma", "mb", "sb", "sq"]),
      strs_to_vec(&["qa", "ma", "me", "se", "sq"]),
      strs_to_vec(&["qa", "ma", "mi", "si", "sq"]),
      strs_to_vec(&["qa", "ma", "mn", "sn", "sq"]),
      strs_to_vec(&["qa", "ma", "mo", "so", "sq"]),
      strs_to_vec(&["qa", "ma", "mr", "sr", "sq"]),
      strs_to_vec(&["qa", "ma", "mt", "st", "sq"]),
      strs_to_vec(&["qa", "na", "nb", "sb", "sq"]),
      strs_to_vec(&["qa", "na", "ne", "se", "sq"]),
      strs_to_vec(&["qa", "na", "ni", "si", "sq"]),
      strs_to_vec(&["qa", "na", "no", "so", "sq"]),
      strs_to_vec(&["qa", "pa", "pb", "sb", "sq"]),
      strs_to_vec(&["qa", "pa", "ph", "sh", "sq"]),
      strs_to_vec(&["qa", "pa", "pi", "si", "sq"]),
      strs_to_vec(&["qa", "pa", "pm", "sm", "sq"]),
      strs_to_vec(&["qa", "pa", "po", "so", "sq"]),
      strs_to_vec(&["qa", "pa", "pt", "st", "sq"]),
      strs_to_vec(&["qa", "ra", "rb", "sb", "sq"]),
      strs_to_vec(&["qa", "ra", "re", "se", "sq"]),
      strs_to_vec(&["qa", "ra", "rh", "sh", "sq"]),
      strs_to_vec(&["qa", "ra", "rn", "sn", "sq"]),
      strs_to_vec(&["qa", "ta", "tb", "sb", "sq"]),
      strs_to_vec(&["qa", "ta", "tc", "sc", "sq"]),
      strs_to_vec(&["qa", "ta", "th", "sh", "sq"]),
      strs_to_vec(&["qa", "ta", "ti", "si", "sq"]),
      strs_to_vec(&["qa", "ta", "tm", "sm", "sq"]),
      strs_to_vec(&["qa", "ta", "to", "so", "sq"]),
      strs_to_vec(&["qa", "ya", "yb", "sb", "sq"]),
      strs_to_vec(&["qa", "ya", "ye", "se", "sq"]),
      strs_to_vec(&["qa", "ya", "yo", "so", "sq"]),
    ]
    .iter()
  ));

  assert!(check_element_eq(
    find_ladders(
      "cet".to_string(),
      "ism".to_string(),
      strs_to_vec(&[
        "kid", "tag", "pup", "ail", "tun", "woo", "erg", "luz", "brr", "gay", "sip", "kay", "per",
        "val", "mes", "ohs", "now", "boa", "cet", "pal", "bar", "die", "war", "hay", "eco", "pub",
        "lob", "rue", "fry", "lit", "rex", "jan", "cot", "bid", "ali", "pay", "col", "gum", "ger",
        "row", "won", "dan", "rum", "fad", "tut", "sag", "yip", "sui", "ark", "has", "zip", "fez",
        "own", "ump", "dis", "ads", "max", "jaw", "out", "btu", "ana", "gap", "cry", "led", "abe",
        "box", "ore", "pig", "fie", "toy", "fat", "cal", "lie", "noh", "sew", "ono", "tam", "flu",
        "mgm", "ply", "awe", "pry", "tit", "tie", "yet", "too", "tax", "jim", "san", "pan", "map",
        "ski", "ova", "wed", "non", "wac", "nut", "why", "bye", "lye", "oct", "old", "fin", "feb",
        "chi", "sap", "owl", "log", "tod", "dot", "bow", "fob", "for", "joe", "ivy", "fan", "age",
        "fax", "hip", "jib", "mel", "hus", "sob", "ifs", "tab", "ara", "dab", "jag", "jar", "arm",
        "lot", "tom", "sax", "tex", "yum", "pei", "wen", "wry", "ire", "irk", "far", "mew", "wit",
        "doe", "gas", "rte", "ian", "pot", "ask", "wag", "hag", "amy", "nag", "ron", "soy", "gin",
        "don", "tug", "fay", "vic", "boo", "nam", "ave", "buy", "sop", "but", "orb", "fen", "paw",
        "his", "sub", "bob", "yea", "oft", "inn", "rod", "yam", "pew", "web", "hod", "hun", "gyp",
        "wei", "wis", "rob", "gad", "pie", "mon", "dog", "bib", "rub", "ere", "dig", "era", "cat",
        "fox", "bee", "mod", "day", "apr", "vie", "nev", "jam", "pam", "new", "aye", "ani", "and",
        "ibm", "yap", "can", "pyx", "tar", "kin", "fog", "hum", "pip", "cup", "dye", "lyx", "jog",
        "nun", "par", "wan", "fey", "bus", "oak", "bad", "ats", "set", "qom", "vat", "eat", "pus",
        "rev", "axe", "ion", "six", "ila", "lao", "mom", "mas", "pro", "few", "opt", "poe", "art",
        "ash", "oar", "cap", "lop", "may", "shy", "rid", "bat", "sum", "rim", "fee", "bmw", "sky",
        "maj", "hue", "thy", "ava", "rap", "den", "fla", "auk", "cox", "ibo", "hey", "saw", "vim",
        "sec", "ltd", "you", "its", "tat", "dew", "eva", "tog", "ram", "let", "see", "zit", "maw",
        "nix", "ate", "gig", "rep", "owe", "ind", "hog", "eve", "sam", "zoo", "any", "dow", "cod",
        "bed", "vet", "ham", "sis", "hex", "via", "fir", "nod", "mao", "aug", "mum", "hoe", "bah",
        "hal", "keg", "hew", "zed", "tow", "gog", "ass", "dem", "who", "bet", "gos", "son", "ear",
        "spy", "kit", "boy", "due", "sen", "oaf", "mix", "hep", "fur", "ada", "bin", "nil", "mia",
        "ewe", "hit", "fix", "sad", "rib", "eye", "hop", "haw", "wax", "mid", "tad", "ken", "wad",
        "rye", "pap", "bog", "gut", "ito", "woe", "our", "ado", "sin", "mad", "ray", "hon", "roy",
        "dip", "hen", "iva", "lug", "asp", "hui", "yak", "bay", "poi", "yep", "bun", "try", "lad",
        "elm", "nat", "wyo", "gym", "dug", "toe", "dee", "wig", "sly", "rip", "geo", "cog", "pas",
        "zen", "odd", "nan", "lay", "pod", "fit", "hem", "joy", "bum", "rio", "yon", "dec", "leg",
        "put", "sue", "dim", "pet", "yaw", "nub", "bit", "bur", "sid", "sun", "oil", "red", "doc",
        "moe", "caw", "eel", "dix", "cub", "end", "gem", "off", "yew", "hug", "pop", "tub", "sgt",
        "lid", "pun", "ton", "sol", "din", "yup", "jab", "pea", "bug", "gag", "mil", "jig", "hub",
        "low", "did", "tin", "get", "gte", "sox", "lei", "mig", "fig", "lon", "use", "ban", "flo",
        "nov", "jut", "bag", "mir", "sty", "lap", "two", "ins", "con", "ant", "net", "tux", "ode",
        "stu", "mug", "cad", "nap", "gun", "fop", "tot", "sow", "sal", "sic", "ted", "wot", "del",
        "imp", "cob", "way", "ann", "tan", "mci", "job", "wet", "ism", "err", "him", "all", "pad",
        "hah", "hie", "aim", "ike", "jed", "ego", "mac", "baa", "min", "com", "ill", "was", "cab",
        "ago", "ina", "big", "ilk", "gal", "tap", "duh", "ola", "ran", "lab", "top", "gob", "hot",
        "ora", "tia", "kip", "han", "met", "hut", "she", "sac", "fed", "goo", "tee", "ell", "not",
        "act", "gil", "rut", "ala", "ape", "rig", "cid", "god", "duo", "lin", "aid", "gel", "awl",
        "lag", "elf", "liz", "ref", "aha", "fib", "oho", "tho", "her", "nor", "ace", "adz", "fun",
        "ned", "coo", "win", "tao", "coy", "van", "man", "pit", "guy", "foe", "hid", "mai", "sup",
        "jay", "hob", "mow", "jot", "are", "pol", "arc", "lax", "aft", "alb", "len", "air", "pug",
        "pox", "vow", "got", "meg", "zoe", "amp", "ale", "bud", "gee", "pin", "dun", "pat", "ten",
        "mob"
      ])
    )
    .iter(),
    vec![
      strs_to_vec(&["cet", "get", "gee", "gte", "ate", "ats", "its", "ito", "ibo", "ibm", "ism",]),
      strs_to_vec(&["cet", "cat", "can", "ian", "inn", "ins", "its", "ito", "ibo", "ibm", "ism",]),
      strs_to_vec(&["cet", "cot", "con", "ion", "inn", "ins", "its", "ito", "ibo", "ibm", "ism",])
    ]
    .iter()
  ));
}
