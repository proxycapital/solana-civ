import React from "react";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import Switch from "@mui/material/Switch";
import EndTurnButton from "./EndTurnButton";
import { useGameState } from '../context/GameStateContext';

interface TopMenuProps {
  debug: boolean;
  setDebug: React.Dispatch<React.SetStateAction<boolean>>;
}

const TopMenu: React.FC<TopMenuProps> = ({ debug, setDebug }) => {
  const { resources, game } = useGameState();

  return (
    <div style={{ display: "flex", justifyContent: "space-between", margin: "20px" }}>
      {/* First AppBar for balances */}
      <AppBar
        position="static"
        className="top-navigation"
        style={{ flex: "2", marginRight: "10px", borderRadius: "8px" }}
      >
        <Toolbar>
          <div className="balance-container">
            <div className="balance-box">
              <img src="/icons/gold.png" width="32" alt="Gold" />
              {resources.gold}
            </div>
            <div className="balance-box">
              <img src="/icons/food.png" width="32" alt="Food" />
              {resources.food}
            </div>
            <div className="balance-box">
              <img src="/icons/lumber.png" width="32" alt="Lumber" />
              {resources.wood}
            </div>
            <div className="balance-box">
              <img src="/icons/solana.png" width="32" alt="SOL" />
              {resources.sol}
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
            {/* <Typography variant="h6" style={{ display: "inline" }}>
              Debug:
            </Typography> */}
            <Switch
              checked={debug}
              onChange={() => setDebug(!debug)}
              name="debug"
              color="default"
              inputProps={{ "aria-label": "Debug mode" }}
            />
            <Typography variant="h6" style={{ display: "inline", marginRight: "20px" }}>
              Day {game.turn}
            </Typography>
            <EndTurnButton />
          </div>
        </Toolbar>
      </AppBar>
    </div>
  );
};

export default TopMenu;
