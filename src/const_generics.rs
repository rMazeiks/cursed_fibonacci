// extra cursed. requires nightly.

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn print_fibonacci_const_generics() {
    trait ComputeFibonacci<const INDEX: i32> {
        const RESULT: i32;
    }

    impl ComputeFibonacci<0> for () {
        const RESULT: i32 = 0;
    }

    impl ComputeFibonacci<1> for () {
        const RESULT: i32 = 1;
    }

    // https://internals.rust-lang.org/t/const-generics-where-restrictions/12742/6
    struct If<const B: bool>;
    trait True {}
    impl True for If<true> {}

    impl<const INDEX: i32> ComputeFibonacci<INDEX> for ()
    where
        // constraint order matters lmao
        If<{ INDEX > 1 }>: True,
        (): ComputeFibonacci<{ INDEX - 1 }>,
        (): ComputeFibonacci<{ INDEX - 2 }>,
    {
        const RESULT: i32 = <() as ComputeFibonacci<{ INDEX - 1 }>>::RESULT
            + <() as ComputeFibonacci<{ INDEX - 2 }>>::RESULT;
    }

    // todo: i want to put this stuff in a &'static str at build time, and only have one print call
    // there is https://docs.rs/const_format/latest/const_format/ whicm makes me think this is possible
    // but i dont want a dependency
    println!("{}", <() as ComputeFibonacci<0>>::RESULT);
    println!("{}", <() as ComputeFibonacci<1>>::RESULT);
    println!("{}", <() as ComputeFibonacci<2>>::RESULT);
    println!("{}", <() as ComputeFibonacci<3>>::RESULT);
    println!("{}", <() as ComputeFibonacci<4>>::RESULT);
    println!("{}", <() as ComputeFibonacci<5>>::RESULT);
    println!("{}", <() as ComputeFibonacci<6>>::RESULT);
    println!("{}", <() as ComputeFibonacci<7>>::RESULT);
    println!("{}", <() as ComputeFibonacci<8>>::RESULT);
    println!("{}", <() as ComputeFibonacci<9>>::RESULT);
}

fn main() {
    println!("Const Generics");
    print_fibonacci_const_generics();
}
