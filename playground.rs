fn main() {

    let hex: u32 = 0xff;          // Hexadecimal
    
    let octal: u8 = 0o77;         // Octal
    let decimal: i32 = 42_069;    // Decimal
    let binary: u8 = 0b0001_0001; // Binary
    let byte: u8 = b'A';          // Byte (u8 only)
    
    println!("Hexadecimal: {}", hex);
    println!("Octal: {}", octal);
    println!("Decimal: {}", decimal);
    println!("Binary: {}", binary);
    println!("Byte: {}", byte);
    
    for i in 0..5 {
        println!("i: {}",i);
        
        if(2..=3).contains(&i){
            println!("wow");
        }
        
    }
    
    println!("sum: {}",(2..50).sum::<i16>());
}
