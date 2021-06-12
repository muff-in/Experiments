
use regex::Regex;

fn main() {
      
let re = Regex::new(r"^[0-9]{4}-[0-9]{3}-[0-9]{3}$").unwrap();

assert!(re.is_match("7891-661-690")); //Checking a phone number

let email: Regex = Regex::new(r"^[a-zA-z0-9]+@+[a-zA-Z0-9]+\.[a-z]{3}$").unwrap();

assert!(email.is_match("test123@test123.com")); //Checking an email 

print!(r#"Compiled Successfully"#);

//let country: Regex = Regex::new()
    
}

// assert!(Regex::new(r"\b\w{13}\b").unwrap().is_match(text));
