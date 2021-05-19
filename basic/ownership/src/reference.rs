fn mut_reference() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
}

fn reference_2() {
    let mut s = String::from("hello");
//    let r1 = &s;
//    let r2 = &mut s;

//    println!("{}, {}, {}", s, r1, r2);
}

fn reference_3() {
    let mut s = String::from("hello");
//    let r1 = &mut s;
//    let r2 = &mut s;

//    println!("{}, {}", r1, r2);
}

fn reference_4() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
}

