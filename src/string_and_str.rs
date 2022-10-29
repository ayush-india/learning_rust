pub(crate) fn main() {
    // let mut foo = String::from("Foo");
    // let bar = "bar";
    // foo.push_str(bar);
    // println!("Can i use bar {foo}");
    // println!("Can i use bar {bar}");
    let hello = "Здравствуйте";
    let answer = &hello[0..5];
    let idk = String::from("Hallo");
    let idk1 = &idk[0..3];

    println!("from str {answer} {idk1}");
}
