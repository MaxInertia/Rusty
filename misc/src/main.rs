#![allow(dead_code)]

//use ::fib;

pub mod fib;
pub mod matching;
pub mod mapping;
pub mod nest;

fn main() {
    case1();
    case2();
    case3();
    mapping::test_map2();
    nest::apple()
}

fn case1() {
    fn call_function_on(f: impl Fn(i64), x: i64) {
        f(x)
    }

    // This works
    call_function_on(|x| println!("{}", x), 5);
    // This also works
    let func = |x| println!("{}", x);
    call_function_on(func, 5);
}

fn case2() {
    fn call_function_on(f: impl Fn(Vec<i64>), x: Vec<i64>) {
        f(x)
    }
    let xs: Vec<i64> = vec![1, 2];
    // This works
    call_function_on(|vs| println!("{:?}", vs), xs);
    // This works
    let xs: Vec<i64> = vec![1, 2];
    let func = |vs| println!("{:?}", vs);
    call_function_on(func, xs)
}

fn case3() {
    fn call_function_on<'a>(f: impl Fn(&'a Vec<i64>), x: &'a Vec<i64>) {
        f(x)
    }
    let xs: Vec<i64> = vec![1, 2];
    // This works
    call_function_on(|vs| println!("{:?}", &vs), &xs);
    // This works
    //let func = |&vs| println!("{:?}", vs);
    //call_function_on(func, &xs)
}

//return_break_for::scope()
//return_break_for::break_outer_loop()
//return_break_for::return_from_loop();
//return_break_for::for_and_iterators()
//fib::go()
//matching::main();

//mapping::vec();
//mapping::array();
//mapping::for_array();
//mapping::test_map();
