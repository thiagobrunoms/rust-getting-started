//cargo --version
//cargo run
//cargo build --release

fn main() {
    let x = 10; //x is immutble - x = 20 throws error - they are immutable by default - signed as default
    let mut y = 20; //makes y explicitly mutable

    println!("Hello, world! {}", x);
    println!("Hello, world! {}", y);
    y = 30;
    println!("Hello, world! {}", y);
    y = -10;
    println!("Hello, world! {}", y);

    //unsigned integer - only positives 8bits to 128-bits - 
    //signed: i8, i16, i32... 
    //unsigned: u8, u16, u32...
    // let a: u8 = -10; // cannot apply unary operator
    let a: u16 = 1000; // cannot apply unary operator - note: the literal `1000` does not fit into the type `u8` whose range is `0..=255`
    println!("Hello, world! {}", a);
    let w: f64 = 10.1234234234234323423423;
    println!("Hello, world! {}", w);

    println!("Hello, world! {:.3}", w);
    println!("Hello, world! {:8.3}", w);
    println!("Hello, world! {:09.3}", w);
    println!("Hello, world! {:09.3}\na is {}", w, a);
    print!("Hello, world!\n {:09.3}", w);
    println!("Hello, world! Variables repeating {0:09.3}\na is {1} and we can repeat by position a = {1}", w, a);

    let mut binary_number = 0b1111_0101u8;
    println!("value is {}", binary_number);
    println!("value in bin {:08b}", binary_number);

    binary_number = !binary_number;
    println!("not value in bin {:08b}", binary_number);
    binary_number = binary_number & 0b1111_0111u8;
    println!("value & is {:08b}", binary_number);
    println!("bit 6 is {}", binary_number & 0b0100_0000); //To check the bit's value in a given location, apply AND operator with 1 at that location
    println!("bit 6 is {:08b}", binary_number | 0b0100_0000); //To check the bit's value in the given location, apply OR operator with 1 at that location
    println!("xor is {:08b}", binary_number ^ 0b1111_1101); //XOR

    let mut binary_number2 = 0b01011101;
    binary_number2 = binary_number2 << 4;
    println!("shift << by 4 {:08b}", binary_number2);
    binary_number2 = binary_number2 >> 2;
    println!("shift >> by 2 {:08b}", binary_number2);

    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average: f64 = (a as f64 + b + c as f64) / 3.0;
    assert_eq!(average, 45.1);
    println!("The average is {}", average);
}
