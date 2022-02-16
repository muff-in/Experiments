rule rust_binary{


strings:
    $check_backtrace = "[-]backtrace_rs[-]" fullword
   $check_if_rust_mangled_itanium_abi = "ZN71_$LT$core..panic..panic_info..PanicInfo$u20$as$u20$core..fmt..Debug$GT$3fmt17h02259adaca680c9eE"
    $just_checking = "memcpy (memchr | memrchr) getenv" fullword
    $some_more_mangling_for_the_sake_of_god = "ZN [-]" fullword
    $count_strings = "std::[-]"

condition:
    ($check_backtrace and $check_if_rust_mangled_itanium_abi)  and ( $just_checking and $some_more_mangling_for_the_sake_of_god) and (#count_strings > 5)
    


}