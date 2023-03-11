use catppuccin::Flavour::Mocha;
use rand::Rng;
use raylib::prelude::*;
use std::fs;
use std::io;

#[derive(Debug)]
struct Person {
    name: String,
    connections: Vec<serde_json::Value>,
    uid: u64,
    x: u32,
    y: u32,
}

fn transform(x: u32, y: u32, width: i32, height: i32) -> (i32, i32) {
    let width_mul = (width as f32) / 100.0;
    let height_mul = (height as f32) / 100.0;

    let transformed_x = (x as i32) * (width_mul as i32);
    let transformed_y = (y as i32) * (height_mul as i32);

    return (transformed_x, transformed_y);
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

        let mut rng = rand::thread_rng();

        let person = Person {
            name,
            connections,
            uid,
            x: rng.gen_range(0..100),
            y: rng.gen_range(0..100),
        };

        people.push(person);
    }

    // Define colors
    let base = Color::from_hex(&Mocha.base().hex()).unwrap();
    let sky = Color::from_hex(&Mocha.sky().hex()).unwrap();
    let green = Color::from_hex(&Mocha.green().hex()).unwrap();

    // Initialize
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello World!")
        .resizable()
        .build();

    // Mainloop
    while !rl.window_should_close() {
        let width = &rl.get_screen_width();
        let height = &rl.get_screen_height();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(base);

        for person in &people {
            let (x, y) = transform(person.x, person.y, *width, *height);

            d.draw_circle(x, y, 10.0, sky);

            // Terrible performace, can be hugely improved using hasmaps for data structure
            for connection in &person.connections {
                for possible_connected_person in &people {
                    if possible_connected_person.uid == connection.as_u64().unwrap() {
                        let connected_person = possible_connected_person;
                        let (connected_x, connected_y) =
                            transform(connected_person.x, connected_person.y, *width, *height);
                        d.draw_line(x, y, connected_x, connected_y, green);
                    }
                }
            }
        }
    }
}
