fn boolean() {
    let b: bool = true;
}

fn unsigned_integers() {
    let u8: u8 = 0;
    let u16: u16 = 0;
    let u32 = 0;
    let u64: u64 = 0;
    let u128: u128 = 0;
}

fn signed_integers() {
    let i8 = 0i8;
    let i16 = 0i16;
    let i32 = 0;
    let i64 = 0i64;
    let i128 = 0i128;
}

fn floating_point_numbers() {
    let f32 = 0.0f32;
    let f64 = 0.;
}

fn platform_specific_integers() {
    let isize = 0isize;
    let usize = 0usize;
}

fn string() {
    let char = 'c'; // unicode value
    let str = "&str"; // string slice
    let string = "String"; // owned string
}

fn tuple() {
    let tuple = ("Team A", 7, 0.5);
}

fn array() {
    // массив имеет фиксированный размер и тип элементов
    let array = [1, 2, 3];

    // размер слайса может меняться во время выполнения
    let slice = &array[..];
}

use std::collections::HashMap;
fn hash_map() {
    let mut hash_map = HashMap::new();
    hash_map.insert("Key".to_string(), 5);
    // перезаписывает значение по ключу или добавляет новый ключ с таким значением
    hash_map.entry("Key".to_string()).or_insert(3);

    println!("{:?}", hash_map);
}

fn structure() {
    struct User {
        name: String,
        active: bool
    }

    let user = User {
        name: "John".to_string(),
        active: true
    };

    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
}

fn enumeration() {
    enum Command {
        Quit,
        Move { x: i32, y: i32 },
        Speak(String),
        Color(i32, i32, i32)
    }

    let cmd = Command::Quit;
    let cmd = Command::Move { x: 1, y: 2 };
    let cmd = Command::Speak("Hi".to_string());
    let cmd = Command::Color(0, 0, 0);
}

fn constant() {
    // константы хранятся в коде
    const MAX_POINTS: u32 = 100_000;
}

fn static_variables() {
    // статические переменные хранятся в выделенной памяти и могут быть изменены
    static VERSION: u32 = 1;
    static mut COUNTER: u32 = 0;

}

fn type_alias() {
    type Int = i32;
}
