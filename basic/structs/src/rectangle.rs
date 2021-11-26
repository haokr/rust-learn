fn cal_area(width: u32, height: u32) -> u32 {
    width * height
}

fn cal_area2(rec: (u32, u32)) -> u32 {
    rec.0 * rec.1
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn cal_area3(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}
