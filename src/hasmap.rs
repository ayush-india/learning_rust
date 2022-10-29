use std::collections::HashMap;
use std::io::stdin;

/// enters a name and socre for
/// two teams
/// writing it to brush up on my basics
pub fn game_for_two() {
    println!("Enter a name fro two teams");
    let mut input1 = String::new();
    let mut scr1 = String::new();

    println!("Enter the name for the first team ");
    stdin().read_line(&mut input1).expect("Unable to read line");

    println!("Enter the score for the first team ");
    stdin().read_line(&mut scr1).expect("Unable to read line");

    let scr1: u32 = scr1.trim().parse().expect("Enter a num u dumb");

    let mut input2 = String::new();
    let mut scr2 = String::new();

    println!("Enter the name for the snd team ");
    stdin().read_line(&mut input2).expect("Unable to read line");

    println!("Enter the score for the snc team ");
    stdin().read_line(&mut scr2).expect("Unable to read line");

    let scr2: u32 = scr2.trim().parse().expect("Enter a num u dumb");

    let mut idk = HashMap::new();

    idk.insert(input1, scr1);
    idk.insert(input2, scr2);

    for x in &idk{
        println!("{}{}", x.0 , x.1);
    }
}
