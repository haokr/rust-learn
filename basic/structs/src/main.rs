include!("define.rs");
include!("rectangle.rs");
include!("methods.rs");

fn main() {
    let rec1 = Rectangle2 {
	width: 30,
	height: 40,
    };

    println!("The area of the rectangle is {} square pixels.", rec1.cal_area());

    let rec2 = Rectangle2::square(10);
    println!("The square rectangle is {:#?}", rec2);
}

fn main1() {
    let width = 30;
    let height = 20;
    println!("The area of the rectangle is {} square pixels.", cal_area(width, height));

    let rec1 = (30, 59);
    println!("The area of the rectangle is {} square pixels.", cal_area2(rec1));

    let rec2 = Rectangle {
	width: 40,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", cal_area3(&rec2));
}

