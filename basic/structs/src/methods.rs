#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32,
}

impl Rectangle2 {
    fn cal_area(&self) -> u32 {
	self.width * self.height
    }

    fn square(size: u32) -> Rectangle2 {
        Rectangle2 {
            width: size,
            height: size
        }
    }
}
