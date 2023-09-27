import React, { createContext, useContext, useState, useEffect } from "react";
import { useWorkspace } from '../context/AnchorContext';
import { getPlayer, getGame, getNpcs } from '../utils/solanaUtils';

type Game = {
  turn: number,
  map: number[],
}

type Resources = {
  gold: number;
  food: number;
  wood: number;
  stone: number;
  iron: number;
  sol: number;
};

interface GameStateContextType {
  fetchPlayerState: () => Promise<void>;
  fetchGameState: () => Promise<void>;
  fetchNpcs: () => Promise<void>;
  game: Game,
  cities: any[];
  upgradedTiles: any[];
  npcUnits: any[];
  resources: Resources,
  allUnits: any[];
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
  const { program, provider } = useWorkspace();
  const [resources, setResources] = useState({} as Resources);
  const [game, setGame] = useState({turn: 1, map: []} as Game);
  const [cities, setCities] = useState([] as any[]);
  const [upgradedTiles, setUpgradedTiles] = useState([] as any[]);
  const [allUnits, setUnits] = useState([] as any[]);
  const [npcUnits, setNpcUnits] = useState([] as any[]);

  // const updateUnits = (updatedUnits: any[]) => setUnits(updatedUnits);

  const fetchGameState = async () => {
    try {
      const game = await getGame(provider, program);
      if (game) {
        setGame(game);
      }
    } catch (error) {
      console.error('Failed to fetch game state', error);
    }
  };

  const fetchNpcs = async () => {
    try {
      const npcs = await getNpcs(provider, program);
      if (npcs) {
        setNpcUnits(npcs.units);
      }
    } catch (error) {
      console.error('Failed to fetch npcs', error);
    }
  }

  const fetchPlayerState = async () => {
    try {
      const player = await getPlayer(provider, program);
      console.log('[GameStateProvider] fetchPlayerState()', player);
      if (player && player.balances) {
        // setSol(player.balances.sol);
        setResources(player.balances);
      }
      if (player && player.units) {
        setUnits(player.units);
      }
      if (player && player.cities) {
        setCities(player.cities);
      }
      if (player && player.tiles) {
        setUpgradedTiles(player.tiles);
      }
    } catch (error) {
      console.error('Failed to fetch balance', error);
      // @todo: alert for player ?
    }
  };

  useEffect(() => {
    fetchGameState();
    fetchPlayerState();
    fetchNpcs();
  }, []);

  return (
    <GameStateContext.Provider value={{ fetchPlayerState, fetchGameState, fetchNpcs, game, cities, upgradedTiles, resources, npcUnits, allUnits }}>
      {children}
    </GameStateContext.Provider>
  );
};
