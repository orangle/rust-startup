use std::collections::HashMap;

pub fn value_tour() {
    let auth: bool = true;
    if auth {
        todo!()
    } else {
        todo!()
    }

    // modify value 
    let mut total = 0;
    total += 1;
    total += 2;

    // pass to function
    let name = "Try".to_string();
    print_my_name(name);

    // pass by ref 
    let map: HashMap<String, String> = HashMap::new();
    print_map(&map);
}

fn print_my_name(name: String) {
    println!("{}", name);
}

fn print_map(map: &HashMap<String, String>) {
    todo!();
}
