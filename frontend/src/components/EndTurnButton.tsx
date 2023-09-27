import React, { useState } from "react";
import * as anchor from "@coral-xyz/anchor";
import Button from "@mui/material/Button";
import { useWorkspace } from "../context/AnchorContext";
import { useGameState } from "../context/GameStateContext";

const EndTurnButton: React.FC = () => {
  const { program, provider } = useWorkspace();
  const { fetchPlayerState, fetchGameState, fetchNpcs } = useGameState();
  const [isProcessing, setIsProcessing] = useState(false);
  const [isClosingGame, setIsClosingGame] = useState(false);

  const endTurn = async () => {
    setIsProcessing(true);
    console.time("End turn");
    try {
      const [gameKey] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("GAME"), provider!.publicKey.toBuffer()],
        program!.programId
      );
      const [playerKey] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("PLAYER"), gameKey.toBuffer(), provider!.publicKey.toBuffer()],
        program!.programId
      );
      const [npcKey] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("NPC"), gameKey.toBuffer()],
        program!.programId
      );
      const accounts = {
        game: gameKey,
        playerAccount: playerKey,
        npcAccount: npcKey,
        player: provider!.publicKey,
      };
      const tx = await program!.methods.endTurn().accounts(accounts).rpc();
      console.log(`End turn TX: https://explorer.solana.com/tx/${tx}?cluster=devnet`);
      await fetchPlayerState();
      await fetchGameState();
      await fetchNpcs();
    } catch (error) {
      console.error("Failed to end turn", error);
    }
    console.timeEnd("End turn");
    setIsProcessing(false);
  };

  const closeGame = async () => {
    if (window.confirm("Are you sure that you want to end the game? All your progress will be lost 💀💀💀") === false) {
      return;
    }
    setIsClosingGame(true);
    setIsProcessing(true);
    try {
      const [gameKey] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("GAME"), provider!.publicKey.toBuffer()],
        program!.programId
      );
      const [playerKey] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("PLAYER"), gameKey.toBuffer(), provider!.publicKey.toBuffer()],
        program!.programId
      );
      const [npcKey] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("NPC"), gameKey.toBuffer()],
        program!.programId
      );
      const accounts = {
        game: gameKey,
        playerAccount: playerKey,
        npcAccount: npcKey,
        player: provider!.publicKey,
      };
      await program!.methods.closeGame().accounts(accounts).rpc();
    } catch (error) {
      setIsProcessing(false);
      console.error("Failed to close game", error);
      alert(error);
      return;
    }
    // redirect to reset the context & local state
    window.location.href = "/";
  };

  return (
    <>
      <Button onClick={endTurn} disabled={isProcessing} variant="contained" color="primary">
        ⌛ End Turn
      </Button>
      <Button onClick={closeGame} variant="contained" color="error" className="end-game-button">
        💀 End Game
      </Button>
      {isProcessing && (
        <div
          style={{
            position: "fixed",
            top: 0,
            left: 0,
            width: "100%",
            height: "100%",
            backgroundColor: "rgba(0, 0, 0, 0.8)",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            zIndex: 1000,
          }}
        >
          <span style={{ color: "white", fontSize: "20px" }}>
            {!isClosingGame && "Waiting for moves of your opponent..."}
            {isClosingGame && "Closing game accounts. Please wait..."}
          </span>
        </div>
      )}
    </>
  );
};

export default EndTurnButton;
