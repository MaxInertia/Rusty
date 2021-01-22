
pub fn scope() {
    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // This binding *shadows* the outer one
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }
    // End of the block

    // Error! `short_lived_binding` doesn't exist in this scope
    //println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_binding);

    // This binding also *shadows* the previous binding
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}

pub fn break_outer_loop() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        //println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

pub fn return_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

pub fn ranges() {
    let range1 = 1..101;
    let range2 = 1..=100;

    assert!(range1.contains(&1));
    assert!(range1.contains(&100));
    assert!(!range1.contains(&101));
    assert!(range2.contains(&1));
    assert!(range2.contains(&100));
    assert!(!range2.contains(&101));
}

pub fn for_and_iterators() {
    // iter: borrows each element of the collection through each iteration. Collection left untouched
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // iter_mut: borrows each element of the collection, allowing for the collection to be modified in place.
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        match name {
            &mut "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // into_iter: consumes the collection so that on each iteration the exact data is provided. Collection is consumed
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    //println!("{}", names[2]);
}