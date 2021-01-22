fn add_n(n :i32) -> impl FnMut(&i32) -> i32 {
    move |&x: &i32| n + x
}

/*
fn add_n<T: Add>(n :T) -> impl FnMut(&T) -> T {
    move |&x: &T| n + x
}
*/

pub fn vec() {
    println!("\nmapping::vec()");
    // `iter.map.collect` to map into new collection
    let xs: Vec<i32> = vec![1, 2, 3];
    let ys: Vec<i32> = xs.iter().map(add_n(1)).collect();
    println!("xs = {:?}", xs);
    println!("xs.iter().map(...).collect() = {:?}", ys);

    // `into_iter.map.collect` to map and move
    let ys: Vec<i32> = xs.into_iter().map(|x| add_n(1)(&x)).collect();
    println!("xs.iter().map(...).collect() = {:?}", ys);
    //println!("xs = {:?}", xs); // This fails because xs elements were moved into zs

    // `iter_mut.map` to map 'in place'
    let mut xs: Vec<i32> = vec![1, 2, 3];
    xs = xs.iter_mut().map(|&mut x| &x + 1).collect();
    println!("xs.iter_mut().map = {:?}", xs);

    /*
    let mut xs: Vec<i32> = vec![1, 2, 3];
    println!("xs before = {:?}", xs);
    xs.map(|x| x + 1);
    println!("xs after = {:?}", xs);*/
}

pub fn array() {
    println!("\nmapping::array()");

    let xs: [i32; 3] = [1, 2, 3];
    let ys: Vec<i32> = xs.iter().map(add_n(1)).collect();
    println!("xs = {:?}", xs);
    println!("ys = {:?}", ys);


    //let zs: &[i32] = xs.iter().map(add_n(1)).collect();
    // help: the trait `std::iter::FromIterator<i32>` is not implemented for `&[i32]`

    let mut xs: [i32; 3] = [1, 2, 3];
    let zs: Vec<i32> = xs.iter_mut().map(|&mut x| x+1).collect();
    println!("xs = {:?}", xs);
    println!("zs = {:?}", zs);
}

pub fn for_array() {
    // mutate in place with for loop
    let mut xs: [i32; 3] = [1, 2, 3];
    println!("xs before = {:?}", xs);
    for x in &mut xs {
        *x = *x + 1;
    }
    println!("xs after = {:?}", xs);

    /*
    let mut xs: [i32; 3] = [1, 2, 3];
    println!("xs before = {:?}", xs);
    map_in_place(xs, |x| x + 1);
    //let zs: Vec<i32> = xs.iter_mut().map(|&mut x| x + 1).collect();
    println!("xs after = {:?}", xs);
    */
}

//

pub fn map_in_place<T>(xs: &mut Vec<T>, f: impl Fn(&T) -> T) {
    for x in xs.iter_mut() {
        *x = f(x);
    }
}

fn test_map_in_place() {
    // mutate in place with map
    let mut xs = vec![1, 2, 3];
    println!("xs before = {:?}", xs);
    map_in_place(&mut xs, |&x| x + 1);
    //let zs: Vec<i32> = xs.iter_mut().map(|&mut x| x + 1).collect();
    println!("xs after = {:?}", xs);
}

//

pub fn map<T, R>(xs: &Vec<T>, f: impl Fn(&T) -> R) -> Vec<R> {
    let mut ys = Vec::new();
    for x in xs.iter() {
        ys.push(f(x));
    }
    return ys
}

pub fn test_map() {
    let xs = vec![2, 6, 8];
    println!("xs = {:?}", xs);
    let ys = map(&xs, |&x| x + 1);
    println!("map({:?}, |&x| x+1) == {:?}", xs, ys);
}

pub fn test_map2() {
    let xs = vec![2, 6, 8];
    println!("xs = {:?}", xs);
    let func: for<'r> fn(&'r i64) -> i64 = |&x| x+1;
    let zs = map(&xs, func);
    //println!("let func = |&x| x+1;");
    //println!("map({:?}, |&x| x+1) == {:?}", xs, ys);
}

/* // Attempting to add lifetimes
fn map<'a, T, R>(xs: &Vec<T>, f: &'a impl Fn(&T) -> R) -> Vec<R> {
    let mut ys = Vec::new();
    for x in xs.iter() {
        ys.push(f(x));
    }
    return ys
}

fn test_map() {
    println!("map 2");
    let xs = vec![2, 6, 8];
    println!("xs = {:?}", xs);
    let func = &|&x| x+1;
    let ys = map(&xs, &func);
    println!("map({:?}, |&x| x+1) = {:?}", xs, ys);
}
*/