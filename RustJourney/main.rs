

//Rust Journey
#[warn(non_snake_case)] //Just in case you wanna avoid snake case
#[allow(unused_variables, unused_mut)] //Just in case you don't wanna use some variables
fn main() {

    //Extreme Basics

    //Hello World ^_^
    println!("Hello World"); //Prints a simple hello world using the println macro

    //Printing and placeholders ^_^
    println!("{}", 1); //Single Placeholder
    println!("{} is {}", "Rust", "awesome") ;//Multiple placeholders
    println!("{1} comes after {0}", 2, 3) ; //Multiple arguments printed on basis of position
    println!("{country} is pretty cool place to live in, although {city} is pretty {climate}", country = "Russia", city = "Moscow", climate = "cold"); //Multiple arguments which gets printed on the basis of name
    println!("Let's play with some numbers , binary of 56: {:b}, octal off 56 : {:o} and hex of 56 is {:x}", 56, 56, 56); //Conversion to certain types, just small number games
    println!("Ever thought of adding a few digits ? Here you go : 10 + 1000 = {addition}", addition = 10 + 1000); //Just a small addition and print
    println!("What if multiple values in once we can use the debug trait indeed {:?}", ("Text1", "Text2")); //Prints multiple values and prevents from writings placeholder for each
    eprintln!("Prints error"); //This is just to show the user that an error has occurred
    print!("Hello World"); //Just prints  a text without appending a new line.

    //Variables in Rust ^_^
    /*Variables on Rust have an owner associated with it*/
    /*Variables in Rust are immutable by default which means we cannot reassign them*/

    let a = 89; //this how a variable is created and initialized
    let b = 65.90 ;  //this how a variable is created and initialized
    let c = 'c' ;  //this how a variable is created and initialized
    let d = -89;  //this how a variable is created and initialized
    let language = "Rust";  //this how a variable is created and initialized
    println!("The value inside a is : {}, inside b is :{} , inside c is : {} and in d is :{}, in {}", a, b, c, d, language); //Prints all of the above

    //mutability in Rust aka just in case if you want to change in the value of the variable

    let mut specsofrust = "Safety";
    println!("A feature of Rust is {}", specsofrust); //Here it will print Safety
    specsofrust = "Speed";
    println!("Another feature of Rust is {}", specsofrust); //Here it will print Speed

    //So you wanna assign multiple variables in Rust ?  Syntax : let (variable1, variable2 , variable3, variable(n)) = (value1, value2, value3, value(n))

    let (nameoflanguage, characters, origin) = ("Rust", "Safe", "Mozilla");
    println!("The name of the programming language is {}, and it's favourite character is it's {}, and it originated from {}" ,nameoflanguage, characters, origin);

     //Scope but what's that?

    let key = 0x76;
    {
        let key_one = 0x89;
        println!("The local variable is being accessed and printed : {:x}", key_one); //Prints the local variable
    }
    //println!("{}", key_one); //This will generate error because the variable key_one is just limited to the scope or the {} brackets
    println!("The value of the global variable key is : {:x}", key); //Prints thee global variable

    //Shadowing but what's that?

    let lock_one = 98; //This value is said to be shadowed by the inner variable
    {
        let lock_two = 89;
        let lock_one= 99; //This variable is said to be mask the outer variable
        println!("The values of both the locks are as follows : lock_one : {}, lock_two : {}", lock_one, lock_two); //Prints the local variable
    }
    println!("The value of the global variable lock_one is : {}", lock_one); //Prints the global shadowed variable


//Data Types in Rust ^^

    //Scalar types : These type of data types store single data values in variable

    let a = 90; //An integer
    let b = 90.99; // A float
    let c = true ;// A boolean
    let character = '🦀'; //A character
    println!("The scalar types of variables are integer : {}, float : {}, boolean : {}, and character:{}", a, b, c, character);

   //Compound Types : These type of data types stores multiple data values in variable

    //An Array : Like other programming languages we have arrays in Rust too which are of fixed size, syntax: let nameofarray : [type ; size] = [elements];

    let characters_of_rust : [&str; 2] = ["Speed", "Safe"];
    println!("The characters of Rust are {:?}", (characters_of_rust[0], characters_of_rust[1])); //Used the debug trait because I was lazy to write placeholders :brutal:

    //Incase you want mutable arrays

    let mut programminglanguages_ilike: [&str ; 3] = ["C", "Rust", "Brainfuck"];
    println!("The programming languages I like are {:?}", (programminglanguages_ilike[0], programminglanguages_ilike[1], programminglanguages_ilike[2]));
    programminglanguages_ilike[2] = "Go";
    println!("Sometimes I like {:?} too ", programminglanguages_ilike[2]) ;//some mutable this ^^
    println!("The number of programming languages I like are {}", programminglanguages_ilike.len()); //Incase you want the length of the array just use the .len function

    //Wait what are slices ? :thinking: -> Slices are like you take some part of  the array and refer tp a subset of a contagious memory location , size of slice is known during run time

    //Syntax : let nameofslice:&[i32/i64/char/&str] = &originalarray

    let chars : [ char; 3] = ['X', 'Y', 'Z'];
    let sliceone:&[char] = &chars[0..2];
    print!("Slice of the array chars is : {:?}", sliceone);

    //Tuples: Unlike all the data types we saw above, they were all homogenous data types , this tuple data type is a heterogeneous data type meaning multiple
    //types can be included in one variable although this is a compound variable it should not be confused with arrays or slices.













}