mod passwordgem;

use std::io;
fn main() {
    println!("How long would you like the password to be?");
    let mut lengthofpassword = String::new();
    io::stdin().read_line(&mut lengthofpassword).unwrap();
    let lengthof = lengthofpassword.trim().parse::<u8>().unwrap();
    println!("How many passwords would you like the password to be?");
    let mut numofpasswords = String::new();
    io::stdin().read_line(&mut numofpasswords).unwrap();
    numofpasswords.trim().parse::<u8>().unwrap();
    let numof = numofpasswords.trim().parse::<u8>().unwrap();

    let mut lcase = String::new();
    println!("Would you like lowwer case letters to be used (y/n)?");
    io::stdin().read_line(&mut lcase).unwrap();
    lcase = lcase.trim().to_string();

    let mut ucase = String::new();
    println!("Would you like upper case letters to be used (y/n)?");
    io::stdin().read_line(&mut ucase).unwrap();
    ucase = ucase.trim().to_string();

    let mut onums = String::new();
    println!("Would you like numbers to be used? (y/n)");
    io::stdin().read_line(&mut onums).unwrap();
    onums = onums.trim().to_string();

    let mut osimbs = String::new();
    println!("Would you like symbols to be used? (y/n)");
    io::stdin().read_line(&mut osimbs).unwrap();
    osimbs = osimbs.trim().to_string();

    passwordgem::passwordgen(lengthof, numof, lcase, ucase, onums, osimbs)
}