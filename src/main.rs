use rand::Rng;
use std::{fmt, io};

const WORDS: &[&str] = &[
    "abruptly",
    "absurd",
    "abyss",
    "affix",
    "askew",
    "avenue",
    "awkward",
    "axiom",
    "azure",
    "bagpipes",
    "bandwagon",
    "banjo",
    "bayou",
    "beekeeper",
    "bikini",
    "blitz",
    "blizzard",
    "boggle",
    "bookworm",
    "boxcar",
    "boxful",
    "buckaroo",
    "buffalo",
    "buffoon",
    "buxom",
    "buzzard",
    "buzzing",
    "buzzwords",
    "caliph",
    "cobweb",
    "cockiness",
    "croquet",
    "crypt",
    "curacao",
    "cycle",
    "daiquiri",
    "dirndl",
    "disavow",
    "dizzying",
    "duplex",
    "dwarves",
    "embezzle",
    "equip",
    "espionage",
    "euouae",
    "exodus",
    "faking",
    "fishhook",
    "fixable",
    "fjord",
    "flapjack",
    "flopping",
    "fluffiness",
    "flyby",
    "foxglove",
    "frazzled",
    "frizzled",
    "fuchsia",
    "funny",
    "gabby",
    "galaxy",
    "galvanize",
    "gazebo",
    "giaour",
    "gizmo",
    "glowworm",
    "glyph",
    "gnarly",
    "gnostic",
    "gossip",
    "grogginess",
    "haiku",
    "haphazard",
    "hyphen",
    "iatrogenic",
    "icebox",
    "injury",
    "ivory",
    "ivy",
    "jackpot",
    "jaundice",
    "jawbreaker",
    "jaywalk",
    "jazziest",
    "jazzy",
    "jelly",
    "jigsaw",
    "jinx",
    "jiujitsu",
    "jockey",
    "jogging",
    "joking",
    "jovial",
    "joyful",
    "juicy",
    "jukebox",
    "jumbo",
    "kayak",
    "kazoo",
    "keyhole",
    "khaki",
    "kilobyte",
    "kiosk",
    "kitsch",
    "kiwifruit",
    "klutz",
    "knapsack",
    "larynx",
    "lengths",
    "lucky",
    "luxury",
    "lymph",
    "marquis",
    "matrix",
    "megahertz",
    "microwave",
    "mnemonic",
    "mystify",
    "naphtha",
    "nightclub",
    "nowadays",
    "numbskull",
    "nymph",
    "onyx",
    "ovary",
    "oxidize",
    "oxygen",
    "pajama",
    "peekaboo",
    "phlegm",
    "pixel",
    "pizazz",
    "pneumonia",
    "polka",
    "pshaw",
    "psyche",
    "puppy",
    "puzzling",
    "quartz",
    "queue",
    "quips",
    "quixotic",
    "quiz",
    "quizzes",
    "quorum",
    "razzmatazz",
    "rhubarb",
    "rhythm",
    "rickshaw",
    "schnapps",
    "scratch",
    "shiv",
    "snazzy",
    "sphinx",
    "spritz",
    "squawk",
    "staff",
    "strength",
    "strengths",
    "stretch",
    "stronghold",
    "stymied",
    "subway",
    "swivel",
    "syndrome",
    "thriftless",
    "thumbscrew",
    "topaz",
    "transcript",
    "transgress",
    "transplant",
    "triphthong",
    "twelfth",
    "twelfths",
    "unknown",
    "unworthy",
    "unzip",
    "uptown",
    "vaporize",
    "vixen",
    "vodka",
    "voodoo",
    "vortex",
    "voyeurism",
    "walkway",
    "waltz",
    "wave",
    "wavy",
    "waxy",
    "wellspring",
    "wheezy",
    "whiskey",
    "whizzing",
    "whomever",
    "wimpy",
    "witchcraft",
    "wizard",
    "woozy",
    "wristwatch",
    "wyvern",
    "xylophone",
    "yachtsman",
    "yippee",
    "yoked",
    "youthful",
    "yummy",
    "zephyr",
    "zigzag",
    "zigzagging",
    "zilch",
    "zipper",
    "zodiac",
    "zombie",
];

/* Stuff to think about
    1. Instead of using a builder you can implement a new function inside the struct
    2. Or you could use Default::default()
        Benefits
        - able to create generic methods for anytype that implements default::defult
*/
// Represents each letter the user must guess
// hidden represents if the letter has been
// guessed (false) or not (true)
#[derive(Debug, Default)]
struct Letter {
    letter: char,
    hidden: bool,
    num: i32,
    other: Option<bool>,
    stuff: String,
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.hidden {
            write!(f, "*")
        } else {
            write!(f, "{} ", self.letter)
        }
    }
}

fn create<T>() -> T
where
    T: Default,
{
    Default::default()
}

struct Word {
    letters: Vec<Letter>,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let word: String = self.letters.iter().map(|l| format!("{}", l)).collect();

        write!(f, "-- {} --", word)
    }
}

impl Default for Word {
    fn default() -> Self {
        let random_number = rand::thread_rng().gen_range(0, WORDS.len());
        let to_guess = WORDS[random_number].to_owned();
        let letters: Vec<Letter> = to_guess
            .chars()
            .map(|letter| Letter {
                letter,
                num: 2,
                ..Default::default()
            })
            .collect();

        Word { letters }
    }
}
impl Word {
    fn win(&self) -> bool {
        let discovered: Vec<_> = self.letters.iter().filter(|letter| letter.hidden).collect();

        discovered.len() == 0
    }

    // Update the displayed word after a guess
    // @Returns if the word was updated
    fn update_letters(&mut self, guess: Vec<char>) {
        self.letters.iter_mut().for_each(|l| {
            if guess.contains(&l.letter) {
                l.hidden = false;
            }
        });
    }
}

fn main() {
    // Generates a word for the player to guess "word"
    // The word representing the player's progress "w _ o _"

    let l: Letter = create();
    let w: Word = create();

    let mut displayed_word: Word = Default::default();
    println!("{}", displayed_word);
    while !displayed_word.win() {
        displayed_word.update_letters(guess());
        println!("{}", displayed_word);
        // update_displayed_word
    }
    println!("You Win!")
}

// @Return the the user's guess
fn guess() -> Vec<char> {
    let mut input = String::new();
    while input.is_empty() {
        println!("Guess a letter or a word!");
        io::stdin().read_line(&mut input).unwrap();
        input = input.to_lowercase().trim().to_string();
    }
    let mut guess: Vec<char> = Vec::new();
    for c in input.chars() {
        guess.push(c);
    }
    guess
}
