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

fn find_ladders_dfs(
  begin_word: String,
  end_word: String,
  word_list: Vec<String>,
) -> Vec<Vec<String>> {
  let mut temp: Vec<Vec<String>> = Vec::new();
  let mut min_size = 0;

  fn dfs(
    current: &String,
    input_list: Vec<String>,
    mut output_list: Vec<String>,
    end_word: &String,
    temp: &mut Vec<Vec<String>>,
    min_size: &mut usize,
  ) {
    if super::check_word(&current, end_word) {
      output_list.push(end_word.clone());
      let size = output_list.len();
      if *min_size == 0 || size < *min_size {
        *min_size = size;
      }
      temp.push(output_list);
      return;
    }
    for i in 0..input_list.len() {
      let next = &input_list[i];
      if super::check_word(&current, &next) {
        let (mut new_output_list, mut new_input_list) = (output_list.clone(), input_list.clone());
        new_output_list.push(next.clone());
        new_input_list.remove(i);
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

  dbg!((min_size, &temp));

  temp
    .into_iter()
    .filter(|v| v.len() == min_size && v.last() == Some(&end_word))
    .collect()
}

fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
  use std::collections::{HashSet, VecDeque};
  use std::iter::FromIterator;

  let mut result: Vec<Vec<String>> = vec![];
  let mut currents: HashSet<String> = HashSet::new();
  let mut word_set: HashSet<String> = HashSet::from_iter(word_list);

  let mut paths = VecDeque::new();
  paths.push_back(vec![begin_word]);

  if word_set.contains(&end_word) {
    let (mut level, mut min_level) = (1, word_set.len());
    while let Some(path) = paths.pop_front() {
      if path.len() > level {
        for word in &currents {
          word_set.remove(word);
        }
        currents.clear();
        level = path.len();
        if level > min_level {
          break;
        }
      }
      if let Some(current) = path.last() {
        let bytes: Vec<u8> = current.bytes().collect();
        for i in 0..bytes.len() {
          // 'a' .. 'z' ASCII number
          for u in 97_u8..=122_u8 {
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
  fn vec_string(str_array: &[&str]) -> Vec<String> {
    str_array.iter().map(|v| v.to_string()).collect()
  }

  let empty: Vec<Vec<String>> = vec![];

  assert_eq!(
    find_ladders(
      "hot".to_string(),
      "dog".to_string(),
      vec_string(&["hot", "dog", "dot"])
    ),
    vec![vec_string(&["hot", "dot", "dog"])]
  );

  assert_eq!(
    find_ladders(
      "hot".to_string(),
      "dog".to_string(),
      vec_string(&["hot", "dog"])
    ),
    empty
  );

  assert_eq!(
    find_ladders(
      "a".to_string(),
      "c".to_string(),
      vec_string(&["a", "b", "c"])
    ),
    vec![vec_string(&["a", "c"])]
  );

  assert_eq!(
    find_ladders(
      "hit".to_string(),
      "cog".to_string(),
      vec_string(&["hot", "dot", "dog", "lot", "log"])
    ),
    empty
  );

  fn is_same(input1: Vec<Vec<String>>, input2: Vec<Vec<String>>) -> bool {
    use std::collections::HashSet;
    fn vec_to_set(input: &Vec<Vec<String>>) -> HashSet<&Vec<String>> {
      let mut set = HashSet::new();
      for i in input {
        set.insert(i);
      }
      set
    }
    match vec_to_set(&input1) == vec_to_set(&input2) {
      true => true,
      false => {
        dbg!((&input1, &input2));
        false
      }
    }
  }

  assert!(is_same(
    find_ladders(
      "hit".to_string(),
      "cog".to_string(),
      vec_string(&["hot", "dot", "dog", "lot", "log", "cog"])
    ),
    vec![
      vec_string(&["hit", "hot", "lot", "log", "cog"]),
      vec_string(&["hit", "hot", "dot", "dog", "cog"])
    ]
  ));

  assert!(is_same(
    find_ladders(
      "qa".to_string(),
      "sq".to_string(),
      vec_string(&[
        "si", "go", "se", "cm", "so", "ph", "mt", "db", "mb", "sb", "kr", "ln", "tm", "le", "av",
        "sm", "ar", "ci", "ca", "br", "ti", "ba", "to", "ra", "fa", "yo", "ow", "sn", "ya", "cr",
        "po", "fe", "ho", "ma", "re", "or", "rn", "au", "ur", "rh", "sr", "tc", "lt", "lo", "as",
        "fr", "nb", "yb", "if", "pb", "ge", "th", "pm", "rb", "sh", "co", "ga", "li", "ha", "hz",
        "no", "bi", "di", "hi", "qa", "pi", "os", "uh", "wm", "an", "me", "mo", "na", "la", "st",
        "er", "sc", "ne", "mn", "mi", "am", "ex", "pt", "io", "be", "fm", "ta", "tb", "ni", "mr",
        "pa", "he", "lr", "sq", "ye"
      ])
    ),
    vec![
      vec_string(&["qa", "ba", "be", "se", "sq"]),
      vec_string(&["qa", "ba", "bi", "si", "sq"]),
      vec_string(&["qa", "ba", "br", "sr", "sq"]),
      vec_string(&["qa", "ca", "ci", "si", "sq"]),
      vec_string(&["qa", "ca", "cm", "sm", "sq"]),
      vec_string(&["qa", "ca", "co", "so", "sq"]),
      vec_string(&["qa", "ca", "cr", "sr", "sq"]),
      vec_string(&["qa", "fa", "fe", "se", "sq"]),
      vec_string(&["qa", "fa", "fm", "sm", "sq"]),
      vec_string(&["qa", "fa", "fr", "sr", "sq"]),
      vec_string(&["qa", "ga", "ge", "se", "sq"]),
      vec_string(&["qa", "ga", "go", "so", "sq"]),
      vec_string(&["qa", "ha", "he", "se", "sq"]),
      vec_string(&["qa", "ha", "hi", "si", "sq"]),
      vec_string(&["qa", "ha", "ho", "so", "sq"]),
      vec_string(&["qa", "la", "le", "se", "sq"]),
      vec_string(&["qa", "la", "li", "si", "sq"]),
      vec_string(&["qa", "la", "ln", "sn", "sq"]),
      vec_string(&["qa", "la", "lo", "so", "sq"]),
      vec_string(&["qa", "la", "lr", "sr", "sq"]),
      vec_string(&["qa", "la", "lt", "st", "sq"]),
      vec_string(&["qa", "ma", "mb", "sb", "sq"]),
      vec_string(&["qa", "ma", "me", "se", "sq"]),
      vec_string(&["qa", "ma", "mi", "si", "sq"]),
      vec_string(&["qa", "ma", "mn", "sn", "sq"]),
      vec_string(&["qa", "ma", "mo", "so", "sq"]),
      vec_string(&["qa", "ma", "mr", "sr", "sq"]),
      vec_string(&["qa", "ma", "mt", "st", "sq"]),
      vec_string(&["qa", "na", "nb", "sb", "sq"]),
      vec_string(&["qa", "na", "ne", "se", "sq"]),
      vec_string(&["qa", "na", "ni", "si", "sq"]),
      vec_string(&["qa", "na", "no", "so", "sq"]),
      vec_string(&["qa", "pa", "pb", "sb", "sq"]),
      vec_string(&["qa", "pa", "ph", "sh", "sq"]),
      vec_string(&["qa", "pa", "pi", "si", "sq"]),
      vec_string(&["qa", "pa", "pm", "sm", "sq"]),
      vec_string(&["qa", "pa", "po", "so", "sq"]),
      vec_string(&["qa", "pa", "pt", "st", "sq"]),
      vec_string(&["qa", "ra", "rb", "sb", "sq"]),
      vec_string(&["qa", "ra", "re", "se", "sq"]),
      vec_string(&["qa", "ra", "rh", "sh", "sq"]),
      vec_string(&["qa", "ra", "rn", "sn", "sq"]),
      vec_string(&["qa", "ta", "tb", "sb", "sq"]),
      vec_string(&["qa", "ta", "tc", "sc", "sq"]),
      vec_string(&["qa", "ta", "th", "sh", "sq"]),
      vec_string(&["qa", "ta", "ti", "si", "sq"]),
      vec_string(&["qa", "ta", "tm", "sm", "sq"]),
      vec_string(&["qa", "ta", "to", "so", "sq"]),
      vec_string(&["qa", "ya", "yb", "sb", "sq"]),
      vec_string(&["qa", "ya", "ye", "se", "sq"]),
      vec_string(&["qa", "ya", "yo", "so", "sq"]),
    ]
  ));

  assert!(is_same(
    find_ladders(
      "cet".to_string(),
      "ism".to_string(),
      vec_string(&[
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
    ),
    vec![
      vec_string(&["cet", "get", "gee", "gte", "ate", "ats", "its", "ito", "ibo", "ibm", "ism",]),
      vec_string(&["cet", "cat", "can", "ian", "inn", "ins", "its", "ito", "ibo", "ibm", "ism",]),
      vec_string(&["cet", "cot", "con", "ion", "inn", "ins", "its", "ito", "ibo", "ibm", "ism",])
    ]
  ));
}
