mod struct_rect;

fn main() {
    let rect1 = struct_rect::Rect{
        length: 20,
        width: 10
    };

    let area: u32 = rect1.area();
    let perimeter = rect1.perimeter();
    println!("Area of the rectangle is {}", area);
    println!("Perimeter of the rectangle is {}", perimeter);
}