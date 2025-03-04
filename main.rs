struct Person<'a> {
    name: &'a str,
    age: u8
}

fn greet_person<'a>(person: Person<'a>) {
    println!("Hello {}!", &person.name);
}

#[allow(dead_code)]
fn main() {
    let mut person: Person = Person {
        name: "Charles",
        age: 30
    };

    greet_person(person);
}