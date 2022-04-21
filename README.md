# my_little_rpg

This is a game coded in rust. It is: 
1. Turn based: Nothing happens before you execute a command. As a result of a command a lot can change. 
2. Role playing game: There is a focus on character development and defeating obstacles. 
3. Command line interface based: You execute commands to the game from a command line and recives all input back again to the command line. 

So it is a Command line interface based Turn based role playing game: CLIT RPG :) 

# Getting started 

Download the executable: TBD
Startup the server: TBD

Have some method of sending TCP packages from the command line. A popular choice is ncat; It is on most linux distroes by default. 

It is also recommended to have some method of parsing JSON. A popular choice is JQ: https://stedolan.github.io/jq/ 

## Some start commands: 

1. printf "State" | ncat -C localhost 1337 | jq .
2. printf "Move 0" | ncat -C localhost 1337
3. printf "Help" | ncat -C localhost 1337
