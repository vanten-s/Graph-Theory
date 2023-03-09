use catppuccin::Flavour::Mocha;
use raylib::prelude::*;
use std::fs;
use std::io;

#[derive(Debug)]
struct Person {
    name: String,
    connections: Vec<serde_json::Value>,
    uid: u64,
}

fn load() -> serde_json::Value {
    // Ask for file name
    println!("Enter file name (people.json): ");

    // Get user input for file location
    let mut file_name: String = String::new();
    _ = io::stdin().read_line(&mut file_name);

    println!("Opening {}", file_name);

    // Remove last charachter
    file_name.pop();

    // Open File
    let file = fs::File::open(file_name).expect("Error Opening File!");

    // load and return JSON
    serde_json::from_reader(file).expect("Error Reading File Or Wrong Format!")
}

fn main() {
    // Define colors
    let base = Color::from_hex(&Mocha.base().hex()).unwrap();
    let sky = Color::from_hex(&Mocha.sky().hex()).unwrap();

    // Get every person
    let json_value = load();
    let json_array = json_value.as_array().unwrap();
    let length = json_array.len();

    let mut people: Vec<Person> = Vec::new();

    for object in 0..length {
        let person_as_object = &json_array[object];

        let name = person_as_object
            .get("name")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned();
        let connections = person_as_object
            .get("connections")
            .unwrap()
            .as_array()
            .unwrap()
            .to_owned();
        let uid = person_as_object.get("uid").unwrap().as_u64().unwrap();

        let person = Person {
            name: name,
            connections: connections,
            uid: uid,
        };

        people.push(person);
    }

    dbg!(people);

    // Initialize
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello World!")
        .resizable()
        .build();

    // Mainloop
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(base);
        d.draw_rectangle(0, 0, 30, 30, sky);
    }
}
