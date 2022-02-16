rule check_xor{


strings:
    $does_xor_hit = "xoriscool" xor 
    $base_sixty_four_hit = "base64iscool" base64


condition:
  $does_xor_hit  or $base_sixty_four_hit 


}