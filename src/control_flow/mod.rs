fn if_and_if_let() {
    let num = Some(22);
    if num.is_some() {
        println!("Number is {}", num.unwrap());
    }
    if let Some(i) = num {
        println!("Number is {}", i);
    }
}

fn loops() {
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break;
        }
    }
}

fn nested_loops_and_labels() {
    'outer: loop {
        'inner: loop {
            break;
            break 'outer;
        }
    }
}

fn returning_from_loop() {
    let mut counter = 0;
    let result = loop {
      counter += 1;
        if counter == 10 {
            break counter;
        }
    };
}

fn while_and_while_let() {
    let mut n = 0;
    while n < 101 {
        n += 1;
    }

    let mut optional = Some(0);
    while let Some(i) = optional {
        println!("{}", i);
        optional = None;
    }
}

fn for_loop() {
    for n in 1..101 {
        println!("{}", n);
    }
}
