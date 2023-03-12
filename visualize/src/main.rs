use catppuccin::Flavour::Mocha;
use rand::Rng;
use raylib::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Person {
    name: String,
    connections: Vec<serde_json::Value>,
    uid: u64,
    x: f64,
    y: f64,
}

static TEXT_SIZE: i32 = 24;

fn distance_squared(person1: &Person, person2: &Person) -> f64 {
    let x_squared = (person1.x - person2.x) * (person1.x - person2.x);
    let y_squared = (person1.y - person2.y) * (person1.y - person2.y);

    return x_squared + y_squared;
}

fn timestep(people: Vec<Person>, hasmap: HashMap<u64, Person>) -> Vec<Person> {

    let mut people_return_array: Vec<Person> = Vec::new();

    for person in &people {
        // Move away from everyone
        
        let mut person_new = person.clone();

        'inner: for other_person in &people {

            if person.name == other_person.name {
                continue 'inner;
            }
            
            let distance = distance_squared(&person, &other_person);
            let direction_x = (person.x - other_person.x) / distance.sqrt();
            let direction_y = (person.y - other_person.y) / distance.sqrt();
            println!("{}", person.name);
            println!("{}", other_person.name);
            println!("{}", direction_x);
            println!("{}", direction_y);
            println!("\n");

            person_new.x += 5.0 * direction_x / distance;
            person_new.y += 5.0 * direction_y / distance;
            
        }

        for connection in &person.connections {
            let other_person = hasmap.get(&connection
                                          .as_u64()
                                          .unwrap()).unwrap();
            let distance = distance_squared(&person, other_person);
            let direction_x = (person.x - other_person.x) / distance.sqrt();
            let direction_y = (person.y - other_person.y) / distance.sqrt();
            println!("{}", person.name);
            println!("{}", other_person.name);
            println!("{}", direction_x);
            println!("{}", direction_y);
            println!("\n");

            person_new.x -= 30.0 * direction_x / (distance.sqrt() - 500.0).abs();
            person_new.y -= 30.0 * direction_y / (distance.sqrt() - 500.0).abs();
        }

        people_return_array.push(person_new);

    }
    return people_return_array;

}

fn make_hasmap(people: &Vec<Person>) -> HashMap<u64, Person> {
    let mut hashmap: HashMap<u64, Person> = HashMap::new();

    for person in people {
        hashmap.insert(person.uid, person.clone());
    }

    return hashmap;
}

fn transform(x: u32, y: u32, width: i32, height: i32, zoom_level: i32, added_x: i32, added_y: i32) -> (i32, i32) {
    let zoom_transform: f64 = (zoom_level as f64) / 10000.0;

    let width_mul = zoom_transform * width as f64 / 10.0;
    let height_mul = zoom_transform * height as f64 / 10.0;

    let transformed_x = (x as f64) * width_mul + (added_x as f64);
    let transformed_y = (y as f64) * height_mul + (added_y as f64);

    return (transformed_x as i32, transformed_y as i32);
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
            x: rng.gen_range(0..100) as f64,
            y: rng.gen_range(0..100) as f64,
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

    rl.set_target_fps(60);

    let mut zoom_level: i32 = 100;

    let mut added_x: i32 = 0;
    let mut added_y: i32 = 0;

    // Mainloop
    'mainloop: while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_D) {
            added_x -= 1;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            added_x += 1;
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            added_y += 1;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            added_y -= 1;
        }

        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            zoom_level -= 1;
        } 
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            zoom_level += 1;
        }

        let width = &rl.get_screen_width();
        let height = &rl.get_screen_height();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(base);

        people = timestep(people.clone(), people_hashmap);
        people_hashmap = make_hasmap(&people);

        'connections: for person in &people {
            let (x, y) = transform(person.x as u32, person.y as u32, *width, *height, zoom_level, added_x, added_y);

            for connection in &person.connections {
                let connected_person = people_hashmap.get(&connection.as_u64().unwrap()).unwrap();
                let (connected_x, connected_y) =
                    transform(connected_person.x as u32, connected_person.y as u32, *width, *height, zoom_level, added_x, added_y);
                d.draw_line(x, y, connected_x, connected_y, green);
            }
        }

        'circles: for person in &people {
            let (x, y) = transform(person.x as u32, person.y as u32, *width, *height, zoom_level, added_x, added_y);
            d.draw_circle(x, y, (zoom_level / 100) as f32, sky);
        }

        'names: for person in &people {
            let (x, y) = transform(person.x as u32, person.y as u32, *width, *height, zoom_level, added_x, added_y);

            let text_width = raylib::text::measure_text(&person.name, TEXT_SIZE);
            let text_offset = (text_width / 2) as i32;

            d.draw_text(&person.name, x - text_offset, y + (zoom_level / 100), TEXT_SIZE, red);
        }
    }
}
