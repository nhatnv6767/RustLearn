struct Person {
    name: String,
    fav_color: String,
    age: i32,
}

fn print(name: &str, fav_color: &str) {
    println!("{}: {}", name, fav_color);
}

fn main() {

    let people = vec![
        Person {
            name: String::from("George"),
            fav_color: String::from("green"),
            age: 7,
        },

        Person {
            name: String::from("NhatNguyen"),
            fav_color: String::from("blue"),
            age: 9,
        },

        Person {
            name: String::from("Balon"),
            fav_color: String::from("Yellow"),
            age: 15,
        },
    ];

    for person in people {
        if person.age <= 10 {
            print(&person.name, &person.fav_color);
        }
    }
}