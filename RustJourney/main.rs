use std::os::raw::c_float;

//Rust Journey
//https://mkaz.blog/working-with-rust/numbers/
#[warn(non_snake_case)] //Just in case you wanna avoid snake case
#[allow(unused_variables, unused_mut)] //Just in case you don't wanna use some variables
const GLOBALCONSTANT: i32 = 78; //Some constant this ^^ Syntax : const nameofvariable: type  value;
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
    println!("Slice of the array chars is : {:?}", sliceone);

    //Tuples: Unlike all the data types we saw above, they were all homogenous data types , this tuple data type is a heterogeneous data type meaning multiple
    //types can be included in one variable although this is a compound variable it should not be confused with arrays or slices.

     //Syntax : let tuplename = (value1, value2, value3) ;//implicit type
    //Syntax : let tuplename : ( &type1, &type2, &type3, &type(n)) = ("value1", value2,value3);


    let tupletest = ("Rust", "A+", "Safe", "7 July 2010");
    println!("I would rate {} as {}, as it is {} also {} was introduced on {}", tupletest.0, tupletest.1, tupletest.2, tupletest.0, tupletest.3); //Values of the Rust and accessing it and printing them.

    //Incase you want a mutable tuple ^^

    let mut tuplenottest = ("ElementalX", 20, "Reverse Engineering");
    println!("{} who is {} likes {}",tuplenottest.0, tuplenottest.1, tuplenottest.2);
    tuplenottest.2 = "Programming"; //reassigning the member of the tuple
    println!("Sometimes {} also like {} although his codes are buggy af.", tuplenottest.0, tuplenottest.2);

    //Another way of accessing the values of a tuple is to destructure the tuple values indeed ^^

    let tuple_to_pass_to_variables = ("Yung Innanet", "YT Cracker", "0xdade", "paranoid", "ATMOS");
    let (y, x, z, l, p) = tuple_to_pass_to_variables; //new owners of the values are these variables
    println!("Shouts to these cool people who make music : {}, {} ,{} ,{} ,{}.", y, x, z, l, p); //Printing them to check out if these things work or not ^^


    //Some constants now ^^

    //Constants are not mutable although after using the mut keyword, constants are declared in global and local scope

    println!("The value of the global constant is {}", GLOBALCONSTANT);
    const LOCALCONST: i64 = 76;
    println!("The value of the local constant is {}", LOCALCONST);


    // Operators : Now some operations and stuffs

    //Operations can be divided in two types i) Unary ii) Binary
    //Unary : These operations act on single operands
       /*
       -> Borrow
       -> Dereference
       -> Negation
       -> Logical Negation
        */

    //Binary : These operations act on two operands



    //Let's use some arithmetic operators ^_^
let number1:i64 = 99;
let number2:i64 = 98;
    println!("The sum of number1 & number 2 is : {addition}, the difference between both of them is : {substraction}, the multiplication between them is : {multiplication}, the division between them is : {division}", addition =number1+number2,substraction = number1 - number2, multiplication= number1 * number2, division = number1 /number2 ); //Performs addition , Performs subtraction. Performs Multiplication, Performs Division

//Let's  do logical operations

    let value1:bool = true;
    let value2:bool = false;

    println!("If AND is used between two values then it evaluates to : {}", value1 && value2); //AND Operation
    println!("If OR is used between two values then it evaluates to : {}", value1 || value2); //OR Operation
    println!("If NOT is used on value1 then it evaluates to {},If NOT is used on value2 then it evaluates to {}", !value1, !value2); //NOR Operation


//Comparison Operation

    let comparevalue1:i64 = 8;
    let mut comparevalue2:i64 = 7;

    println!("Compare Value 1 is greater than comparevalue2 : {}", comparevalue1> comparevalue2);
    println!("Compare Value 2 is less than comparevalue1 : {}", comparevalue2 < comparevalue1);
    comparevalue2 = 8;
    println!("Compare Value 2 is greater than or equal to compare value1 : {}" ,comparevalue2 >= comparevalue1);
    println!("Compare Value 2 is  equal to one : {}", comparevalue2 == comparevalue1);
    comparevalue2 = 9;
    println!("Compare Value 2 is not equal to one : {}", comparevalue2 != comparevalue1);

// Let's do some bitwise operations ^-^

    let bitwiseone = 77;
    let bitwisetwo = 88;

    println!("Once we apply bitwise AND the result would be :{:x}, Once we apply bitwise OR the result would be : {:x}, Once we apply bitwise XOR the result would be : {:x}", bitwiseone & bitwisetwo, bitwiseone | bitwisetwo , bitwiseone ^ bitwisetwo);

/*
Although we have already used the "=" assignment operator to assign values now we are gonna see compound assignment operator

+= adds a value and assign
-= subtracts a value and assign
*= multiply a value and assign
/= divide a value and assign
&= bitwise operation and assign
|= bitwise or and assign
 */

let mut rawvalue:i32 = 32;
let mut rawvalue2:i32 = 32;

rawvalue+= 68; //Adds 68 to rawvalue and assigns Back to rawvalue back the new value i.e 32+68
println!("The new value is : {}", rawvalue); //Prints new value

    rawvalue -= 32; //Substracts and assigns back to rawvalue the old value that is 100-32
    println!("The new value is : {}", rawvalue); //Prints new value

    rawvalue2 /= 2 ;
    println!("The new value of rawvalue2 is : {}", rawvalue2); //Divides with the original value with 2 and stores the result inside rawvalue2 that is 16.
    rawvalue2 *= 3 ;
    println!("The new value of rawvalue2 after new operand is : {}", rawvalue2) ; // Multiplies 3 with 16 and prints i.e 48.

// Noe some Typecasting

    /*
    Type casting in simple words  change the type of the data type to some other
   syntax : operand as newdatatype
     */

let mut originalvalue:i32 = 100;
    let mut modifiedtype:f64 = originalvalue as f64 ; //Type casted from integer to float64
println!("The new value is : {}", modifiedtype * 76.889) ;//Printed

    //Integer to string

    let integerx = 7778 ;
    let str = integerx.to_string();
    println!("{}", str + "0w0"); //Converted from Integer to string and added some more string :')

/*Time has come to learn something about Borrowing and dereferences

Borrowing : It is simply like storing the memory location of our operand to other variable and then
the variable which holds this is known as referenced variable, just assume a variable 'a' is stored in
memory at location 0x70 so another variable 'b' points to the memory location and borrows it, now borrowing
is of two types one is the shared borrowing other the mutable borrowing, in shared borrowing the value cannot be altered
whereas in mutable borrowing the value can be altered. Now coming to the point how do we dereference, just by adding '*'
before the referenced variable we can do that also update it, if it is mutable.
 */

let  toborrow = 70; //declare and initialize a immutable variable
let mut tomutableborrow:i64 = 80; //declare and initialize a mutable variable

    let sharedborrowing = &toborrow; //shared borrowing
    let mutableborrowing = &mut tomutableborrow; //mutable borrowing
    *mutableborrowing = 10; //dereferencing and adding some value to the referenced variable
    let mut kp =9; //declared and initialized
    *mutableborrowing = kp; //dereferenced

    println!("{sum}", sum = *mutableborrowing + 89);
    println!("{:x}", sharedborrowing + 89); //Printing the shared borrowed variable
    println!("{}", kp + 91); //Printing the dereference variable and adds 91 to it.

// Now we will be doing some conditionals : Just like another programming language changing the control flow
    //of the program based on some decision is where conditional expression comes to play.


    /*
    Rust gas three types of conditional expressions:

    "if" expression
    "if let" expression
    "match" expression
     */

let mut condition:&str = "Golang";//Declaring a variable
if condition == "Rust"{ //Comparing it against a condition

    println!("Yes I do like {}", condition);
}
else {                                //if ... else condition
    condition = "C++";
    println!("Yep I like {}", condition);
}

let number:i32 = 7;  //Declaring and initializing a variable
if number == 9{ //Comparing it against a variable

println!("The number is definitely not 9");
}

else if number == 7{ //If the condition in "if" does not fall correct it will jump to the "else if" condition
println!("The number is 7");

}

else{ //If the condition does not fall on the "if" or "if else" condition it will fall back to the "else" condition
println!("The number is definitely nor 7 neither 9");
}


//One-liner if..else

let oneline = 'x' ;
let v = if oneline == 'y' {println!("No");} else {println!("Yes");};


    /*
    if let : Now this might be new to a few people, imagine this in a simple way you have a set of value
    initialized to a variable 'b' like "let b = ("India", "USA", "Cuba", "China", "Russia") now if let matches
    with this scrutiny expression if it matches the first expression it can guess the rest, if it does not matches
    it will fall to the else block or the next else..if block and further. Now let us see how it works.
    */

let subjects = ("Mathematics", "Physics", "Chemistry"); //his is known as scrutiny expression or in simple terms against whom you want to compare//
if let ("Mathematics" , "Physics" , "Chemistry") = subjects {
println!("List of all the subjects in the tuple subjects : {:?}", subjects); //Here the match works and it prints all the subjects
}
else{
println!("The pattern did not match");
}

//Now we will see an example where if let did not match with the scrutiny expression

let notmatch = ( 1, 2, 3,4,5,6,7,8);
if let (2,4,6,8,9,_, _,_) = notmatch{
println!("The pattern matched");
}
else{
println!("Thepattern did not match with the scrutiny expression");
}

//Guessing a random or last variable using if..let

let alphabets = ('A' , 'B', 'C','D', 'E', 'F');
if let ('A', 'B', 'C','D', 'E', guess) = alphabets{
if guess == 'F'{
println!("Perfectly matched");
}
else{
println!("Not perfectly matched");
}
}
else{
println!("The if let stuff doesn't work");
}


//Sometimes the pattern is knowingly should be executed which means the statement inside the if..let should execute
let knowingly = ( 89, 90, 91, 92, 93, 94);
if let ( _, _, _,_, _ , _) = knowingly {  //No equal values, no checking just the pattern replaced with '_' and it jumps to execute the statements inside the if..let
println!("Yes Executed");

}

// Match Expression :
}
