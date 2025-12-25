fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuples(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_structure(&rect1)
    );

    // Imprimir estructuras complejas
    println!("rect1 is {:#?}", rect1);
    println!("rect1 is {rect1:?}");

    // Imprimir depuración
    dbg!(&rect1);
    /*
        [src\main.rs:32:5] &rect1 = Rectangle {
            width: 30,
            height: 50,
        }
     */

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}

fn area(width1: u32, height1: u32) -> u32 {
    width1 * height1
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_structure(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}