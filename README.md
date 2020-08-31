# Mount-drive-in-linux

I tried mounting my HDD automatically with fstab but it is not giving me appropriate permissions. I tried everything. But then I found that if I mount using nautilus in gnome I was getting appropriate permissions. But here the problem was that I wasn't able to find automatic mount. But after researching I found that by using ```gio mount``` command. So, that is what I wrote a script for in rust. Now putting it in startup scripts, it will mount my HDD automatically on startup.

## Index
* [Technologies](#technologies)
* [Setup](#setup)
* [Credit](#credit)
* [Contribute](#contribute)
* [License](#license)

## Technologies
Project is created with:
* rustc 1.45.0
* cargo 1.45.0

## Setup
First of all open the ```src/main.rs``` and change the drive name al line 8 to what you wish to mount.
To run the project you need to install rust version 1.45.0. For reference you can see [Technologies](#technologies). Then run:

```
$ git clone https://github.com/Sahil2004/Mount-drive-in-linux.git
$ cd Mount-drive-in-linux
$ cargo install cargo-deb
$ cargo deb
```
Then visit ```target/debian``` and then there you will find a file with .deb extension. That is your app that you have to install and run on system startup.
```
$ cd ~/.config/autostart/
```
If you don't have a folder named autostart then create one with that name using ```$ mkdir autostart```.
Now type the following commands:
```
$ touch mount-external-drive-onstartup-using-nautilus.sh.desktop.sh.desktop
$ nano mount-external-drive-onstartup-using-nautilus.sh.desktop.sh.desktop
```
Then paste:
```
[Desktop Entry]
Type=Application
Exec="mount-drive-using-gnome"
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true
Name[en_IN]=Mount-external-HDD-on-startup.
Name=Mount-external-HDD-on-startup.
Comment[en_IN]=Automation of mounting HDD using nautilus. Made by Sahil Garg in Rust.
Comment=Automation of mounting HDD using nautilus. Made by Sahil Garg in Rust.
```
Then press ```ctrl+o```, ```Enter``` and then ```ctrl+x```.
Done!
The next time you boot up, your disk will be mounted automatically.

## Credit
The software was created by Sahil Garg.
This was made for personal use but thought that might help others looking for some solution like this.

## Contribute
You may contribute by updating the readme by submitting a pull request through a fork of the repo. You may add setup instrructions for other use cases and other distros. You may even contribute the code if you think there can be some more improvement in it. To make it unarchived DM me on twitter (@real_SahilGarg).

## License
To see the license open the LICENSE file in the repo.
