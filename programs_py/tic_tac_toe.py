# fizzbuzz
# Built with Seahorse v0.2.2
#
# On-chain, persistent FizzBuzz!

from seahorse.prelude import *

# This is your program's public key and it will update
# automatically when you build the project.
declare_id('HNBneQ4xwdY7VLmb7HiWYUyKUjf2JjacA1mnJWTLp7B6')

class Game(Account):
  player1:Pubkey
  player2:Pubkey
  moves:Array[u8,10]
  game_status:u8
  next_move:u8


@instruction
def init_game(owner:Signer,player1:Pubkey,player2:Pubkey,game:Empty[Game]):
  game=game.init(
    payer=owner,
  )
  game.player1=player1
  game.player2=player2
  game.game_status=0
  game.next_move=1

def win_check(moves:Array[u8,10]) -> i8:
  if((moves[1]==1 and moves[2]==1 and moves[3]==1) or (moves[1]==1 and moves[4]==1 and moves[7]==1) or 
    (moves[7]==1  and moves[8]==1 and moves[9]==1) or (moves[3]==1 and moves[6]==1 and moves[9]==1) or
    (moves[1]==1 and moves[5]==1 and moves[9]==1 ) or (moves[3]==1 and moves[5]==1 and moves[7]==1) or
    (moves[2]==1 and moves[5]==1 and moves[8]==1) or (moves[4]==1 and moves[5]==1 and moves[6]==1)):
      return 1
  elif((moves[1]==2 and moves[2]==2 and moves[3]==2) or (moves[1]==2 and moves[4]==2 and moves[7]==2) or 
    (moves[7]==2  and moves[8]==2 and moves[9]==2) or (moves[3]==2 and moves[6]==2 and moves[9]==2) or
    (moves[1]==2 and moves[5]==2 and moves[9]==2 ) or (moves[3]==2 and moves[5]==2 and moves[7]==2) or
    (moves[2]==2 and moves[5]==2 and moves[8]==2) or (moves[4]==2 and moves[5]==2 and moves[6]==2)):
      return 2
  elif((moves[1]==1 or moves[1]==2) and (moves[2]==1 or moves[2]==2) and(moves[3]==1 or moves[3]==2) and (moves[4]==1 or moves[4]==2) and (moves[5]==1 or moves[5]==2) and (moves[6]==1 or moves[6]==2) and
       (moves[7]==1 or moves[7]==2) and (moves[2]==1 or moves[8]==2) and (moves[9]==1 or moves[9]==2)):
      return 3
  else:
      return 0
    
@instruction
def play_game(owner:Signer,game_data:Game,played_by:u8,move_position:u8):
  move_position=move_position-1
  if(game_data.game_status==0):
    if((game_data.moves[move_position]==0) and (game_data.next_move==played_by)):
      if(game_data.next_move==1):
        game_data.moves[move_position]=1
        game_data.next_move=2
      elif(game_data.next_move==2):
        game_data.moves[move_position]=2
        game_data.next_move=1
    else:
        print("wrong move")
    game_status=win_check(Array(game_data.moves,len=10))
    if(game_status==0):
      print(game_data.next_move)
    elif(game_status==1):
      game_data.game_status=1
      print("player1 wins the Game")
    elif(game_status==2):
      game_data.game_status=2
      print("player2 win the game")
    elif(game_status==3):
      game_data.game_status=3
      print("The Game was draw")
    else:
      print("Error Game")
  else:
    print("Invalid Instruction")
      
  