use std::*;

fn main() {

    //Arrays
    let mut array = [1, 2, 3];
    array[0] = 10;
    
    //Vectors
        //Vec macro is the easiest way to make a vector
    let mut nums = vec![1, 2, 3];
    
        //Vectors are resizeable
    nums.push(4);

        //use {:?} to print a vector
    println!("{:?}", nums);

        //remove values with pop()
    nums.pop(); //removes last variable of the vector
    println!("{:?}", nums);

        //create a vector with Vec::new();
    let mut vec = Vec::new(); //equivilant to vec!
    vec.push("Test");
    vec.push("String");
    println!("{:?}", vec);

    vec.reverse(); // Reverse order of vector
    println!("{:?}", vec);

        //Specify size of vector
    let mut vect = Vec::<i32>::with_capacity(2);
    println!("{}", vect.capacity());

        //Iterate vectors
    let v: Vec<i32> = (0..5).collect(); //Creates a vector with values 0 to 4
    println!("{:?}", v);

    //Slices
    let sv: &[i32] = &v[2..4]; //make sv = vector elements 2 to 4
    println!("{:?}", sv);

    //Strings
        //These two are the same
    let name = String::from("Tyler");
    let course = "Rust".to_string();

        //Modify string
    let newName = name.replace("Tyler", "Ty");
    
    println!("{} {} {}", name, course, newName);

        // &str = "String slice" or "stir"
    let str1 = "hello"; //&str
    let str2 = str1.to_string(); //convert string slice to string
    let str3 = &str2; //make str2 a string slice
    
    println!("{} {} {}", str1, str2, str3);

        //compare string with == and !=
    println!("{}", "ONE".to_lowercase() == "one"); //will print true
    
        //String literals
    let rust = "\x52\x75\x73\x74"; 
    println!("{}", rust); //prints "Rust"
    
    //Functions (Dont put functions in other functions)
        //Use snake casing for functions snake_casing

    print_phase("Print arg"); //Call function optional with args if needed

    fn print_phase(phrase: &str) {  //put args inbetween perenthesis
        println!("{}", phrase);
    }
    
        //Returning values
    println!("{}", return_values(20, 5));

    fn return_values(mut a: i32, mut b: i32) -> i32{

        while a != 0 {
            if a < b {
                let c = a;
                a = b;
                b = c;
            }

            a = a % b;
        }
        b // return value by putting just the value without ;
    }

    //Control flow
    let one = 10;

    if one > 10 {
        println!("True");
    } else if one == 10 {
        println!("Equal");
    } else {
        println!("False");
    }

    //Loops
        //Loop keyword creates infinite repeating loop
    // loop { 
    //   println!("Loop!");
    // }
    
    let mut num = 0;
    'counter: loop {
        println!("Count: {}", num);
        let mut decrease = 5;
        loop {
            println!("Decreasing {}", decrease);
            if decrease == 4 {
                break; // leave current loop, not loop counter
            }
            if num == 2 {
                break 'counter;
            }
            decrease -= 1;
        }
        num += 1;
    }

        //While loops
    let mut num1 = 0;
    while num1 < 5 {
        println!("{}", num1);
        num1 += 1;
    }
        //For loops

    let vec: Vec<i8> = (0..10).collect();

    for element in vec {
        println!("{}", element);
    }
}
