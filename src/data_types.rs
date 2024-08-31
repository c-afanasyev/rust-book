fn scalar_data_types() {
    // integer types
    let e_bits: u8 = 8;
    let byte_char: u8 = b'A';
    let st_bits: i16 = -16;
    let tt_bits: u32 = 32;
    let sf_bits: i64 = -64;
    let hte_bits: u128 = 128;
    let a_bits: isize = 64;

    // floating-point types
    let double_precision = 64.0;
    let single_precision: f32 = 32.0;

    // boolean type
    let b = true;
    let f: bool = false;

    // character type
    let a = 'a'; // 32 bits
    let b: char = 'B'; // 32 bits
    let emoji = '😺'; // 32 bits

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}

fn compound_data_types() {
    // tuples
    let tup_explicit: (i32, f64, u8) = (500, 6.4, 1);
    let tup_implicit = (500, 6.4, 1);
    let (x, y, z) = tup_explicit;
    println!("x: {x}, y: {y}, z: {z}");

    let five_hundred = tup_implicit.0;
    let six_point_four = tup_implicit.1;
    let one = tup_implicit.2;

    let unit = (); // empty tuple is called 'unit'

    // arrays
    let arr_implicit = [1,2,3,4,5];
    let arr_explicit: [i32;5] = [1,2,3,4,5];
    let all_zero_implicit = [0;5];
    let all_zero_explicit: [u8;5] = [0;5];

    let first = arr_implicit[0];
    let second = arr_explicit[1];
}