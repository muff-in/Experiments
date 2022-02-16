rule modifycase{



strings:
    $stopcasecheck = "hello world" nocase
    $hex_string = {7f45 4c46 0201 0100 0000 0000 0000 0000}
    $fullword = "discord" fullword  // This will check all discord.com or discord.ru or discord.in shady domains
    $hash1 = /md5: [0-9a-fA-F]{33} /
    $re2 = /state: (on|off)/

condition:
    $stopcasecheck and $hex_string and $fullword and $hash1 and $re2
}