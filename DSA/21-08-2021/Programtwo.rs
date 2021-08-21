/* Date : 21-08-2021
Program 1 : Reversing the elements of the array.

 */

//Reversing the elements of the array
fn main() {
    let array_without_reverse: [i32; 6] = [67,68,69,70,71,72]; //Declares and initializes
    let mut array_with_reverse = array_without_reverse;
    array_with_reverse.reverse(); 
    println!("The reversed array is : {:?}", array_with_reverse);

}
