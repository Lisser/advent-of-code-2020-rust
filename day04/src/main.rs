use std::collections::HashMap;

fn main() {

    let filename = "input2.txt";

    let input = std::fs::read_to_string(filename).unwrap();

    let required_fields
        = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    // let optional_field = "cid";


    let credentials: Vec<HashMap<&str, &str>> = input
        .split("\n\n")
        .map(|cred| {
            let mut fields = HashMap::new();
            let fields_strings = cred.split_whitespace();
            for key_value in fields_strings {
                let key_value_splitted:Vec<&str> = key_value.split(":").collect();
                fields.insert(key_value_splitted[0], key_value_splitted[1]);
            }
            fields
        })
        .collect();

    println!("{}", credentials.iter().count());

    let mut bad = 0;
    for (index, credential) in credentials.iter().enumerate() {
        // let mut is_ok = true;
        let required_available:Vec<bool> = required_fields
            .clone()
            .iter()
            .map(|f| credential.contains_key(f))
            .collect();
        match required_available.iter().find(|x| **x == false) {
            Some(_) => {
                println!("Missing!");
                println!("{}: {:?}", index, required_available);
                bad = bad + 1
            },
            None => {}
        }
    }

    println!("{}", bad);
}


// fn parse_file (str: String) -> {

// }