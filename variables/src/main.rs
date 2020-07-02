fn main() {
    // ----------------------------------------------------------------
    // 3.1 変数と可変性

    // constants
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // ----------------------------------------------------------------
    // 3.2 データ型

    // 整数型
    // 符号付き
    let a: i8 = 0;
    let b: i16 = 0xff; // 16進数
    let c: i32 = 0;
    let d: i64 = 0;
    let e: isize = 0;
    // 符号なし
    let f: u8 = b'A'; //バイト
    let g: u16 = 12_345; //
    let h: u32 = 0o77; // 8進数
    let i: u64 = 0b1111_0000; // 2進数
    let j: usize = 0;
    // 浮動小数点型
    let k: f32 = 0.0;
    let l: f64 = 0.0;
    // 論理値型
    let m: bool = true;
    // 文字型
    let n: char = 'a';

    // 複合型
    // タプル型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of (x, y, z) is: {}, {}, {}", x, y, z);
    println!("The value of tup is: {}, {}, {}", tup.0, tup.1, tup.2);
    // 配列型
    let array: [u8; 5] = [1, 2, 3, 4, 5];
    let element = array[0];
    println!("The value of element is: {}", element);
}
