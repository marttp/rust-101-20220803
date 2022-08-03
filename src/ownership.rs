pub fn example() {
    // If you want 2 copies use clone
    let str1 = String::from("World");
    let _str2 = str1.clone();
    println!("Hello {}", str1);
    // The above doesn't apply with data types :
    // Integers, bool, char, floats, tuples with the above data types only

    // Here the string was borrowed by the function
    let str3: String = String::from("World");
    print_str(str3);

    // This throws an error because the string was dropped when the
    // function ends
    // println!("str3 = {}", str3);

    let str4: String = String::from("World");
    let str5 = print_return_str(str4);
    println!("str5 = {}", str5);

    let mut str6: String = String::from("Derek");
    change_string(&mut str6);
}

fn print_str(x: String) {
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" is Happy");
    println!("Message : {}", name);
}
