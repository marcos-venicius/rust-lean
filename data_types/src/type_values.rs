pub fn new() {
    println!("TYPES AND YOUR MAX & MIN VALUES\n");

    println!("UNSIGNED");

    let u8_min: u8 = u8::MIN;
    let u8_max: u8 = u8::MAX;

    println!("u8:    {u8_min} -> {u8_max}");

    let u16_min: u16 = u16::MIN;
    let u16_max: u16 = u16::MAX;

    println!("u16:   {u16_min} -> {u16_max}");

    let u32_min: u32 = u32::MIN;
    let u32_max: u32 = u32::MAX;

    println!("u32:   {u32_min} -> {u32_max}");

    let u64_min: u64 = u64::MIN;
    let u64_max: u64 = u64::MAX;

    println!("u64:   {u64_min} -> {u64_max}");

    let u128_min: u128 = u128::MIN;
    let u128_max: u128 = u128::MAX;

    println!("u128:  {u128_min} -> {u128_max}");

    let usize_min: usize = usize::MIN;
    let usize_max: usize = usize::MAX;

    println!("usize: {usize_min} -> {usize_max}   | based on arch");

    println!("\nSIGNED");

    let i8_min: i8 = i8::MIN;
    let i8_max: i8 = i8::MAX;

    println!("i8:    {i8_min} -> {i8_max}");

    let i16_min: i16 = i16::MIN;
    let i16_max: i16 = i16::MAX;

    println!("i16:   {i16_min} -> {i16_max}");

    let i32_min: i32 = i32::MIN;
    let i32_max: i32 = i32::MAX;

    println!("i32:   {i32_min} -> {i32_max}");

    let i64_min: i64 = i64::MIN;
    let i64_max: i64 = i64::MAX;

    println!("i64:   {i64_min} -> {i64_max}");

    let i128_min: i128 = i128::MIN;
    let i128_max: i128 = i128::MAX;

    println!("i128:  {i128_min} -> {i128_max}");

    let isize_min: isize = isize::MIN;
    let isize_max: isize = isize::MAX;

    println!("isize: {isize_min} -> {isize_max}   | based on arch");
}
