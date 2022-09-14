use std::io;


fn main() {




    print!("--------- Floating Types ----------- \n");

    let x = 2.0; // f64
    print!("floating 64 : , {x}\n");

    let y: f32 = 3.0; // f32
    print!("floating 32 : , {y} \n");

    print!("-------- Numeric Operations ----------- \n");
    // addition
    let sum = 5 + 10;
    println!("Sum :,{sum}");
    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference :,{difference}");

    // multiplication
    let product = 4 * 30;
    println!("product :,{product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient :,{quotient}");

    let floored = 2 / 3; // Results in 0
    println!("floored :,{floored}");

    // remainder
    let remainder = 43 % 5;
    println!("ramainder :,{remainder}");

    println!("-------- Bolean type --------");

    let t = true;
    print!("true : {t} \n");

    let f: bool = false; // with explicit type annotation
    print!("false : {t} \n");

    print!("-------- Character Types --------\n");
    
    let c = 'z';
    print!("value of c :, {c} \n");
    
    let z: char = 'ℤ'; // with explicit type annotation
    print!("value of z :, {z} \n");    
    
    let heart_eyed_cat = '😻';
    print!("heart eyed cat :, {heart_eyed_cat} \n");

}