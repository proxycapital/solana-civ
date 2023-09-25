import React, { useState, useEffect } from "react";
import Container from "@mui/material/Container";
import Button from "@mui/material/Button";
import Grid from "@mui/material/Grid";
import { useNavigate } from "react-router-dom";
import { Connection, Keypair, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import bs58 from "bs58";
import { useWorkspace } from "../context/AnchorContext";
import { initializeGame } from '../utils/solanaUtils';
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import "../App.css";

const HomePage: React.FC = () => {
  const navigate = useNavigate();
  const workspace = useWorkspace();
  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  const [initializationSteps, setInitializationSteps] = useState([
    { name: "Requesting airdrop", status: "pending" },
    { name: "Initializing game", status: "pending" },
  ]);
  const [showButtons, setShowButtons] = useState(true);

  useEffect(() => {
    document.body.classList.add('light-mode');
    return () => {
      document.body.classList.remove('light-mode');
    };
  }, []);

  const updateStepStatus = (stepName: string, status: string) => {
    setInitializationSteps((steps) => steps.map((step) => (step.name === stepName ? { ...step, status } : step)));
  };

  const createWalletAndStartGame = async () => {

    setShowButtons(false);
    const connection = workspace.connection as Connection;
    const wallet = {
      publicKey: workspace.provider?.publicKey as PublicKey,
    };

    try {
      // get sol balance
      const balance = await connection.getBalance(wallet.publicKey);
      console.log("Balance: ", balance)
      if (balance > 0.1 * LAMPORTS_PER_SOL) {
        updateStepStatus("Requesting airdrop", "completed");
      } else {
        const airdropSignature = await connection.requestAirdrop(wallet.publicKey, 1 * LAMPORTS_PER_SOL);

        const latestBlockHash = await connection.getLatestBlockhash();
  
        await connection.confirmTransaction({
          blockhash: latestBlockHash.blockhash,
          lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
          signature: airdropSignature,
        });
        updateStepStatus("Requesting airdrop", "completed");
      }
    } catch (error) {
      console.log("Error while requesting airdrop: ", error);
      updateStepStatus("Requesting airdrop", "failed");
      setErrorMsg(`Requesting airdrop failed: ${error}`);
      setShowButtons(true);
      return;
    }

    try {
      // @todo: add better checks for workspace/provider/program
      const provider = workspace.provider!;
      const program = workspace.program!;
      await initializeGame(provider, program);
      updateStepStatus("Initializing game", "completed");
    } catch (error) {
      console.log("Error while initializing the game: ", error);
      updateStepStatus("Initializing game", "failed");
      setErrorMsg(`Initializing game failed: ${error}`);
      setShowButtons(true);
      return;
    }

    navigate("/game");
  };

  return (
    <Container className="home-container">
      <Grid container direction="column" alignItems="center" justifyContent="center" className="center-grid">
        <Grid item xs={12}>
          <img src="/logo.png" alt="Logo" className="logo" />
        </Grid>
        {showButtons ? (
          <>
            <Grid item xs={12}>
              <Button
                variant="contained"
                color="primary"
                className="fixed-width-button"
                onClick={createWalletAndStartGame}
              >
                Start New Game
              </Button>
            </Grid>
            <Grid item xs={12}>
              <Button variant="outlined" color="secondary" className="fixed-width-button-secondary">
                <a href="/" style={{ textDecoration: "none", color: "inherit" }}>
                  Documentation
                </a>
              </Button>
            </Grid>
          </>
        ) : (
          initializationSteps.map((step, index) => (
            <Grid item xs={12} key={index} style={{ textAlign: "left", width: "200px" }}>
              <pre>
                {step.status === "completed" && "✅ "}
                {step.status === "failed" && "❌ "}
                {step.status === "pending" && "⏳ "}
                {step.name}
              </pre>
            </Grid>
          ))
        )}
        {errorMsg && (
          <div className="error-container">
            <span className="error-message">{errorMsg}</span>
          </div>
        )}
      </Grid>
    </Container>
  );
};

export default HomePage;
