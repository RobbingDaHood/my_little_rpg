# my_little_rpg

This is a game coded in rust. It is: 
1. Turn based: Nothing happens before you execute a command. As a result of a command a lot can change. 
2. Role playing game: There is a focus on character development and defeating obstacles. 
3. Command line interface based: You execute commands to the game from a command line and receives all input back again to the command line. 

So it is a Command line interface based Turn based role playing game: CLIT RPG :) 

# Getting started 

## Method one: Use our executable
1. Download the executable: TBD 
2. Startup the server: TBD
3. Have some method of sending TCP packages from the command line. A popular choice is ncat/netcat.
4. It is also recommended to have some method of parsing JSON. A popular choice is JQ: https://stedolan.github.io/jq/ 

## Method Two: Build executable yourself

1. Install rust: https://www.rust-lang.org/tools/install 
2. Checkout this repo
3. At the root of the repo: 'cargo build --release'
4. Startup the server: 'cargo run --release'
5. Have some method of sending TCP packages from the command line. A popular choice is ncat/netcat.
6. It is also recommended to have some method of parsing JSON. A popular choice is JQ: https://stedolan.github.io/jq/ . 

## Some start commands: 

### Using ncat and jq
1. printf "State" | ncat -C localhost 1337 | jq .
2. printf "Move 0" | ncat -C localhost 1337 | jq .
3. printf "Help" | ncat -C localhost 1337 | jq .

### Using netcat and jq
1. printf "State" | netcat localhost 1337 | jq .
2. printf "Move 0" | netcat localhost 1337 | jq .
3. printf "Help" | netcat localhost 1337 | jq .

# Some more about Command line interface games

When all the input and output happens through the command line then you can: 
1. Use all the command line tools you have. 
2. Organize the output from the game as you like.
3. Save the output data to files in whatever order you like. 
4. You can automate anything you like. Even create a fully autonomous AI if that is what you want. 
5. Open scripts in multiple terminals both for automation and visuals. 
6. You can treat the game as a backend and add any frontend you please. 

Command line games give you a lot of freedom. 

## This is not a text based games
Text based game wiki: https://en.wikipedia.org/wiki/Text-based_game 

The main difference is that text based games locks you into text based UI; You cannot use any CLI tools (at least not directly) on top of it, like with a CLI game. 

# Code format

Code is formatted using: https://github.com/rust-lang/rustfmt

# Project is using the nightly toolchain