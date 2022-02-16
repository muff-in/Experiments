rule check_correct_offset{

 strings:
     $is_elf = "7f45 4c46 0201 0100 [6-8]"
     $some_string_present = "6000 0000 0000 0000 5821 0003 0000 0000"
     


condition:
    $some_string_present at 100 or $is_elf at 0x0100


}