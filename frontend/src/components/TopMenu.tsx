import React, { useState, useEffect } from "react";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import Switch from "@mui/material/Switch";
import { Connection, clusterApiUrl, PublicKey } from "@solana/web3.js";

interface TopMenuProps {
  debug: boolean;
  setDebug: React.Dispatch<React.SetStateAction<boolean>>;
}

const TopMenu: React.FC<TopMenuProps> = ({ debug, setDebug }) => {
  const gold = 100;
  const food = 50;
  const lumber = 70;

  const [solBalance, setSolBalance] = useState<number | null>(null);

  useEffect(() => {
    const fetchSolBalance = async () => {
      const publicKeyString = localStorage.getItem("solanaWalletPublicKey");
      if (!publicKeyString) return;

      const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
      const publicKey = new PublicKey(publicKeyString);
      try {
        const balance = await connection.getBalance(publicKey);
        setSolBalance(balance / 1e9);
      } catch (error) {
        console.error("Failed to fetch balance", error);
      }
    };

    fetchSolBalance();
  }, []);

  return (
    <div style={{ display: "flex", justifyContent: "space-between", margin: "20px" }}>
      {/* First AppBar for balances */}
      <AppBar
        position="static"
        className="top-navigation"
        style={{ flex: "2", marginRight: "10px", borderRadius: "8px" }}
      >
        <Toolbar>
          <div style={{ display: "flex", alignItems: "center" }}>
            <div style={{ display: "flex", alignItems: "center", marginRight: "20px", padding: "5px 10px", border: "1px solid #3d4a57", borderRadius: "15px" }}>
              <img src="/icons/gold.png" width="32" style={{ marginRight: "5px" }} />
              {gold}
            </div>
            <div style={{ display: "flex", alignItems: "center", marginRight: "20px", padding: "5px 10px", border: "1px solid #3d4a57", borderRadius: "15px" }}>
              <img src="/icons/food.png" width="32" style={{ marginRight: "5px" }} />
              {food}
            </div>
            <div style={{ display: "flex", alignItems: "center", marginRight: "20px", padding: "5px 10px", border: "1px solid #3d4a57", borderRadius: "15px" }}>
              <img src="/icons/lumber.png" width="32" style={{ marginRight: "5px" }} />
              {lumber}
            </div>
            <div style={{ display: "flex", alignItems: "center", marginRight: "20px", padding: "5px 10px", border: "1px solid #3d4a57", borderRadius: "15px" }}>
              <img src="/icons/solana.png" width="32" style={{ marginRight: "5px" }} />
              {solBalance === null ? "Loading..." : solBalance}
            </div>
          </div>
        </Toolbar>
      </AppBar>

      {/* Second AppBar for Debug and End Turn */}
      <AppBar
        position="static"
        className="top-navigation"
        style={{ flex: "1", marginLeft: "10px", borderRadius: "8px" }}
      >
        <Toolbar>
          <div style={{ marginLeft: "auto" }}>
            <Typography variant="h6" style={{ display: "inline" }}>
              Debug:
            </Typography>
            <Switch
              checked={debug}
              onChange={() => setDebug(!debug)}
              name="debug"
              color="default"
              inputProps={{ "aria-label": "Debug mode" }}
            />
            <Typography variant="h6" style={{ display: "inline", marginRight: "20px" }}>
              Day 1
            </Typography>
            <Button variant="contained" color="secondary" className="end-turn-button">
              ⌛ End Turn
            </Button>
          </div>
        </Toolbar>
      </AppBar>
    </div>
  );
};

export default TopMenu;
