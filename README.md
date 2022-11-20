#### This project was created by Seahorse 0.2.4.

# Seahorse Tic-Tac-Toe Program

The main objective of this program is to Implement a TicTacToe game using Seahorse. Where players can play their game on the deployed contract.

# Prerequisites

we need to Install [Solana](https://docs.solana.com/cli/install-solana-cli-tools),[Anchor](https://www.anchor-lang.com/docs/installation),[Rust](https://www.rust-lang.org/tools/install),and [Seahorse](https://seahorse-lang.org/docs/installation).

if you are not installed click on the link to the guide to install it.

you can check if the package got installed or not by the following commands.


`solana -V` `anchor -V` `rustc -V` `seahorse -V`

I have used these versions for the Project :- 

* Solana(1.14.3)
* Anchor(0.25.0)
* rust(1.65.0)
* seahorse(0.2.4)

# Getting Started with Seahorse

To initialize seahorse project we need to use this command `seahorse init tic_tac_toe `

it will create a project directory and it contains  py file under `programs_py/tic_tac_toe.py`

I wrote a full [blog](https://chskkishore.hashnode.dev/the-easy-way-of-writing-solana-smart-contracts-using-seahorse) how to get start with seahorse.

click on the link and you get a detailed explanation to get start with Seahorse.

# Program Accounts

In this Project , we have used only one account,Game account. it contains data fields like `player1:Pubkey` `player2:Pubkey` `moves:Array[u8,10]` `game_status:u8` `next_move:u8`

we need to initialize the Game account to start the game.

```
class Game(Account):
  player1:Pubkey
  player2:Pubkey
  moves:Array[u8,9]
  game_status:u8
  next_move:u8
```

#  Program Instructions

In this project, we have 2 Instruction and one function ,we will go with one by one in detail.

# 1. Init Game

In this Instruction, we need to Initialize the game account to start the game, we set some default values to the game account.

```
@instruction
def init_game(owner:Signer,player1:Pubkey,player2:Pubkey,game:Empty[Game]):
  game=game.init(
    payer=owner,
  )
  game.players[0]=player1
  game.players[1]=player2
  game.game_status=0
  game.next_move=1

```
as you can see in the above snippet we set some default values to game account.

# 2. Play Game

In this Instruction, the main part of the game logic is done. where players call the instructions with their moves. and get the return response to their moves. and also In this instruction, it will decide who is the winner or the game is drawn. if the user makes any invalid step, and it shows the error as a response.

```   
@instruction
def play_game(player:Signer,game_data:Game,played_by:u8,move_position:u8):
   #to check the signer is valid or not
   assert game_data.players[played_by-1]==player.key(),'Invalid Signer'
   #to check the played_by and game_data.next_move 
   assert played_by==game_data.next_move,'Invalid Player'
   #to check the game_status, if the game_status is 0 , still there is a game , if not the game is end,you cannot play
   assert game_data.game_status == 0,'Invalid Instruction'
   #this can be solve,player cannot make his move on already occupied cell,which was occupied by another player
   assert game_data.moves[move_position-1]==0,'Invalid move position'
   move_position=move_position-1
   if(game_data.next_move==1):
      game_data.moves[move_position]=1
      game_data.next_move=2
   elif(game_data.next_move==2):
      game_data.moves[move_position]=2
      game_data.next_move=1
   game_status=win_check(Array(game_data.moves,len=9))
   if(game_status==State.game):
     print(game_data.next_move)
   elif(game_status==State.Player1wins):
     game_data.game_status=1
     print("player1 wins the Game")
   elif(game_status==State.Player2wins):
     game_data.game_status=2
     print("player2 wins the Game")
   elif(game_status==State.draw):
     game_data.game_status=3
     print("Game Draw")
   else:
     print("Game Error")
```
# 3. win check

This is the function we have, In this function, we check the condition to make the player as the winner. and we check the conditions is the game ends in a draw.

```
def win_check(moves:Array[u8,9]) -> State:
  if((moves[0]==1 and moves[1]==1 and moves[2]==1) or (moves[0]==1 and moves[3]==1 and moves[6]==1) or 
    (moves[6]==1  and moves[7]==1 and moves[8]==1) or (moves[2]==1 and moves[5]==1 and moves[8]==1) or
    (moves[0]==1 and moves[4]==1 and moves[8]==1 ) or (moves[2]==1 and moves[4]==1 and moves[6]==1) or
    (moves[1]==1 and moves[4]==1 and moves[7]==1) or (moves[3]==1 and moves[4]==1 and moves[5]==1)):
      return State.Player1wins
  if((moves[0]==2 and moves[1]==2 and moves[2]==2) or (moves[0]==2 and moves[3]==2 and moves[6]==2) or 
    (moves[6]==2  and moves[7]==2 and moves[8]==2) or (moves[2]==2 and moves[5]==2 and moves[8]==2) or
    (moves[0]==2 and moves[4]==2 and moves[8]==2 ) or (moves[2]==2 and moves[4]==2 and moves[6]==2) or
    (moves[1]==2 and moves[4]==2 and moves[7]==2) or (moves[3]==2 and moves[4]==2 and moves[5]==2)):
      return State.Player2wins
  if(moves[0]>0 and moves[1]>0 and moves[2]>0 and moves[3]>0 and moves[4]>0 and moves[5]>0 and moves[6]>0 and moves[7]>0 and moves[8]>0):
      return State.draw
  else:
      return State.game
      
```
you can see in the above snippet, we are checking every possible to make a decision on who is the winner of the game, and the game status. and returning the game status.

# Run Tests

if you want to run it on your local machine, you can run I have written the typescript test under the `tests` folder.

![image](https://user-images.githubusercontent.com/55663050/202907804-3fe6636d-66b6-43c7-8646-69e9b548d682.png)


# Conclusion

The Seahorse Simplified the Writing of Smart Contracts on Solana, it really helps so many new developers to come to write smart contracts on Solana.



