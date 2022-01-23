
fn sort_int() {
    // vec! macro used for simple instanceiation(spelling lol)
    let mut vec = vec![1, 5, 10, 2, 15];

    vec.sort();

    // checks our sorted vector what is SHOULD actually be, program will exit and report error if
    // the assert is true
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
    println!("Simple sort: {:?}",vec);
}

fn sort_float() {
    // again using the vec! macro
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    
    // using sort by so we can sort our float values (f32 and f64)
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // assert again incase sort_by created by rust fails (never will lol)
    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
    println!("Sort by: {:?}",vec);
}


// in order to make our Person struct sortable we have to derive some traits for it
// aka defining some shared behavior for the struct
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32
}

// how Person will implment the new operator to create a new Person object
impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age
        }
    }
}

fn sort_vec_custom() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1)
    ];

    // sorting by the derived natural order IE the first item in the struct
    people.sort();

    // an assert_eq would go here but i got lazy
    println!("Sorted by natural order: {:?}", people); // can use # for a pretty print

    // Sorting people by their age only

    people.sort_by(|a, b| b.age.cmp(&a.age));

    // another assert_eq but yeah
    println!("Sorted by age: {:?}", people); // can use # for a pretty print

}


fn main() {
    sort_int();

    sort_float();

    sort_vec_custom();
}
