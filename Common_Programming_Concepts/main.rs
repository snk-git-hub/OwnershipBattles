
fn main (){
    /*immutable variables*/

    // let mut x=5; // mut makes the variable x mutable
    // println!("value of x is {x}");
    // x=6;
    // println!("{x}");

    /*Constants */
    // ⚠️constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.
    // const THREE_HOURS_IN_SECONDS: u16 = 60 * 60 * 3;
    // println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
    

//     | Type    | Size              | Signed | Range                           |
// | ------- | ----------------- | ------ | ------------------------------- |
// | `i8`    | 8-bit             | Yes    | -128 to 127                     |
// | `u8`    | 8-bit             | No     | 0 to 255                        |
// | `i16`   | 16-bit            | Yes    | -32,768 to 32,767               |
// | `u16`   | 16-bit            | No     | 0 to 65,535                     |
// | `i32`   | 32-bit            | Yes    | -2,147,483,648 to 2,147,483,647 |
// | `u32`   | 32-bit            | No     | 0 to 4,294,967,295              |
// | `i64`   | 64-bit            | Yes    | Very large negative to positive |
// | `u64`   | 64-bit            | No     | Very large                      |
// | `i128`  | 128-bit           | Yes    | Even larger                     |
// | `u128`  | 128-bit           | No     | Even larger                     |
// | `isize` | depends on system | Yes    | For indexing; 32 or 64-bit      |
// | `usize` | depends on system | No     | For indexing; 32 or 64-bit   

// Floating-Point Types
// Type	Size	Description
// f32	32-bit	Single precision float
// f64	64-bit	Double precision float (default)|


/*Shadowing */

// let x=5;
// let x=x+1;
// {
//     let x=x*2;
//     println!("{x}");
// }
// println!("{x}")

// let  _a : u32 = "5".parse().expect("not a number");

// println!("{_a}")



// Length	Signed	Unsigned
// 8-bit	i8	u8
// 16-bit	i16	u16
// 32-bit	i32	u32
// 64-bit	i64	u64
// 128-bit	i128	u128
// arch	isize	usize

//
// Number literals	Example
// Decimal	98_222
// Hex	0xff
// Octal	0o77
// Binary	0b1111_0000
// Byte (u8 only)	b'A'

//etc/...

//The tuple type
// let tup:(i32,f64,u8)=(500,6.4,1);

// println!("{}",tup.1)
//////or////
// let tup=(500,6.4,1);
// let(x,y,z)=tup;
// println!("{x},{y},{z}")

}





