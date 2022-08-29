use ansi_term::Color;
use std::process::exit;
use rand::prelude::*;

const WORD_LIST: &str = include_str!("../common_words.txt");

fn split_words_by_length() -> Vec<Vec<String>> {
    const MAX_LEN: usize = 10usize;
    let words = WORD_LIST.split('\n');

    let mut array = vec![Vec::new(); MAX_LEN];

    for w in words {
        let length = w.len();
        if(length > 0 && length <= MAX_LEN) {
            array[length-1].push(w.to_string());
        }
    }


    return array;
}

#[derive(Default)]
struct Game {
    word: String,
    guesses: [String; 5],
    incorrect_guesses: u32,
    letters_exact: Vec<char>,
    letters_correct: Vec<char>,
}

fn read_line() -> String {
    let stdin = std::io::stdin();

    let mut s = String::new();
    stdin.read_line(&mut s).expect("failed to read line");

    s
}

impl Game {
    fn new(word: &str, len: u32) -> Self {
        const PLACEHOLDER: char = '_';

        let mut placeholder_word = String::new();
        for _ in 0..len {
            placeholder_word.push(PLACEHOLDER);
        }

        let guesses = [
            placeholder_word.clone(),
            placeholder_word.clone(),
            placeholder_word.clone(),
            placeholder_word.clone(),
            placeholder_word.clone(),
        ];

        Self{
            word: word.to_string().to_lowercase(),
            guesses,
            ..Default::default()
        }
    }

    fn print_word(&self, word: &str) {
        for i in 0..word.len() {
            let ch = word.chars().nth(i);
            let ch = match ch {
                Some(s) => s,
                None => { continue }
            };

            let style; 
            if self.word.chars().nth(i) == word.chars().nth(i) {
                style = Color::Green;
            } else if self.letters_correct.contains(&ch) {
                style = Color::Yellow;
            } else {
                style = Color::White;
            }


            print!("{}", style.paint(ch.to_string()));
        }
        println!("");
    }

    fn print(&self) {
        for word in &self.guesses {
            self.print_word(&word);
        }
    }

    //return true on win
    fn match_word(&mut self, word: &str) -> bool {
        let mut match_count = 0;
        for i in 0..word.len() {
            let ch = word.chars().nth(i).unwrap();
            if word.chars().nth(i) == self.word.chars().nth(i) {
                self.letters_exact.push(ch);
                match_count += 1;
            } else if self.word.contains(ch) {
                self.letters_correct.push(ch);
            }
        }

        if match_count == self.word.len() {
            return true;
        }

        false
    }
}


fn main() {
    println!("Enter word length (1-10)");
    let word_length = read_line();
    let word_length = word_length.trim_matches(' ');
    let word_length = word_length.trim_matches('\n');

    let result = u32::from_str_radix(&word_length, 10);

    let word_length = match result {
        Ok(o) => o,
        Err(_e) => {
            println!("Error: invalid input");
            exit(1);
        }
    };

    if word_length > 10 {
        println!("Error: number outside valid range");
        exit(1);
    }

    let word_pool = &split_words_by_length()[(word_length-1) as usize];

    let mut rng = thread_rng();
    let index = rng.gen_range(0..word_pool.len());
    
    let word = &word_pool[index];
    let mut game = Game::new(&word, word_length);
    
    while game.incorrect_guesses < 5 {
        game.print();

        let input = read_line();
        let input = input.trim_matches('\n').to_string();
        let input = input.to_lowercase();

        let result = game.match_word(&input);
        if result {
            println!("You win!");
            break;
        }

        game.guesses[game.incorrect_guesses as usize] = input;
        game.incorrect_guesses += 1;


        println!("-------------------");
    }

    println!("Correct word: {}", word);
}
