use std::fmt::{Debug, Display};

use rand::{Rng, thread_rng};

trait Signed {
    fn is_strictly_negative(self) -> bool;
}


#[derive(Debug)]
struct Number {
    odd: bool,
    value: i32,
}

impl Signed for Number {
    fn is_strictly_negative(self) -> bool {
        self.value < 0
    }
}

impl Number {
    fn new(odd: bool, value: i32) -> Number {
        Number { odd, value }
    }

    fn isOdd(value: i32) -> bool {
        value % 2 == 1
    }

    fn is_odd(&self) -> bool {
        self.value % 2 == 1
    }
}

fn main() {
    let x: i32 = 42;

    println!("The value of x is: {}", x);

    let mut y;

    y = x;

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);

    y = y + 42;

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);

    let x = 13;
    let x = x + 3;

    println!("The value of x is: {}", x);

    let pair = ("a", 23, "b");
    println!("The value of pair is: {}", pair.0);
    println!("The value of pair is: {}", pair.1);
    println!("The value of pair is: {}", pair.2);

    let (_, y, z) = pair;
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    println!("{}", greet());

    let x = "out";
    {
        // this is a different `x`
        let x = "in";
        println!("{}", x);
    }
    println!("{}", x);

    let resolving_expression = { 4 };
    println!("{}", resolving_expression);

    let mut rng = thread_rng();
    let bigger_expression = {
        let x = 3;
        rng.gen_range(1..=5)
    };

    if (bigger_expression == 5) {
        println!("Bigger Expression: {}", bigger_expression);
    } else {
        println!("Bigger Expression is lower than 5 {}", bigger_expression);
    }

    let name = "Timothy";

    println!("Hello Timothy with this many characters, {}!", name.len());

    let mut v: Vec<i32> = Vec::new();

    v.push(3);

    println!("No Longer Empty Vector with Length, {}!", v.len());

    println!("{}!", v[0]);

    struct FirstStruct {
        x: f64,
        y: f64,
    }

    let v1 = FirstStruct { x: 1.0, y: 2.0 };

    let v2 = FirstStruct { x: 1.0, ..v1 };

    println!("Here is the v1 {}", v1.x);
    println!("Here is the v2 {}", v2.x);

    // enum FirstEnum {
    //     First = 1,
    //     Second = 2
    // };

    // println!("{}", FirstEnum::First as i32);
    // println!("{}", FirstEnum::Second as i32);

    let one = Number {
        odd: true,
        value: 1,
    };

    let two = Number {
        odd: false,
        value: 2,
    };

    // print_number(one);
    // print_number(two);

    print_number_two(one);
    print_number_two(two);

    let mut three = Number {
        odd: false,
        value: 3,
    };
    println!("is Odd? {}", &three.is_odd());

    println!("is Odd? {}", &three.is_odd());
    println!("Random whatever {:?}", &three);

    print(three);

    let x_ref = {
        let x = 42;
        x
    };

    println!("X Ref: {}", x_ref);


    let mut mutatablex = 42;
    let  x_ref_two = &mut mutatablex;

    *x_ref_two += 1;

    println!("X Ref: {}", x_ref_two);

}

fn print<T: Debug>(value: T) {
    println!("value = {:?}", value);
}

fn print_number(n: Number) {
    if n.odd {
        println!("The value is odd{}", n.value)
    } else {
        println!("The value is even{}", n.value)
    }
}

fn print_number_two(n: Number) {
    match n {
        Number { odd, value: 1 } => println!("The value is odd as one {}", odd),
        Number { odd: false, value } => println!("The value is even{}", value),
        Number { odd: true, value } => println!("The value is odd{}", value),
    }
}

fn greet() -> i32 {
    4
}
