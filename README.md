# dexmaker

a small horribly-written utility for quickly making desktop-entries on gnome systems

# Use
- typically needs sudo permissions 'cause it defaults to /usr/shared/applications for it's write location
the executable needs 4 arguments with an optional 5th


-n the desktop entry name (what shows up on your desktop/searchbar)

-f the name of the executable (e.g. steam, cargo, g++, etc)

-i the location of your icon (e.g. ~/Downloads/icontheme/scaled/icon.svg)

-s whether or not to show the console

-l [OPTIONAL] the directory to store the desktop entry (e.g. ~/Desktop/)
