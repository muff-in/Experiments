rule HelloRule
{

  strings:
    $first_string = "Hello World" //Checks for the string Hello World
    $elf_check  = { 7F 45 } // Checks whether the file is ELF or not 

 condition:
   $first_string and $elf_check

}
