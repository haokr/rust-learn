fn main() {
    let is_unit1 = no_pi();
    let is_unit2 = {
        pi();
    };

    println!("is_unit1: {:?}, is_unit2: {:?}", is_unit1, is_unit2);
}

fn no_pi() {
    3.14;
}

fn pi() -> f32 {
    3.14
}