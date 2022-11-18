import * as anchor from "@project-serum/anchor";
import { Program ,web3} from "@project-serum/anchor";
import { Transaction } from "@solana/web3.js";
import { BN } from "bn.js";
import { TicTacToe } from "../target/types/tic_tac_toe";

function printgame(array) {
  console.log(`${array[1]} ${array[2]} ${array[3]}`)
  console.log(`${array[4]} ${array[5]} ${array[6]}`)
  console.log(`${array[7]} ${array[8]} ${array[9]}`)
  
}

describe("tic_tac_toe", async() => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TicTacToe as Program<TicTacToe>;

  const game1=anchor.web3.Keypair.generate();
  
  const playerOne = (program.provider as anchor.AnchorProvider).wallet;
  const playerTwo = anchor.web3.Keypair.generate();

  console.log(playerOne.publicKey.toString());
  console.log(playerTwo.publicKey.toString());
  console.log(game1.publicKey.toString());
  
  it("init Game", async () => {
       const txhash=await program.methods.initGame(playerOne.publicKey,playerTwo.publicKey)
       .accounts({
          game:game1.publicKey,
          owner: program.provider.publicKey
       })
       .signers([game1])
       .rpc()       
      })   
    it("play Game", async()=>{
      const person=1;
      const position=1;
      const txhash=await program.methods.playGame(person,position)
      .accounts({
        player:playerOne.publicKey,
        gameData:game1.publicKey
      }).rpc();
      await program.provider.connection.confirmTransaction(txhash);
      const game_account=await program.account.game.fetch(game1.publicKey);
      printgame(game_account.moves);
    })
    it("play Game", async()=>{
      const person=2;
      const position=2;
      const txhash=await program.methods.playGame(person,position)
      .accounts({
        player:playerTwo.publicKey,
        gameData:game1.publicKey
      }).signers([playerTwo]).rpc();
      await program.provider.connection.confirmTransaction(txhash);
      const game_account=await program.account.game.fetch(game1.publicKey);
      printgame(game_account.moves);
    })
});
