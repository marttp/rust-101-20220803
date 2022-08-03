use std::collections::HashMap;

pub fn example() {
    // Create an empty hash map
    let mut heroes = HashMap::new();

    // Insert in hashmap (To overwrite use the same key)
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    // Iterate over hashmap
    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }
    println!("Length : {}", heroes.len());

    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(_x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }
}
