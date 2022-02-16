rule matchlength{

strings:
    $catch_unwind = "_Unwind_*"


condition:
    
    (!catch_unwind[1]  == 14 and !catch_unwind[2] < 16) or filesize == 3577KB

}