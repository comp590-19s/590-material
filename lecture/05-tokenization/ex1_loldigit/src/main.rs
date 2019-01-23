#![allow(unused)]
// The line above mutes warnings about unused variables, functions, etc.
// It should only be used when initially sketching out your code.

// loldigit is a tokenizer demo that reads input from stdin
use std::io;

// We need to be able to pass Peekable Char iterators around
use std::iter::Peekable;
use std::str::Chars;

// Constants work as you'd expect given experience in other languages.
// They're here to help us avoid magic numbers and strings.
const QUIT_STRING: &str = "q\n";
const EXIT_OK: i32 = 0;
const EXIT_ERR: i32 = 1;

/**
 * The Tokens of the loldigit language are as follows.
 */
#[derive(Debug)]
enum Token {
    Digit(char),
    LOL(String),
    Error,
}

// Main is a simple read-evaluate-loop. The printing is in eval.
fn main() {
    loop {
        eval(&read());
    }
}

/**
 * Evaluate user input under the given options.
 */
fn eval(input: &str) {
    println!("INPUT: {}", input);

    let mut chars = input.chars().peekable();
    println!("== TOKENS ==");
    while let Some(c) = chars.peek() {
        // Ignore new line chars
        if *c == '\n' {
            chars.next(); // Consume the new line char! Never forget!
            continue;
        }

        // Produce the next Token
        let token = match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                // We found a digit! Let's take it with `next`.
                // We can safely "unwrap" the Option since we already peeked it.
                Token::Digit(chars.next().unwrap())
            }
            'l' => { // lol tokens always begin with an 'l'
                take_lol(&mut chars)
            }
            _ => {
                // Uh-oh, this isn't a token we can recognize. Take it,
                // produce an error, and keep on iterating.
                chars.next();
                Token::Error
            }
        };

        println!("{:?}", token);
    }
    print!("\n");
}

/*
 * Take an LOL token from a peekable chars iterator. This helper function 
 * expects the next character of the iterator to be an 'l' and will panic 
 * if it is not.
 *
 * Regular Definition:
 *  out_louds 	-> ('o' | 'l')
 *  lol     	-> 'l' 'o' 'l' out_louds*
 */
fn take_lol(chars: &mut Peekable<Chars>) -> Token {
    let mut lolstr = String::new();
    lolstr.push(chars.next().unwrap()); // We know we started with an 'l'

    // Concatenation: and then the second char should be an 'o'
    if let Some(c) = chars.peek() {
        match c {
            'o' => lolstr.push(chars.next().unwrap()),
            _ => return Token::Error,
        }
    } else {
        // None means end of input.
        return Token::Error;
    }

    // Concatenation: and then the third char should be an 'l'
    if let Some(c) = chars.peek() {
        match c {
            'l' => lolstr.push(chars.next().unwrap()),
            _ => return Token::Error,
        }
    } else {
        // None means end of input.
        return Token::Error;
    }

    // TODO: out_louds* -> and then take zero or more repetitions of ('o' | 'l')

    return Token::LOL(lolstr);
}

/**
 * Read input from the user. We'll handle the case of quitting
 * via the lolstr "q" and exit the program from here. Additionally,
 * in the event of a read failure from standard input we'll exit
 * with a failure status code.
 */
fn read() -> String {
    match read_line() {
        Ok(line) => {
            if line == QUIT_STRING {
                // Exit the process with an Ok exit code.
                std::process::exit(EXIT_OK);
            } else {
                line
            }
        }
        Err(message) => {
            eprintln!("Err: {}", message);
            std::process::exit(EXIT_ERR);
        }
    }
}

/**
 * Helper function to read a line of input from stdin.
 */
fn read_line() -> Result<String, io::Error> {
    let mut input = String::new();
    // There are two interesting things happening on the line below:
    // 1. We are giving the read_line method a mutable reference to the input
    //    string. It is filling this String as a buffer via the reference
    //    rather than returning a String value. Notice the way the argument
    //    is being passed _tells_ us the method will change input's value.
    // 2. The trailing ? is a handy feature in Result/Error propagation.
    //    If read_line results in an Err, then it will automatically
    //    be returned in a propagated fashion to the caller of this fn.
    //    Otherwise, if it's Ok, it is automatically unwrapped. We are
    //    not using the returned value (number of bytes read) here since
    //    we only care about the input buffer being filled.
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
