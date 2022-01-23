use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Standard, Alphanumeric};

#[derive(Debug)]
// Custom type to hold our random values
struct Point {
    x: i32,
    y: i32,
}

// implmenting Distribution for our custom type Point
impl Distribution<Point> for Standard {
    // defining how sample will work with out custom type
    fn sample<R: Rng +?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn tuple_point() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();

    println!("Random Tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);

}

fn rand_pass() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric) // can also do this with a user defined set of ASCII characters
        .take(30)
        .map(char::from)
        .collect();

    println!("Pass: {}", rand_string);
}

fn main() {
    tuple_point();
    rand_pass();
}
