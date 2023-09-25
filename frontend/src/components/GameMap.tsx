import React, { useRef, useEffect, useState } from 'react';
import * as anchor from "@coral-xyz/anchor";
import Terrain, { TileType } from './Terrain';
import Unit from './Unit';
import UnitInfoWindow from './UnitInfoWindow';
import VillageModal from './VillageModal';
import { useGameState } from '../context/GameStateContext';
import { useWorkspace } from '../context/AnchorContext';
import { getMap } from '../utils/solanaUtils';
import "../App.css";

interface GameMapProps {
  debug: boolean;
  logMessage: (message: string, type?: 'error' | undefined) => void;
}

interface Tile {
  x: number;
  y: number;
  imageIndex: number;
  type: string;
}

const GameMap: React.FC<GameMapProps> = ({ debug, logMessage }) => {
  const rows = 20;
  const cols = 20;
  const isDragging = useRef(false);
  const [showVillageModal, setShowVillageModal] = useState(false);
  const { fetchPlayerState, updateUnits, allUnits } = useGameState();
  const { program, provider } = useWorkspace();

  const [tiles, setTiles] = useState([] as Tile[]);
  const [units, setUnits] = useState<Unit[]>(allUnits);

  interface Unit {
    unitId: number;
    x: number;
    y: number;
    type: string;
    isSelected: boolean;
    movementRange: number;
  }

  const constructionOptions = [
    { title: 'Barracks', description: 'Produces warriors', cost: 100, image: '/barracks.png' },
    // { title: 'Farm', description: 'Increases food production', cost: 50, image: 'https://place-hold.it/100x100' },
    { title: 'Wall', description: 'Enhances defense', cost: 75, image: '/wall.png' },
    { title: 'Warrior', description: 'Basic combat unit', cost: 25, image: '/warrior.png' },
    { title: 'Worker', description: 'Can build and gather resources', cost: 10, image: '/builder.png' },
  ];
  const containerRef = useRef<HTMLDivElement | null>(null);
  let dragStart = { x: 0, y: 0 };

  useEffect(() => {
    const updatedUnits = allUnits.map(unit => ({ ...unit, isSelected: false, type: Object.keys(unit.unitType)[0] }));
    setUnits(updatedUnits);
  }, [allUnits]);
  
  useEffect(() => {

    (async () => {
      const map = await getMap(provider, program);
      if (!map) {
        return;
      }
      let newTiles = [];
    
      for (let row = 0; row < 20; row++) {
        for (let col = 0; col < 20; col++) {
          const index = row * 20 + col;
          const tile = map[index];
          if (tile) {
            newTiles.push({ x: col, y: row, imageIndex: tile, type: TileType[tile as keyof typeof TileType] });
          } else {
            console.error('No tile at', col, row);
          }
        }
      }
      
      setTiles(newTiles);
    })();
    
  }, []);

  const startDrag = (event: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
    event.preventDefault();
    isDragging.current = true;
    dragStart.x = event.clientX;
    dragStart.y = event.clientY;
  };

  const whileDrag = (event: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
    if (isDragging.current && containerRef.current) {
      const dx = event.clientX - dragStart.x;
      const dy = event.clientY - dragStart.y;
      containerRef.current.scrollLeft -= dx;
      containerRef.current.scrollTop -= dy;
      dragStart = { x: event.clientX, y: event.clientY };
    }
  };

  const endDrag = () => {
    isDragging.current = false;
  };

  const isInRange = (unit: any, x: number, y: number) => {
    // do not consider "in range" the tile with the selected unit
    if (unit.x === x && unit.y === y) {
      return false;
    }
    return unit.isSelected && isWithinDistance(unit.x, unit.y, x, y, unit.movementRange);
  };

  const isWithinDistance = (x1: number, y1: number, x2: number, y2: number, distance: number) => {
    // const withinDistance = Math.abs(x1 - x2) <= distance && Math.abs(y1 - y2) <= distance;
    const withinDistance = (Math.abs(x1 - x2) + Math.abs(y1 - y2)) <= distance;
    const targetTile = tiles.find(t => t.x === x2 && t.y === y2);
    const blockedTileTypes = ['Village', 'Mountains'];
    if (targetTile && blockedTileTypes.includes(targetTile.type)) {
      return false;
    }
    return withinDistance;
  };

  const selectOrMoveUnit = async (x: number, y: number, type: string) => {
    const selectedUnit = units.find((unit) => unit.isSelected);
    const isTileOccupied = units.some(unit => unit.x === x && unit.y === y);

    if (selectedUnit && !isTileOccupied && isWithinDistance(selectedUnit.x, selectedUnit.y, x, y, selectedUnit.movementRange)) {
      console.log('Condition 1: Moving unit');
      const [gameKey] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("GAME"), provider!.publicKey.toBuffer()],
        program!.programId
      );
      const [playerKey] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("PLAYER"), gameKey.toBuffer(), provider!.publicKey.toBuffer()],
        program!.programId
      );
      const accounts = {
        playerAccount: playerKey,
        player: provider!.publicKey,
      };
      try {
        const tx = await program!.methods.moveUnit(selectedUnit.unitId, x, y).accounts(accounts).rpc();
        console.log(`Move unit TX: https://explorer.solana.com/tx/${tx}?cluster=devnet`);
        logMessage(`Unit #${selectedUnit.unitId} ${type} moved to (${x}, ${y})`);
      } catch (error) {
        console.error('Failed to move unit', error);
      }
      await fetchPlayerState();

    } else {
      console.log('Condition 2: Selecting unit');
      const newUnits = units.map(unit => {
        if (unit.x === x && unit.y === y && unit.type === type) {
          return { ...unit, isSelected: !unit.isSelected };
        } else {
          return { ...unit, isSelected: false };
        }
      });
      setUnits(newUnits);
    }
  };

  const handleTileClick = (col: number, row: number) => {
    const tile = tiles.find(t => t.x === col && t.y === row);
    if (tile && tile.type === 'Village') {
      setShowVillageModal(true);
    }
  };

  const selectedUnit = units.find(unit => unit.isSelected);

  return (
    <div className="game-container" ref={containerRef}>
      <VillageModal show={showVillageModal} onClose={() => setShowVillageModal(false)} options={constructionOptions} />
      {selectedUnit && (
        <UnitInfoWindow
          unit={selectedUnit}
          // type={selectedUnit.type}
          // remainingMoves={selectedUnit.movementRange}
          // movementRange={selectedUnit.movementRange}
          // builds={selectedUnit.type === 'worker' ? 1 : undefined}
          // strength={selectedUnit.type === 'warrior' ? 10 : undefined}
        />
      )}
      <div
        className={`game-map no-select`}
        onMouseDown={startDrag}
        onMouseMove={whileDrag}
        onMouseUp={endDrag}
        onMouseLeave={endDrag}
      >
        {Array.from({ length: rows * cols }, (_, index) => {
          const row = Math.floor(index / cols);
          const col = index % cols;
          /* render the tile or default Plains */
          const currentTile = tiles.find(t => t.x === col && t.y === row) || { imageIndex: 0, type: 'Plains', x: col, y: row };
          const currentUnit = units.find(u => u.x === col && u.y === row);
          const isInRangeForAnyUnit = units.some(u => isInRange(u, col, row));

          return (
            <div 
              key={index} 
              className={`game-tile ${isInRangeForAnyUnit ? 'in-range' : ''}`} 
              onClick={() => {
                console.log(`Tile clicked at ${col}, ${row}`);
                handleTileClick(col, row);
                const selectedUnit = units.find(u => u.isSelected);
                if (!currentUnit && !selectedUnit) {
                  return;
                }
                selectOrMoveUnit(col, row, currentUnit?.type || selectedUnit?.type || 'unknown');
            }}
            >
              <Terrain x={col} y={row} imageIndex={currentTile.imageIndex} isInRange={isInRangeForAnyUnit} debug={debug} />
              {currentUnit && <Unit {...currentUnit} onClick={() => selectOrMoveUnit(col, row, currentUnit.type)} />}
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default GameMap;
