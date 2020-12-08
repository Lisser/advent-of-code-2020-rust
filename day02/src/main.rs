use std::fs;

#[derive(Copy, Clone)]
struct PasswordTuple<'a> {
    min_occ: i32,
    max_occ: i32,
    letter: char,
    password: &'a str,
}

impl <'a>PasswordTuple<'a> {
    fn is_valid (&'a self) -> bool {

        let mut first_matched = false;
        let mut second_matched = false;
        for (position, character) in self.password.chars().enumerate() {
            if character == self.letter && (position + 1) as i32 == self.min_occ {
                first_matched = true;
            }
            if character == self.letter && (position + 1) as i32 == self.max_occ {
                second_matched = true;
            }
        }

        let is_valid = first_matched ^ second_matched;

        
        println!("{} ^ {} = {}: {}-{} {}: {} ", first_matched, second_matched, first_matched ^ second_matched, self.min_occ, self.max_occ, self.letter, self.password);

        is_valid
    }
}

// Example: 15-16 r: rrrrrrrrrrrrrrwr
// Indices: ^0    ^1 ^2
fn build_password_tuple(input: &str) -> PasswordTuple {
    // Split by whitespace
    let mut parts = input.split_whitespace();
    
    let min_max = parts.next().unwrap();
    let letter_part = parts.next().unwrap();
    let password = parts.next().unwrap();

    let mut min_max_parts = min_max.split("-");
    
    PasswordTuple {
        min_occ: min_max_parts.next().unwrap().parse::<i32>().unwrap(),
        max_occ: min_max_parts.next().unwrap().parse::<i32>().unwrap(),
        letter: letter_part.chars().next().unwrap(),
        password: password
    }
}

fn main() {
    let filename = "input.txt";

    println!("In file {}", filename);

    // Read File
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("{}", contents);

    // Parse into number collection
    let mut collection: Vec<PasswordTuple> = Vec::new();
    for entry in contents.lines() {
        collection.push(build_password_tuple(entry));
    }

    
    println!("\n");
    println!("{}", collection.iter().count());
    println!("\n");
    let x = collection.iter()
        // .cloned()
        .filter(|t| t.is_valid());

    println!("{}", x.cloned().count());

    // OK(())
    // x.

    // println!(x.)

}

#[test]
fn test_build_password_tuple () {
    let tuple = build_password_tuple("15-16 r: rrrrrrrrrrrrrrwr");
    assert_eq!(tuple.min_occ, 15);
    assert_eq!(tuple.max_occ, 16);
    assert_eq!(tuple.letter, 'r');
    assert_eq!(tuple.password, "rrrrrrrrrrrrrrwr");
}

#[test]
fn test_build_password_tuple_2 () {
    let tuple = build_password_tuple("10-12 r: rrrrrrrrrrrrrrwr");
    assert_eq!(tuple.min_occ, 10);
    assert_eq!(tuple.max_occ, 12);
    assert_eq!(tuple.letter, 'r');
    assert_eq!(tuple.password, "rrrrrrrrrrrrrrwr");
}

#[test]
fn test_password_tuple_valid () {
    assert_eq!(build_password_tuple("1-3 a: abcde").is_valid(), true);
    assert_eq!(build_password_tuple("1-3 b: cdefg").is_valid(), false);
    assert_eq!(build_password_tuple("2-9 c: ccccccccc").is_valid(), false);
}