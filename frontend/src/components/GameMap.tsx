import React, { useRef, useEffect, useState } from 'react';
import Terrain from './Terrain';
import Unit from './Unit';
import UnitInfoWindow from './UnitInfoWindow';
import VillageModal from './VillageModal';
import "../App.css";

interface GameMapProps {
  debug: boolean;
  logMessage: (message: string, type?: 'error' | undefined) => void;
}

const GameMap: React.FC<GameMapProps> = ({ debug, logMessage }) => {
  const rows = 20;
  const cols = 20;
  const isDragging = useRef(false);
  const [showVillageModal, setShowVillageModal] = useState(false);
  const [tiles, setTiles] = useState([
    { x: 1, y: 1, type: 'Village' },
  ]);
  const [units, setUnits] = useState([
    { x: 3, y: 3, type: 'worker', isSelected: false, movementRange: 3 },
    { x: 4, y: 3, type: 'warrior', isSelected: false, movementRange: 2 },
  ]);
  const constructionOptions = [
    { title: 'Barracks', description: 'Produces warriors', cost: 100, image: '/barracks.png' },
    // { title: 'Farm', description: 'Increases food production', cost: 50, image: 'https://place-hold.it/100x100' },
    { title: 'Wall', description: 'Enhances defense', cost: 75, image: '/wall.png' },
    { title: 'Warrior', description: 'Basic combat unit', cost: 25, image: '/warrior.png' },
    { title: 'Worker', description: 'Can build and gather resources', cost: 10, image: '/builder.png' },
  ];
  const containerRef = useRef<HTMLDivElement | null>(null);
  let dragStart = { x: 0, y: 0 };

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
    const withinDistance = Math.abs(x1 - x2) <= distance && Math.abs(y1 - y2) <= distance;
    const targetTile = tiles.find(t => t.x === x2 && t.y === y2);
    const blockedTileTypes = ['Village', 'Mountains'];
    if (targetTile && blockedTileTypes.includes(targetTile.type)) {
      return false;
    }
    return withinDistance;
  };
  

  const selectOrMoveUnit = (x: number, y: number, type: string) => {
    const selectedUnit = units.find((unit) => unit.isSelected);
    const isTileOccupied = units.some(unit => unit.x === x && unit.y === y);

    if (selectedUnit && !isTileOccupied && isWithinDistance(selectedUnit.x, selectedUnit.y, x, y, selectedUnit.movementRange)) {
      console.log('Condition 1: Moving unit');
      const newUnits = units.map(unit => {
        if (unit.isSelected) {
          return { ...unit, x, y, isSelected: false };
        }
        return unit;
      });
      logMessage(`Unit ${type} moved to (${x}, ${y})`);
      setUnits(newUnits);
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
          type={selectedUnit.type}
          remainingMoves={selectedUnit.movementRange}
          movementRange={selectedUnit.movementRange}
          builds={selectedUnit.type === 'worker' ? 1 : undefined}
          strength={selectedUnit.type === 'warrior' ? 10 : undefined}
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
          const currentTile = tiles.find(t => t.x === col && t.y === row) || { type: 'Plains', x: col, y: row };
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
              <Terrain x={col} y={row} type={currentTile.type} isInRange={isInRangeForAnyUnit} debug={debug} />
              {currentUnit && <Unit {...currentUnit} onClick={() => selectOrMoveUnit(col, row, currentUnit.type)} />}
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default GameMap;
