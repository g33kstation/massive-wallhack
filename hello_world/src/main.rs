fn add2(x:i32, y:i32) -> i32 {
    x + y
}

fn main() {
    println!("Hello, world!");
    let x:i32 = 1; //immutable binding
    let y:i32 = 13i32; //integer suffix
    let f:f64 = 1.3f64; // float suffix

    //type inference
    let implicit_x = 1;
    let implicit_f = 1.3;

    let sum = x + y + 13;

    let mut a = 1;
    a = 5;
    a += 2;

    let x:&str = "hello world"; // string literal
    println!("{} {}", f, x);

    let s:String = "hello world".to_string();

    // just a pointer to something that does contain a string: s
    let s_slice:&str = &*s;
    println!("{}", s);

    // fixed size ints array
    let four_ints:[i32; 4] = [1,2,3,4];
    // a dynamic int array, vector array
    let mut vec:Vec<i32> = vec![1,2,3,4];
    vec.push(5);

    // just a pointer to something that does contains a vector/array
    let slice_vec:&[i32] = &*vec;

    // print debug style
    println!("{:?} {:?}", vec, slice_vec);

    // tuple: a fixed size set of values
    let d:(i32, &str, f64) = (1, "hello", 3.4);

    // destructuring let
    let (e, f, g) = d;
    println!("{} {} {}", e, f, g);
    // indexing
    println!("{}", d.1);

    // struc w/ named field
    struct Point {
        x:i32,
        y:i32,
    }
    let origin:Point = Point{x:0, y:0};

    // struc w/ unamed field: tuple struct
    struct Point2(i32, i32);
    let origin2:Point2 = Point2(0,0);

    // c like enum
    enum Direction {
        l,
        r,
        u,
        d
    }
    let up = Direction::u;


}
