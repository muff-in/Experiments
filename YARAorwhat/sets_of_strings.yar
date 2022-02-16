rule set_of_strings{


strings:
    $russian = ".ru" 
    $info = ".info" 
    $someother = ".in | .pk | .cn | .co.uk" 


condition:
    1 of ($russian , $info , $someother)  // If any one of them is present 
    1 of ($r*, $i* , $s*) //Representing the cases using wild cards.
    2 of them  // This is an alternative of 2 of ( $russian , $info, $someother)
    any of them //lol any of them








}