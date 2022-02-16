rule alert_string
{

  strings:
      $C2 = "XXX.XXX.XX.XX"
      $message = "Your files are infected"
      

   condition:
       $C2 and $message 
}