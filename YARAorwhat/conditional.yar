rule conditonalstrings {

strings :

    $hex_strings = { 7f45 4c46 0201 0100 (0000 0000| 0101 0101) 0000 0000} //Conditions among the hex strings
    $normal_string = "Hello World"

condition:
    $hex_strings or $normal_string

}