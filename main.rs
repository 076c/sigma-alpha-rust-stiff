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

struct Pet<'a, 'b> {
    name: &'a str,
    age: u8,
    race: &'b str
}

fn what_is_pet_name<'a, 'b>(pet: &Pet<'a, 'b>) -> &'a str {
    pet.name
}

fn what_is_pet_age<'a, 'b>(pet: &Pet<'a, 'b>) -> u8 {
    pet.age
}

fn what_is_pet_race<'a, 'b>(pet: &Pet<'a, 'b>) -> &'b str {
    pet.race
}

#[allow(dead_code)]
#[allow(unused_mut)]
fn main() {
    let mut charles: Person = Person {
        name: "Charles",
        age: 30
    };

    greet_person(&charles);
    println!("{} is {} year/s old.", what_is_name(&charles), what_is_age(&charles));

    let mut james: Person = Person {
        name: "James",
        age: 26
    };

    greet_person(&james);
    println!("but {} is {} year/s old.", what_is_name(&james), what_is_age(&james));

    let mut charles_pet: Pet = Pet {
        name: "Joseph",
        age: 1,
        race: "a dog"
    };

    println!(
        "This is {}. {} is {} year/s old and is {}",
        what_is_pet_name(&charles_pet),
        what_is_pet_name(&charles_pet),
        what_is_pet_age(&charles_pet),
        what_is_pet_race(&charles_pet)
    );
}