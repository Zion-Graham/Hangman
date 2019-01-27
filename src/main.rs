extern crate rand;
use rand::Rng;
use std::io;
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

// Represents each letter the user must guess
// hidden represents if the letter has been
// guessed (false) or not (true)
struct Letter {
    letter: char,
    hidden: bool,
}
impl Letter {
    fn show(&self) -> char {
        if self.hidden {
            '_'
        } else {
            self.letter
        }
    }
}

fn build_letter(letter: char) -> Letter {
    Letter {
        letter,
        hidden: true,
    }
}

fn main() {
    // Generates a word for the player to guess "word"
    let secret_word = generate_word().to_string();
    // The word representing the player's progress "w _ o _"
    let mut displayed_word = generate_displayed_word(&secret_word);
    while !win(&displayed_word) {
        update_displayed_word(guess(), &mut displayed_word);
        show_displayed_word(&displayed_word);
        // update_displayed_word
    }
    println!("You Win!")
}

fn win(displayed_word: &Vec<Letter>) -> bool {
    !displayed_word
        .into_iter()
        .map(|letter| letter.show() != '_')
        .collect::<Vec<bool>>()
        .contains(&false)
}

// Update the displayed word after a guess
// @Returns if the word was updated
fn update_displayed_word(guess: Vec<char>, displayed_word: &mut Vec<Letter>) -> bool {
    let mut updated: bool = false;
    if guess.len() == 1 {
        for l in displayed_word {
            if guess[0] == l.letter {
                l.hidden = false;
                updated = true;
            }
        }
    }
    updated
}

// @Return the word the player needs to guess
fn generate_word() -> String {
    let random_number = rand::thread_rng().gen_range(0, WORDS.len());
    WORDS[random_number].to_string()
}

// @Return a vec of hidden Letters
fn generate_displayed_word(secret_word: &String) -> Vec<Letter> {
    let mut displayed_word = Vec::new();
    for c in secret_word.chars() {
        displayed_word.push(build_letter(c))
    }
    show_displayed_word(&displayed_word);
    displayed_word
}

fn show_displayed_word(word: &Vec<Letter>) {
    let output: String = word
        .into_iter()
        .map(|letter| letter.show().to_string() + " ")
        .collect();
    println!("{:?}", output.trim());
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
