# tic_tac_toe
# Built with Seahorse v0.2.4

from seahorse.prelude import *

declare_id('G5s63XbRZMDnypYYBcnZZxg73FftZ2byYe1KbpcNXUwf')

class State(Enum):
  game=0
  Player1wins=1
  Player2wins=2
  draw=3


class Game(Account):
  players:Array[Pubkey,2]
  moves:Array[u8,9]
  game_status:u8
  next_move:u8

@instruction
def init_game(owner:Signer,player1:Pubkey,player2:Pubkey,game:Empty[Game]):
  game=game.init(
    payer=owner,
  )
  game.players[0]=player1
  game.players[1]=player2
  game.game_status=0
  game.next_move=1


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

  