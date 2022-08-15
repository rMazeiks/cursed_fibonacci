use std::fmt::Display;
use std::iter::successors;

// boring old fibonacci
pub fn print_fibonacci_imperative() {
    let (mut a, mut b) = (0, 1);
    for _ in 0..10 {
        println!("{a}");
        (a, b) = (b, a + b);
    }
}

// my personal favorite. reads like poetry
pub fn print_fibonacci_functional() {
    successors(Some((0, 1)), |(a, b)| Some((*b, b + a)))
        .map(|(a, _)| a)
        .take(10)
        .for_each(println);

    fn println<T: Display>(value: T) {
        println!("{value}");
    }
}

// do not use this or you will be fired
pub fn print_fibonacci_recursive() {
    fn compute_nth(index: usize) -> i32 {
        match index {
            0 => 0,
            1 => 1,
            _ => compute_nth(index - 2) + compute_nth(index - 1),
        }
    }

    fn print_up_to_nth(index: usize) {
        if index > 0 {
            print_up_to_nth(index - 1);
        }

        println!("{}", compute_nth(index))
    }

    print_up_to_nth(9);
}

// wait, it gets worse
// this one reads like vogon poetry
pub fn print_fibonacci_macro() {
    // this expands to a ton of 0s and 1s added together, probably computed at compile time
    macro_rules! compute_nth {
        () => (0);
        (()) => (1);
        (()()$($y:tt)*) => (compute_nth!($($y)*) + compute_nth!(()$($y)*));
    }

    // accepts a number in () unary
    macro_rules! print_up_to_nth {
        () => {};
        (()$($y:tt)*) => {
            print_up_to_nth!($($y)*);
            let value = compute_nth!($($y)*);
            println!("{value}");
        };
    }

    print_up_to_nth!(()()()()()()()()()());
}

// for those who haven't stabbed their eyes out yet
// there's another one in const_generics.rs

fn main() {
    println!("Imperative");
    print_fibonacci_imperative();

    println!("Functional");
    print_fibonacci_functional();

    println!("Recursive");
    print_fibonacci_recursive();

    println!("Macro");
    print_fibonacci_macro();
}
