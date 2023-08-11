
                                                              
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


# Flowst
Flowst is a CLI tool for the Pomodoro Technique, built in Rust. It provides a user-friendly text-based interface for managing work and rest intervals, allowing you to stay focused and productive.

Flowst Screenshot Optional screenshot of the application

Features
- Timer Management: Start, pause, and reset work and rest intervals
- Interface: A text-based interface that displays the timer, configurations, welcome logo, and controls.
- Configuration Handling: Save, load, and reset timer configurations to suit your preferences.
- Scrollable Configurations: Easily navigate through different timer configurations.
- Keyboard Controls: Intuitive keybindings for controlling the timer and navigating the UI.

# Installation
Might put it up on Homebrew, but not sure.

For now, to use this app, clone the repository and run the executable or use ``cargo run``

# Usage
```
# View CLAP interface

flowst

# Example command to start a 25 : 5 timer (standard)

flowst start

# Custom time:

flowst start -w 40 -r 20

# Run the app:

flowst app


```

