use ahash::AHashMap;
use lazy_static::lazy_static;

const SMALL_K: &str = "
 ##  ###   ##  #### ####  ##  #  # ###   ## #  # #     ##  ###  ###   ### #  # #   # ####
#  # #  # #  # #    #    #  # #  #  #     # # #  #    #  # #  # #  # #    #  # #   #    #
#  # ###  #    ###  ###  #    ####  #     # ##   #    #  # #  # #  # #    #  #  # #    #.
#### #  # #    #    #    # ## #  #  #     # # #  #    #  # ###  ###   ##  #  #   #    # .
#  # #  # #  # #    #    #  # #  #  #  #  # # #  #    #  # #    # #     # #  #   #   #  .
#  # ###   ##  #### #     ### #  # ###  ##  #  # ####  ##  #    #  # ###   ##    #   ####
";
const SMALL_V: &str = "ABCEFGHIJKLOPRSUYZ";

const LARGE_K: &str = "
  ##   #####   ####  ###### ######  ####  #    #    ### #    # #      #    # #####  #####  #    # ######
 #  #  #    # #    # #      #      #    # #    #     #  #   #  #      ##   # #    # #    # #    #      #
#    # #    # #      #      #      #      #    #     #  #  #   #      ##   # #    # #    #  #  #       #
#    # #    # #      #      #      #      #    #     #  # #    #      # #  # #    # #    #  #  #      #.
#    # #####  #      #####  #####  #      ######     #  ##     #      # #  # #####  #####    ##      # .
###### #    # #      #      #      #  ### #    #     #  ##     #      #  # # #      #  #     ##     #  .
#    # #    # #      #      #      #    # #    #     #  # #    #      #  # # #      #   #   #  #   #   .
#    # #    # #      #      #      #    # #    # #   #  #  #   #      #   ## #      #   #   #  #  #    .
#    # #    # #    # #      #      #   ## #    # #   #  #   #  #      #   ## #      #    # #    # #    .
#    # #####   ####  ###### #       ### # #    #  ###   #    # ###### #    # #      #    # #    # ######
";
const LARGE_V: &str = "ABCEFGHJKLNPRXZ";

const SPECIAL_K: &str = "
      ##        ##        ##    #    #  ####
     #  #      #  #      #  #  # #  ##     #
     #  #  ##  #            #  # #   #    #.
     #### #  # #           #   # #   #    #.
     #  # #  # #  #       #    # #   #   # .
     #  #  ##   ##       ####   #   ###  # .
";
const SPECIAL_V: &str = "AoC2017";

fn separate_letters(input: &str, fill_opt: Option<char>) -> Vec<String> {
    let mut res = Vec::new();
    let input = input.trim_matches(&['\n', '\r'] as &[_]);
    let fill = fill_opt.unwrap_or('#');
    let lns = input.lines().collect::<Vec<_>>();
    let mut prev_col = 0;
    for col in 0..=lns[0].len() {
        if col != lns[0].len() && (0..lns.len()).any(|row| lns[row].as_bytes()[col] == fill as u8) {
            continue;
        }
        let mut rows = Vec::new();
        for row in lns.iter() {
            rows.push(&row[prev_col..col]);
        }
        let letter = rows.join("\n");
        prev_col = col + 1;
        if letter.contains(fill) {
            res.push(
                letter
                    .replace(fill, "#")
                    .replace(|c: char| c != '#' && !c.is_whitespace(), " "),
            );
        }
    }
    res
}

lazy_static! {
    static ref SMALL_LETTERS: AHashMap<String, char> = separate_letters(SMALL_K, None)
        .into_iter()
        .zip(SMALL_V.chars())
        .collect();
    static ref LARGE_LETTERS: AHashMap<String, char> = separate_letters(LARGE_K, None)
        .into_iter()
        .zip(LARGE_V.chars())
        .collect();
    static ref SPECIAL_LETTERS: AHashMap<String, char> = separate_letters(SPECIAL_K, None)
        .into_iter()
        .zip(SPECIAL_V.chars())
        .collect();
}

pub fn parse_letters(input: &str, fill_opt: Option<char>) -> String {
    let mut res = String::new();
    let letters = separate_letters(input, fill_opt);
    for letter in letters {
        if SMALL_LETTERS.contains_key(&letter) {
            res.push(SMALL_LETTERS[&letter]);
        } else if LARGE_LETTERS.contains_key(&letter) {
            res.push(LARGE_LETTERS[&letter]);
        } else if SPECIAL_LETTERS.contains_key(&letter) {
            res.push(SPECIAL_LETTERS[&letter]);
        } else {
            panic!("Failed to parse letter: {}", letter);
        }
    }
    res
}
