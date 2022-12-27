fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    another_function(x);
    ownership();
}

fn another_function( x: i32) {
    println!("The value of x is: {x}");
}

fn ownership() {

    let x = 5i32;
    let y = x;

    println!("{}, x!", x);
    println!("{}, y!", y);

    let s1 = String::from("hello");
    let s2 = s1;


    //Will cause error, contents od s1 are now ownerd by s2
    //println!("{}, world!", s1);

    let s1 = String::from("hello");
    takes_ownership(s1);            // s's value moves into the function...
                                   // ... and so is no longer valid here
    
    //println!("{}, world!", s1);     // will cause compile error
    let s1 = String::from("hello");
    takes_borrowship(&s1);

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
    println!("{}, x!", x);          // but i32 is Copy, so it's okay to still
                                    // use x afterward


} 


fn makes_copy(x:i32)  {
    println!("{}, world!", x);
    

}

fn takes_ownership(s1:String) {
    println!("{}, world!", s1);
}

fn takes_borrowship(s1:&String) {
    println!("{}, world!", s1);
}