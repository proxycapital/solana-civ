import React, { createContext, useContext, useState } from "react";
import { fetchBalances } from '../utils/solanaUtils';


// @todo: move this to a environment.ts or other single point hook
const { REACT_APP_RPC: RPC, REACT_APP_PROGRAM_ID: PROGRAM_ADDRESS } = process.env;
if (!PROGRAM_ADDRESS) {
  throw new Error("REACT_APP_PROGRAM_ID is undefined. Hint: `cp .env.sample .env`");
}

interface GameStateContextType {
  updateBalances: () => Promise<void>;
  gold: number | null;
  food: number | null;
  lumber: number | null;
  sol: number | null;
}

interface BaseLayoutProps {
  children?: React.ReactNode;
}

const GameStateContext = createContext<GameStateContextType | undefined>(undefined);

export const useGameState = () => {
  const context = useContext(GameStateContext);
  if (!context) {
    throw new Error("useGameState must be used within a GameStateProvider");
  }
  return context;
};

export const GameStateProvider: React.FC<BaseLayoutProps> = ({ children }) => {
  const [gold, setGold] = useState(0);
  const [food, setFood] = useState(0);
  const [lumber, setLumber] = useState(0);
  const [sol, setSol] = useState(0);

  const updateBalances = async () => {
    try {
      const balance = await fetchBalances();
      if (balance) {
        setSol(balance.sol);
        setGold(balance.gold);
        setFood(balance.food);
        setLumber(balance.lumber);
      }
    } catch (error) {
      console.error('Failed to fetch balance', error);
      // @todo: alert for player ?
    }
  };

  return (
    <GameStateContext.Provider value={{ updateBalances, gold, food, lumber, sol }}>
      {children}
    </GameStateContext.Provider>
  );
};
