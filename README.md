
                                                              
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


# What is the Pomodoro Technique?
Essentially, the Pomodoro Technique is a tool for focus; you set a timer for the period for which you intend to work followed by a shorter timer for a much needed rest period. You repeat this timer duo until the work is done! Typically, this is done in 25 minute (work) and 5 minute (rest) intervals, but can be modified to any ratio of intervals you prefer (personally I tend to go for the 50/10 split)! 

# Flowst
Flowst is a command line interface tool for the Pomodoro Technique, built in Rust. It provides a user-friendly text-based interface for managing work and rest intervals, allowing you to stay focused and productive. Using Flowst, you can adjust custom intervals for work and rest and repeat them as many times as you want, from the convenience of the command line! 

<img width="863" alt="image" src="https://github.com/ben-toker/flowst/assets/117331544/9414b955-e884-4b3c-a586-f1181bd73fd5">


Features
- Timer Management: Start, pause, and reset work and rest intervals
- Interface: A text-based interface that displays the timer, configurations, welcome logo, and controls.
- Configuration Handling: Save, load, and reset timer configurations to suit your preferences.
- Scrollable Configurations: Easily navigate through different timer configurations.
- Keyboard Controls: Intuitive keybindings for controlling the timer and navigating the UI.
- Notifications: A bell sound plays after each interval, ensuring that you stay on task.

# Installation
For Mac users:
```
brew tap ben-toker/flowst
brew install flowst

```
In order to use the most recent version of the project, cloning the repo and re-building the app would be necessary. 

For Windows users:
I currently don't have a package manager set up for windows installation, but as long as you clone the repo and use cargo to build an executable, it works well (according to my Windows-user friends)!

# The App
You must use ``cargo run app`` or ``flowst app`` in order to take full advantage of Flowst's functionality. The bell notification cannot be heard unless
the app is open. If you need to use the terminal, simply open up a new terminal tab while flowst is running in the background. Since a background daemon is out of
the scope for this project, this is the easiest option.

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
    None currently! :)

```

# Directives
The main project (minimum viable product) is done, so the following are potential add-ons that would improve the experience! 
```
    - Include ```cassowary``` crate to ensure scaling constraints
    - Maybe include some music feature to queue youtube videos or some built-in sound for working.
    - Maybe a work log?
```
