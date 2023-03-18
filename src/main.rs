fn main() {
    variables_and_mutability();

    data_types();

    let param1 = 15;
    let param2 = 'L';
    functions(param1, param2);

    control_flow();

    repetition_and_loops();
}

fn repetition_and_loops(){
    loop {
        println!("again!");
        break;  // breaks out of the loop
    }

    // return value from loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };
    println!("result: [{result}]");
}

fn control_flow(){
    let number = 3;
    if number < 5{
        println!("condition was true");
    }else {
        println!("condition was true");
    }

    // if number {                      // this crashes
    //     println!("this crashes");    // this crashes
    // }                                // this crashes

    if number % 3 == 0 {
        println!("treis");
    } else if number % 2 == 0{
        println!("par igual");
    } else {
        println!("impar diferente");
    }

    let condition = true;
    let num = if condition { 5 } else if condition == false { 4 } else { 0 };
    // let num_err = if condition { 5 } else { "seis" }; // this crashes different types
    println!("condition, num: [{condition}, {num}]");
}

fn return_five() -> i32 {
    5
}

fn plus_five(x: i32) -> i32{
    x + 5
}

fn functions(value: u32, character: char){
    println!("\n functions(x) =========");
    println!("the 'main' function is the most important function in the language");
    println!("'fn' keyworkd allows you to declare new functions");
    println!("rust uses snake case as the conventional style = like_this");
    println!("parameters: [{value}, {character}]");

    let inner = {
        let x = 3;
        x + 1
    };
    println!("inner: [{inner}]");

    let five = return_five();
    println!("five: [{five}]");
    let five_plus = plus_five(10);
    println!("five_plus(10): [{five_plus}]");
}

fn data_types(){
    // // data type'fn' keyworkd allows you to declare new functionss
    println!("\ndata types =========");

    // integers
    let number: u32 = 32;           // number
    let negative: i32 = -32;        // signed number, sign = '-'
    let decimal: u32 = 98_222;      // decimal number (can use '_' to separate like dots)
    let hex: u32 = 0xff;            // hexa decimal number
    let octal: u32 = 0o77;          // octal number
    let binary: u32 = 0b1111_0000;  // binary number
    let byte: u8 = b'A';            // byte 

    println!("\nintegers ===");
    println!("number: [{number}]");
    println!("negative: [{negative}]");
    println!("decimal: [{decimal}]");
    println!("hex: [{hex}]");
    println!("octal: [{octal}]");
    println!("binary: [{binary}]");
    println!("byte: [{byte}]");

    // float 
    println!("\nfloats ===");
    let single: f32 = 10.0 / 3.0;  // f32 single-precision
    let double = 10.0 / 6.0;       // f64 double-precision (by default is f64) 

    println!("single-precision float: [{single}]");
    println!("double-precision float: [{double}]");

    println!("\nfloats ===");

    let sum = 5 + 10;               // addition
    let difference = 95.5 - 4.3;    // subtraction
    let product = 4 * 30;           // multiplication
    let quotient = 56.7 / 32.2;     // division
    let truncated = -5 / 3;         // truncated division
    let remainder = 43 % 5;         // remainder

    println!("sum + : [{sum}]");
    println!("difference - : [{difference}]");
    println!("product *: [{product}]");
    println!("quotient / : [{quotient}]");
    println!("division truncated / : [{truncated}]");
    println!("remainder % : [{remainder}]");

    println!("\nboolean ===");
    let t = true;
    let f: bool = false;    // explicit annotation

    println!("t: [{t}]");
    println!("f: [{f}]");

    println!("\nchar ===");
    let char1 = 'z';
    let char2: char = 'Z';          // explicit type annotation
    let heart_eyed_cat = '😻';  // accepts emotes

    println!("char1: [{char1}]");
    println!("char2: [{char2}]");
    println!("heart_eyed_cat: [{heart_eyed_cat}]");

    println!("\ncompound type ======");

    println!("\ntuple ===");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;    // destructuring

    // println!("tup: [{tup}]"); // this crashes different types on print
    println!("tup: [{x}, {y}, {z}]");
    println!("x: [{x}]");
    println!("y: [{y}]");
    println!("z: [{z}]");

    let t1 = tup.0;
    let t2 = tup.1;
    let t3 = tup.2;

    println!("t1 (tup.0): [{t1}]");
    println!("t2 (tup.1): [{t2}]");
    println!("t3 (tup.2): [{t3}]");

    println!("\narray ===");
    let arr = [1, 2, 3, 4, 5];
    let expl_arr: [i32; 5] = [1, 2, 3, 4, 5];
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul",
    "Aug", "Sep", "Oct", "Nov", "Dec"];
    let five_threes = [3; 5]; // array of 3's with index 5 = [3, 3, 3 ,3 ,3]

    let arr0 = arr[0];      
    let expl_arr1 = expl_arr[1];
    let months2 = months[2];
    let five_threes3 = five_threes[3];
    // let months2 = months[12]; // if try to print something like this the rust panicks

    println!("arr[0]: [{arr0}]");
    println!("expl_arr[1]: [{expl_arr1}]");
    println!("months[2]: [{months2}]");
    println!("five_threes[3]: [{five_threes3}]");
}

fn variables_and_mutability(){
    // variables and mutability
    const TST: u8 = 3;
    let mut x = 5;
    println!("value of x = {x}");
    x = 6;
    println!("value of x = {x}, and TST is = {TST}");

    // shadowing (recreating the variable with a new value)
    let x = 5;
    let x = x + 1; // can't reassign value without let when not mut

    {
        let x = x * 2; // this valut assignment don't affect outside scope
        println!("value of x in the inner scope: {x}");
    }
    println!("value of x in the outter scope: {x}");

    let spaces = "    "; // reuse variable (shadowing)
    let spaces = spaces.len(); // if usud a mutable variable will cause error
    // because the variable would change its type
    println!("spaces: {spaces}");
    
}
