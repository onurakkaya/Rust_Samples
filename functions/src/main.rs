fn main() {
    let num1 = 10;
    let num2 = 20;
    let sum_of_values =  sum(num1,num2); // Calling function with num1,num2 params.
    println!("Sum of Values are {}",sum_of_values);
    void_function();
    println!("{}es! It worked.",function());
}

// i32 means Integer (32 bit) 
// x and y are input parameters 
//( "-> i32" ) is return type of function
fn sum(x:i32,y:i32) -> i32  
{
    x + y  // this statement hasn't semicolon.
    // it means this value will be return by the function.
}

fn void_function()
{
    println!("This is Void function.");
}

fn function() -> char
{
    'Y'
}