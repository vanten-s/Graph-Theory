use catppuccin::Flavour::Mocha;
use rand::Rng;
use raylib::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Clone)]
#[allow(dead_code)]
struct Person {
    name: String,
    connections: Vec<serde_json::Value>,
    uid: u64,
    x: u32,
    y: u32,
}

static TEXT_SIZE: i32 = 12;

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

#[allow(unused_labels)]
fn main() {
    // Get every person
    let json_value = load();
    let json_array = json_value.get(0).unwrap().as_array().unwrap();
    let length = json_array.len();

    let mut people: Vec<Person> = Vec::new();
    let mut people_hashmap: HashMap<u64, Person> = HashMap::new();

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

        people.push(person.clone());
        people_hashmap.insert(uid, person);
    }

    // Define colors
    let base = Color::from_hex(&Mocha.base().hex()).unwrap();
    let sky = Color::from_hex(&Mocha.sky().hex()).unwrap();
    let green = Color::from_hex(&Mocha.green().hex()).unwrap();
    let red = Color::from_hex(&Mocha.red().hex()).unwrap();

    // Initialize
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello World!")
        .resizable()
        .msaa_4x()
        .build();

    let mut zoom_level: i32 = 10000;

    // Mainloop
    'mainloop: while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            zoom_level -= 1;
        } else if rl.is_key_down(KeyboardKey::KEY_UP) {
            zoom_level += 1;
        }

        let width = &rl.get_screen_width();
        let height = &rl.get_screen_height();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(base);

        'connections: for person in &people {
            let (x, y) = transform(person.x, person.y, *width, *height);

            for connection in &person.connections {
                let connected_person = people_hashmap.get(&connection.as_u64().unwrap()).unwrap();
                let (connected_x, connected_y) =
                    transform(connected_person.x, connected_person.y, *width, *height);
                d.draw_line(x, y, connected_x, connected_y, green);
            }
        }

        'circles: for person in &people {
            let (x, y) = transform(person.x, person.y, *width, *height);

            d.draw_circle(x, y, (width / 100) as f32, sky);
        }

        'names: for person in &people {
            let (x, y) = transform(person.x, person.y, *width, *height);

            let text_width = raylib::text::measure_text(&person.name, TEXT_SIZE);
            let text_offset = (text_width / 2) as i32;

            d.draw_text(&person.name, x - text_offset, y, TEXT_SIZE, red);
        }
    }
}
