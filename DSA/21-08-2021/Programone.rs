/* Date : 21-08-2021
Program 1 : Finding the sum of elements in an array.

 */

//Finding the sum of elements in an array.
fn main() {
    println!("This program is used to find the sum of the elements of an array"); //Prints a message
    let arr : [i64; 5] = [67, 68, 69, 70, 71]; //declares and initializes an array
    let mut sum = 0; //a mutable variable in which the sum of all the array elements will be initialized via loop
    for i in arr{ //i = variable and arr is the range 

        sum = arr.iter().sum(); //.iter() iterates and .sum() sums the elements of an iterator.


    }
    println!("The total sum of the array is : {}", sum); //Prints the sum

}
