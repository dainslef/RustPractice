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

// use dfs to find all possible result, but time limit exceeded
fn find_ladders_dfs(
  begin_word: String,
  end_word: String,
  word_list: Vec<String>,
) -> Vec<Vec<String>> {
  let (mut temp, mut min_size) = (vec![], 0);

  // use recursion to find out all path
  fn dfs(
    current_word: &String,
    input: Vec<String>,
    mut output: Vec<String>,
    end_word: &String,
    temp: &mut Vec<Vec<String>>,
    min_size: &mut usize,
  ) {
    // check if find out the target path
    if check_diff_one_char(&current_word, end_word) {
      output.push(end_word.clone());
      let size = output.len();
      // check and update the path size
      if *min_size == 0 || size < *min_size {
        *min_size = size;
      }
      // add the path to the temp
      temp.push(output);
      // find the target, finish this recursion
      return;
    }
    for i in 0..input.len() {
      // get the next word
      let next_word = &input[i];
      // check the next word if match the condition
      if check_diff_one_char(&current_word, &next_word) {
        // copy and update the input/ouput list
        let (mut next_output, mut next_input) = (output.clone(), input.clone());
        next_output.push(next_word.clone());
        next_input.remove(i);
        // recursively check the next word until find all the paths
        dfs(next_word, next_input, next_output, end_word, temp, min_size);
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

  // save all of the target paths which match the condition
  let mut results: Vec<Vec<String>> = vec![];
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
      if let Some(current_word) = path.last() {
        let bytes = current_word.clone().into_bytes();
        for i in 0..bytes.len() {
          // 'a' .. 'z' ASCII number
          for u in 97_u8..=122_u8 {
            // change a character in the word and check if the new word is contained by the word set
            let mut bytes = bytes.clone();
            bytes[i] = u;
            if let Ok(next_word) = String::from_utf8(bytes) {
              if word_set.contains(&next_word) {
                currents.insert(next_word.clone());
                let mut new_path = path.clone();
                new_path.push(next_word.clone());
                if next_word == end_word {
                  min_level = new_path.len();
                  results.push(new_path);
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

  results
}

#[test]
fn test_q126() {
  fn test(find_ladders: impl Fn(String, String, Vec<String>) -> Vec<Vec<String>>) {
    let empty: Vec<Vec<String>> = vec![];

    assert_eq!(
      find_ladders("hot".into(), "dog".into(), string_vec!["hot", "dog", "dot"]),
      vec![string_vec!["hot", "dot", "dog"]]
    );

    assert_eq!(
      find_ladders("hot".into(), "dog".into(), string_vec!["hot", "dog"]),
      empty
    );

    assert_eq!(
      find_ladders("a".into(), "c".into(), string_vec!["a", "b", "c"]),
      vec![string_vec!["a", "c"]]
    );

    assert_eq!(
      find_ladders(
        "hit".into(),
        "cog".into(),
        string_vec!["hot", "dot", "dog", "lot", "log"]
      ),
      empty
    );

    assert!(check_element_eq(
      find_ladders(
        "hit".into(),
        "cog".into(),
        string_vec!["hot", "dot", "dog", "lot", "log", "cog"]
      ),
      vec![
        string_vec!["hit", "hot", "lot", "log", "cog"],
        string_vec!["hit", "hot", "dot", "dog", "cog"]
      ]
    ));

    assert!(check_element_eq(
      find_ladders(
        "qa".into(),
        "sq".into(),
        string_vec![
          "si", "go", "se", "cm", "so", "ph", "mt", "db", "mb", "sb", "kr", "ln", "tm", "le", "av",
          "sm", "ar", "ci", "ca", "br", "ti", "ba", "to", "ra", "fa", "yo", "ow", "sn", "ya", "cr",
          "po", "fe", "ho", "ma", "re", "or", "rn", "au", "ur", "rh", "sr", "tc", "lt", "lo", "as",
          "fr", "nb", "yb", "if", "pb", "ge", "th", "pm", "rb", "sh", "co", "ga", "li", "ha", "hz",
          "no", "bi", "di", "hi", "qa", "pi", "os", "uh", "wm", "an", "me", "mo", "na", "la", "st",
          "er", "sc", "ne", "mn", "mi", "am", "ex", "pt", "io", "be", "fm", "ta", "tb", "ni", "mr",
          "pa", "he", "lr", "sq", "ye"
        ]
      ),
      vec![
        string_vec!["qa", "ba", "be", "se", "sq"],
        string_vec!["qa", "ba", "bi", "si", "sq"],
        string_vec!["qa", "ba", "br", "sr", "sq"],
        string_vec!["qa", "ca", "ci", "si", "sq"],
        string_vec!["qa", "ca", "cm", "sm", "sq"],
        string_vec!["qa", "ca", "co", "so", "sq"],
        string_vec!["qa", "ca", "cr", "sr", "sq"],
        string_vec!["qa", "fa", "fe", "se", "sq"],
        string_vec!["qa", "fa", "fm", "sm", "sq"],
        string_vec!["qa", "fa", "fr", "sr", "sq"],
        string_vec!["qa", "ga", "ge", "se", "sq"],
        string_vec!["qa", "ga", "go", "so", "sq"],
        string_vec!["qa", "ha", "he", "se", "sq"],
        string_vec!["qa", "ha", "hi", "si", "sq"],
        string_vec!["qa", "ha", "ho", "so", "sq"],
        string_vec!["qa", "la", "le", "se", "sq"],
        string_vec!["qa", "la", "li", "si", "sq"],
        string_vec!["qa", "la", "ln", "sn", "sq"],
        string_vec!["qa", "la", "lo", "so", "sq"],
        string_vec!["qa", "la", "lr", "sr", "sq"],
        string_vec!["qa", "la", "lt", "st", "sq"],
        string_vec!["qa", "ma", "mb", "sb", "sq"],
        string_vec!["qa", "ma", "me", "se", "sq"],
        string_vec!["qa", "ma", "mi", "si", "sq"],
        string_vec!["qa", "ma", "mn", "sn", "sq"],
        string_vec!["qa", "ma", "mo", "so", "sq"],
        string_vec!["qa", "ma", "mr", "sr", "sq"],
        string_vec!["qa", "ma", "mt", "st", "sq"],
        string_vec!["qa", "na", "nb", "sb", "sq"],
        string_vec!["qa", "na", "ne", "se", "sq"],
        string_vec!["qa", "na", "ni", "si", "sq"],
        string_vec!["qa", "na", "no", "so", "sq"],
        string_vec!["qa", "pa", "pb", "sb", "sq"],
        string_vec!["qa", "pa", "ph", "sh", "sq"],
        string_vec!["qa", "pa", "pi", "si", "sq"],
        string_vec!["qa", "pa", "pm", "sm", "sq"],
        string_vec!["qa", "pa", "po", "so", "sq"],
        string_vec!["qa", "pa", "pt", "st", "sq"],
        string_vec!["qa", "ra", "rb", "sb", "sq"],
        string_vec!["qa", "ra", "re", "se", "sq"],
        string_vec!["qa", "ra", "rh", "sh", "sq"],
        string_vec!["qa", "ra", "rn", "sn", "sq"],
        string_vec!["qa", "ta", "tb", "sb", "sq"],
        string_vec!["qa", "ta", "tc", "sc", "sq"],
        string_vec!["qa", "ta", "th", "sh", "sq"],
        string_vec!["qa", "ta", "ti", "si", "sq"],
        string_vec!["qa", "ta", "tm", "sm", "sq"],
        string_vec!["qa", "ta", "to", "so", "sq"],
        string_vec!["qa", "ya", "yb", "sb", "sq"],
        string_vec!["qa", "ya", "ye", "se", "sq"],
        string_vec!["qa", "ya", "yo", "so", "sq"],
      ]
    ));

    assert!(check_element_eq(
      find_ladders(
        "cet".into(),
        "ism".into(),
        string_vec![
          "kid", "tag", "pup", "ail", "tun", "woo", "erg", "luz", "brr", "gay", "sip", "kay",
          "per", "val", "mes", "ohs", "now", "boa", "cet", "pal", "bar", "die", "war", "hay",
          "eco", "pub", "lob", "rue", "fry", "lit", "rex", "jan", "cot", "bid", "ali", "pay",
          "col", "gum", "ger", "row", "won", "dan", "rum", "fad", "tut", "sag", "yip", "sui",
          "ark", "has", "zip", "fez", "own", "ump", "dis", "ads", "max", "jaw", "out", "btu",
          "ana", "gap", "cry", "led", "abe", "box", "ore", "pig", "fie", "toy", "fat", "cal",
          "lie", "noh", "sew", "ono", "tam", "flu", "mgm", "ply", "awe", "pry", "tit", "tie",
          "yet", "too", "tax", "jim", "san", "pan", "map", "ski", "ova", "wed", "non", "wac",
          "nut", "why", "bye", "lye", "oct", "old", "fin", "feb", "chi", "sap", "owl", "log",
          "tod", "dot", "bow", "fob", "for", "joe", "ivy", "fan", "age", "fax", "hip", "jib",
          "mel", "hus", "sob", "ifs", "tab", "ara", "dab", "jag", "jar", "arm", "lot", "tom",
          "sax", "tex", "yum", "pei", "wen", "wry", "ire", "irk", "far", "mew", "wit", "doe",
          "gas", "rte", "ian", "pot", "ask", "wag", "hag", "amy", "nag", "ron", "soy", "gin",
          "don", "tug", "fay", "vic", "boo", "nam", "ave", "buy", "sop", "but", "orb", "fen",
          "paw", "his", "sub", "bob", "yea", "oft", "inn", "rod", "yam", "pew", "web", "hod",
          "hun", "gyp", "wei", "wis", "rob", "gad", "pie", "mon", "dog", "bib", "rub", "ere",
          "dig", "era", "cat", "fox", "bee", "mod", "day", "apr", "vie", "nev", "jam", "pam",
          "new", "aye", "ani", "and", "ibm", "yap", "can", "pyx", "tar", "kin", "fog", "hum",
          "pip", "cup", "dye", "lyx", "jog", "nun", "par", "wan", "fey", "bus", "oak", "bad",
          "ats", "set", "qom", "vat", "eat", "pus", "rev", "axe", "ion", "six", "ila", "lao",
          "mom", "mas", "pro", "few", "opt", "poe", "art", "ash", "oar", "cap", "lop", "may",
          "shy", "rid", "bat", "sum", "rim", "fee", "bmw", "sky", "maj", "hue", "thy", "ava",
          "rap", "den", "fla", "auk", "cox", "ibo", "hey", "saw", "vim", "sec", "ltd", "you",
          "its", "tat", "dew", "eva", "tog", "ram", "let", "see", "zit", "maw", "nix", "ate",
          "gig", "rep", "owe", "ind", "hog", "eve", "sam", "zoo", "any", "dow", "cod", "bed",
          "vet", "ham", "sis", "hex", "via", "fir", "nod", "mao", "aug", "mum", "hoe", "bah",
          "hal", "keg", "hew", "zed", "tow", "gog", "ass", "dem", "who", "bet", "gos", "son",
          "ear", "spy", "kit", "boy", "due", "sen", "oaf", "mix", "hep", "fur", "ada", "bin",
          "nil", "mia", "ewe", "hit", "fix", "sad", "rib", "eye", "hop", "haw", "wax", "mid",
          "tad", "ken", "wad", "rye", "pap", "bog", "gut", "ito", "woe", "our", "ado", "sin",
          "mad", "ray", "hon", "roy", "dip", "hen", "iva", "lug", "asp", "hui", "yak", "bay",
          "poi", "yep", "bun", "try", "lad", "elm", "nat", "wyo", "gym", "dug", "toe", "dee",
          "wig", "sly", "rip", "geo", "cog", "pas", "zen", "odd", "nan", "lay", "pod", "fit",
          "hem", "joy", "bum", "rio", "yon", "dec", "leg", "put", "sue", "dim", "pet", "yaw",
          "nub", "bit", "bur", "sid", "sun", "oil", "red", "doc", "moe", "caw", "eel", "dix",
          "cub", "end", "gem", "off", "yew", "hug", "pop", "tub", "sgt", "lid", "pun", "ton",
          "sol", "din", "yup", "jab", "pea", "bug", "gag", "mil", "jig", "hub", "low", "did",
          "tin", "get", "gte", "sox", "lei", "mig", "fig", "lon", "use", "ban", "flo", "nov",
          "jut", "bag", "mir", "sty", "lap", "two", "ins", "con", "ant", "net", "tux", "ode",
          "stu", "mug", "cad", "nap", "gun", "fop", "tot", "sow", "sal", "sic", "ted", "wot",
          "del", "imp", "cob", "way", "ann", "tan", "mci", "job", "wet", "ism", "err", "him",
          "all", "pad", "hah", "hie", "aim", "ike", "jed", "ego", "mac", "baa", "min", "com",
          "ill", "was", "cab", "ago", "ina", "big", "ilk", "gal", "tap", "duh", "ola", "ran",
          "lab", "top", "gob", "hot", "ora", "tia", "kip", "han", "met", "hut", "she", "sac",
          "fed", "goo", "tee", "ell", "not", "act", "gil", "rut", "ala", "ape", "rig", "cid",
          "god", "duo", "lin", "aid", "gel", "awl", "lag", "elf", "liz", "ref", "aha", "fib",
          "oho", "tho", "her", "nor", "ace", "adz", "fun", "ned", "coo", "win", "tao", "coy",
          "van", "man", "pit", "guy", "foe", "hid", "mai", "sup", "jay", "hob", "mow", "jot",
          "are", "pol", "arc", "lax", "aft", "alb", "len", "air", "pug", "pox", "vow", "got",
          "meg", "zoe", "amp", "ale", "bud", "gee", "pin", "dun", "pat", "ten", "mob"
        ]
      ),
      vec![
        string_vec!["cet", "get", "gee", "gte", "ate", "ats", "its", "ito", "ibo", "ibm", "ism"],
        string_vec!["cet", "cat", "can", "ian", "inn", "ins", "its", "ito", "ibo", "ibm", "ism"],
        string_vec!["cet", "cot", "con", "ion", "inn", "ins", "its", "ito", "ibo", "ibm", "ism"]
      ]
    ));
  }

  test(find_ladders);
  // test(find_ladders_dfs); // time limit exceeded
}
