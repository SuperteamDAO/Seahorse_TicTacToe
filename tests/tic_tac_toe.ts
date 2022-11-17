import * as anchor from "@project-serum/anchor";
import { Program ,web3} from "@project-serum/anchor";
import { BN } from "bn.js";
import { TicTacToe } from "../target/types/tic_tac_toe";

describe("tic_tac_toe", async() => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TicTacToe as Program<TicTacToe>;

  const owner=program.provider.publicKey;

  const [game1] = await web3.PublicKey.findProgramAddress(
    [Buffer.from("Game-account"), owner.toBuffer()],
    program.programId);
    const play1=await web3.Keypair.generate();
    const play2=await web3.Keypair.generate();
    it("init Game", async () => {
        const txhash=await program.methods.initGame(play1.publicKey,play2.publicKey)
        .accounts({
          owner:program.provider.publicKey,
          game:game1
        }).rpc();
        await program.provider.connection.confirmTransaction(txhash);
      })   
    it("play Game", async()=>{
      const person=1;
      const position=1;
      const txhash=await program.methods.playGame(person,position)
      .accounts({
        owner:program.provider.publicKey,
        gameData:game1
      }).rpc();
      await program.provider.connection.confirmTransaction(txhash);
      const game_account=await program.account.game.fetch(game1);
      console.log(game_account);
    })
});
