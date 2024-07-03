use rand::prelude::ThreadRng;
use rand::Rng;

pub fn passwordgen(length: u8, passwordcount: u8, lowercase: String, uppercase: String, nums: String, symbols: String) {
    let mut types: Vec<u8> = Vec::new();

    if lowercase == "y" {
        types.push(0u8);
    }
    if uppercase  == "y" {
        types.push(1u8);
    }
    if nums  == "y" {
        types.push(2u8);
    }
    if symbols  == "y" {
        types.push(3u8);
    }

    let mut rng = rand::thread_rng();
    let lowweracse: &str = "qwertyuiopasdfghjklzxcvbnm";
    let uppercase: &str = "QWERTYUIOPASDFGHJKLZXCVBNM";
    let nums: &str  = "1234567890";
    let symbols: &str = "!?[]£$()";


    for i in 0..passwordcount {
        let mut currentpass: String = String::from("");
        for i in 0..length {
            currentpass += &*chardecide(types.clone(), &mut rng, lowweracse, uppercase, nums, symbols).to_string();
        }
        println!("password {} is {}", i+1, currentpass);
        currentpass = "".to_string()
    }
}

fn chardecide(types: Vec<u8>, rng: &mut ThreadRng, lowweracse: &str, uppercase: &str, nums: &str, symbols: &str) -> char {
    let t = rng.gen_range(0..types.len());
    return randomchar(types[t], rng, lowweracse, uppercase, nums, symbols);
}

fn randomchar(typeofchar: u8, rng: &mut ThreadRng, lowweracse: &str, uppercase: &str, nums: &str, symbols: &str) -> char {
    match typeofchar {
        0 => {
            let chars: Vec<char> = lowweracse.chars().collect();
            return chars[rng.gen_range(0..chars.len())];
        }
        1 => {
            let chars: Vec<char> = uppercase.chars().collect();
            return chars[rng.gen_range(0..chars.len())];
        }
        2 => {
            let chars: Vec<char> = nums.chars().collect();
            return chars[rng.gen_range(0..chars.len())];
        }
        3 => {
            let chars: Vec<char> = symbols.chars().collect();
            return chars[rng.gen_range(0..chars.len())];
        }
        _ => {
            panic!("Invalid typeofchar value");
        }
    }
}