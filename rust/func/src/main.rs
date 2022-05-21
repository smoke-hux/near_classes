fn main() {
    println!("Hello, world!");
    let x = my_function_2(22);
    let b = my_function_3();
    let v = my_function_4();
}

fn my_function_2(number: i32){
    println!("Hello my function my number {}", number);
}

fn my_function_3()-> String{
    println!("Hello my function my number 3");
    return String:: from("Hello world");
}

fn my_function_4()-> i32{
    println!("Hello my function my number 4");
    33
}

fn my_function_5()->(i32, String){
    println!("Hello my function my number 4 ");
    (33, String:: from("hey"))
}


// Arrays the size is known 
// the double .. dots is used to define the range of a given number like (1 ..4 )
/*
vectors is a powerfull data type because we will be using it along
Vec is the keyword ,is used when declearing a vector and it is generic
is is a generic , they can change themselves partly pokymophism

in rust we use two semicollons when you want to call a function inside a module unlike other languages where you use . like const.function to be called 

*/