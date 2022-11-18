# fizzbuzz
# Built with Seahorse v0.2.2
#
# On-chain, persistent FizzBuzz!

from seahorse.prelude import *

# This is your program's public key and it will update
# automatically when you build the project.
declare_id('HNBneQ4xwdY7VLmb7HiWYUyKUjf2JjacA1mnJWTLp7B6');

class State(Enum):
  start=0
  Player1wins=1
  Player2wins=2
  draw=3

class Game(Account):
  players:Array[Pubkey,3]
  moves:Array[u8,10]
  game_status:u8
  next_move:u8

@instruction
def init_game(owner:Signer,player1:Pubkey,player2:Pubkey,game:Empty[Game]):
  game=game.init(
    payer=owner,
  )
  game.players[1]=player1
  game.players[2]=player2
  game.game_status=0
  game.next_move=1


def win_check(moves:Array[u8,10]) -> i8:
  # Currently Seahorse is not supporting with Index 0 for Array, that's why we start the Array with Index 1 to Index 10 ,it makes 3x3 Grid.
  # 1 2 3
  # 4 5 6
  # 7 8 9
  if((moves[1]==1 and moves[2]==1 and moves[3]==1) or (moves[1]==1 and moves[4]==1 and moves[7]==1) or 
    (moves[7]==1  and moves[8]==1 and moves[9]==1) or (moves[3]==1 and moves[6]==1 and moves[9]==1) or
    (moves[1]==1 and moves[5]==1 and moves[9]==1 ) or (moves[3]==1 and moves[5]==1 and moves[7]==1) or
    (moves[2]==1 and moves[5]==1 and moves[8]==1) or (moves[4]==1 and moves[5]==1 and moves[6]==1)):
      return 1
  if((moves[1]==2 and moves[2]==2 and moves[3]==2) or (moves[1]==2 and moves[4]==2 and moves[7]==2) or 
    (moves[7]==2  and moves[8]==2 and moves[9]==2) or (moves[3]==2 and moves[6]==2 and moves[9]==2) or
    (moves[1]==2 and moves[5]==2 and moves[9]==2 ) or (moves[3]==2 and moves[5]==2 and moves[7]==2) or
    (moves[2]==2 and moves[5]==2 and moves[8]==2) or (moves[4]==2 and moves[5]==2 and moves[6]==2)):
      return 2
  if(moves[1]>0 and moves[2]>0 and moves[3]>0 and moves[4]>0 and moves[5]>0 and moves[6]>0 and moves[7]>0 and moves[8]>0 and moves[9]>0):
      return 3
  else:
      return 0
    
@instruction
def play_game(player:Signer,game_data:Game,played_by:u8,move_position:u8):
   #to check the signer is valid or not.
   assert game_data.players[played_by]==player.key(),'Invalid Signer'
   #to check the game_status, if the game_status is 0 , still there is a game , if not the game is end,you cannot play
   assert game_data.game_status == 0,'Invalid Instruction'
   #it can be solve , one player cannot play another player game.
   assert played_by==game_data.next_move,'Invalid Player'
   #this can be solve,player cannot make his move on already occupied cell,which was occupied by another player
   assert game_data.moves[move_position]==0,'Invalid move position'
   if(game_data.next_move==1):
      game_data.moves[move_position]=1
      game_data.next_move=2
   elif(game_data.next_move==2):
      game_data.moves[move_position]=2
      game_data.next_move=1
   game_status=win_check(Array(game_data.moves,len=10))
   if(game_status==State.start):
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

  