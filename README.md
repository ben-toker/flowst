
                                                              
```                                                            
          ,--,                                        ___     
  .--., ,--.'|                                      ,--.'|_   
,--.'  \|  | :     ,---.           .---.            |  | :,'  
|  | /\/:  : '    '   ,'\         /. ./|  .--.--.   :  : ' :  
:  : :  |  ' |   /   /   |     .-'-. ' | /  /    '.;__,'  /   
:  | |-,'  | |  .   ; ,. :    /___/ \: ||  :  /`./|  |   |    
|  : :/||  | :  '   | |: : .-'.. '   ' .|  :  ;_  :__,'| :    
|  |  .''  : |__'   | .; :/___/ \:     ' \  \    `. '  : |__  
'  : '  |  | '.'|   :    |.   \  ' .\     `----.   \|  | '.'| 
|  | |  ;  :    ;\   \  /  \   \   ' \ | /  /`--'  /;  :    ; 
|  : \  |  ,   /  `----'    \   \  |--" '--'.     / |  ,   /  
|  |,'   ---`-'              \   \ |      `--'---'   ---`-'   
`--'                          '---"                           
 ```                                                             


# Welcome to Flowst!

This is designed to be a *really* basic pomodoro CLI tool built w/ **Rust**. 

# This is the basic application.
I made a "simple" branch that just takes in a timer with default values.
These values are set to the basic pomodoro ratio (25min:5min) but allows for
custom values to be inputted with -w (-work) or -r (-rest) flags.
This will start the timer, without being able to pause.

I initially put this here as a firm checkpoint while I messed around with more complicated stuff in the main branch and was intending to merge them after I made decent progress in the main branch. However,
I think I'll keep this here as a more minimal, pared down version of the app. This simply is a CLAP (command-line-argument-parser) based tool that prints out when each timer is starting and has finished. 

Experimental branch:
This branch is a lot more complicated, involving further async functionality, 
potentially multi-threaded operations, etc. etc. 

Have fun!
