#[allow(dead_code)]
// Sort a Vector of Integers
fn sort_vec_integers() {
    let mut vec: Vec<i32> = vec![1, 4, 5, 2, 31];
    vec.sort();
    assert_eq!(vec, vec![1, 2, 4, 5, 31]);
}
// Sort a Vector of Floats
#[allow(dead_code)]
fn sort_vec_floats() {
    let mut vec = vec![1.2, 3.4, 2.3, 5.5, 1.1112, 2.0];
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.1112, 1.2, 2.0, 2.3, 3.4, 5.5,]);
    // assert_eq!(vec, vec![1.1112, 1.2, 2.0, 2.3, 3.4, 5.3,]);
}

// Sort a Vector of Structs
// To make a Vector of Struct sortable we need four traits `Eq`, `PartialEq`, `Ord` and `PartialOrd`
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}
impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

fn main() {
    let mut people: Vec<Person> = vec![
        Person::new("Mark".to_string(), 31),
        Person::new("Noe".to_string(), 23),
        Person::new("Tomi".to_string(), 32),
    ];
    // Sort people by derived natural order (name and age)
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Mark".to_string(), 31),
            Person::new("Tomi".to_string(), 32),
            Person::new("Noe".to_string(), 23),
        ]
    );

    // Sort people by age
    people.sort_by(|a: &Person, b: &Person| b.age.cmp(&a.age));

    assert_eq!(
        people,
        vec![
            Person::new("Noe".to_string(), 23),
            Person::new("Mark".to_string(), 31),
            Person::new("Tomi".to_string(), 32),
        ]
    );
}
