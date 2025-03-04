struct Person<'a> {
    name: &'a str,
    age: u8
}

fn greet_person<'a>(person: &Person<'a>) {
    println!("Hello {}!", &person.name);
}

#[allow(unused_parens)]
fn what_is_age<'a>(person: &Person<'a>) -> u8 {
    person.age
}

fn what_is_name<'a>(person: &Person<'a>) -> &'a str {
    person.name
}

#[allow(dead_code)]
#[allow(unused_mut)]
fn main() {
    let mut person: Person = Person {
        name: "Charles",
        age: 30
    };

    greet_person(&person);
    println!("{} is {} years old.", what_is_name(&person), what_is_age(&person));
}