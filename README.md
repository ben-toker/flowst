
                                                              
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

<img width="863" alt="image" src="https://github.com/ben-toker/flowst/assets/117331544/9414b955-e884-4b3c-a586-f1181bd73fd5">


Features
- Timer Management: Start, pause, and reset work and rest intervals
- Interface: A text-based interface that displays the timer, configurations, welcome logo, and controls.
- Configuration Handling: Save, load, and reset timer configurations to suit your preferences.
- Scrollable Configurations: Easily navigate through different timer configurations.
- Keyboard Controls: Intuitive keybindings for controlling the timer and navigating the UI.

# Installation
Might put it up on Homebrew, but not sure.

For now, to use this app, clone the repository and use  ``cargo build`` and ``cargo run``, or 
download the executable on the latest release.

# Usage
```
# View CLAP interface

flowst

# Example command to start a 25 : 5 timer (standard)

flowst start

# Custom time:

flowst start -w 40 -r 20

# Reset configuration file:

flowst reset

# Run the app:

flowst app


```
# Issues
```
    Not working on Windows correctly!

```
