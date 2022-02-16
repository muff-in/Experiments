rule unknownlength {


strings:
    $gcc_check = "GCC"
    $is_upx = "UPX"
    $hex_strings = { 7f45 4c46 0201 [4-6] 0000 0000 }

condition:
    $gcc_check or $is_upx or $hex_strings

}