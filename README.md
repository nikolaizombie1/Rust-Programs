# rplexformatter

## Description
This tool allows you to batch rename files into a format recognized by Plex Media Server.

## Building from source
1. Download and install [rustup](https://rustup.rs/).
2. Clone the github repository
``` sh
git clone "https://github.com/nikolaizombie1/rplexformatter.git" 
```
3. Enter the repository
``` sh
cd rplexformatter
```
4. Build the executable binary
``` sh
cargo build --release
```
5. Make the binary executable
``` sh
cd target/release && chmod 755 plexformatter
```
6. Move the executable to .local/bin
``` sh
cp plexformatter ~/.local/bin
```

## Usage
1. Run plexformatter from the shell inside the directory of the files you would like to rename.
![Running the executable](ReadMe_Pictures/1.%20Usage.png)
2. Input the name you would like the files to be called. Note: It cannot contain a '/' character.
![Entering a name](ReadMe_Pictures/2.%20Usage.png)
3. Input the season number. It has to be a number greater than or equal to 0.
![Entering a season](ReadMe_Pictures/3.%20Usage.png)
4. Preview the dry run on the files.
![Dry run](ReadMe_Pictures/4.%20Usage.png)
5. Select a range of files you would like to rename: Note: pressing enter will select all of the files as is. Valid formats are number-number (Example: 1-5), comma separated values (Example: 1,2,3,4,5) and space separated values (Example: 1 2 3 4 5).
![Selecting range](ReadMe_Pictures/5.%20Usage.png)
6. Review the dry run from the selected files.
![Review dry run](ReadMe_Pictures/6.%20Usage.png)
7. Authorize the renaming of the files.
![Authorize changes](ReadMe_Pictures/7.%20Usage.png)
8. Optional: Authorize the moving the files into plex formatted folder.
![Move files to folder](ReadMe_Pictures/8.%20Usage.png)
