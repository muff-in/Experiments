rule checkifwildcardworks
{

      strings:
          $hexstrings = { 7f 45 4c 46 ?? ?? 01 00}
          $hexstringstwo = {?? ?? 4c 46 ?? ?? 01 00}

     condition:
         $hexstrings or  $hexstringstwo   



}