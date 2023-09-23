
                                                              
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
For mac users:
```
brew tap ben-toker/flowst
brew install flowst

```
In order to use the most recent version of the project, cloning the repo and re-building the app would be necessary. 

**Not available on windows currently!**
(You could try to install it by cloning the repo and compiling it with cargo, but
it is completely fudged on this platform. I'm working on getting it to work on Windows.)


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

# Directives
```
    - Include *cassowary* crate to ensure scaling constraints
    - Find out why this breaks on Windows and how to fix it.
```
