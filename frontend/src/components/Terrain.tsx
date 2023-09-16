import React from 'react';

interface TerrainProps {
  x: number;
  y: number;
  imageIndex: number;
  isInRange: boolean;
  debug: boolean;
}

// Weighted random index selection for terrain tile images
// Used to initialize the map that will be stored in PDA
export function weightedRandomTile() {
  const weightedIndices = [
    1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 4, 4, 4, 4, 5, 6, 7, 8, 8, 8
  ];
  const randomIndex = Math.floor(Math.random() * weightedIndices.length);
  return weightedIndices[randomIndex];
}

// Mapping of tile indices to their type
// Only "Village" is used for now to display the village modal.
export const TileType = {
  0: "Empty",
  1: "Plains",
  2: "Plains",
  3: "Plains",
  4: "Plains",
  5: "Plains",
  6: "Plains",
  7: "Plains",
  8: "Plains",
  9: "Plains",
  10: "Village",
};

const Terrain: React.FC<TerrainProps> = ({ x, y, imageIndex, isInRange, debug }) => {
  const tileType = TileType[imageIndex as keyof typeof TileType];
  const imageUrl = `/terrain/Layer ${imageIndex}.png`;

  return (
    <div>
      { imageIndex !== null && (
        <img src={imageUrl} className="terrain" alt={tileType} draggable="false" />
      )}
      {debug && <DebugCoordinates x={x} y={y} />}
    </div>
  );
};

// Helper function to display tile coordinates when debug is true
const DebugCoordinates: React.FC<{ x: number, y: number }> = ({ x, y }) => (
  <div className="tile-coordinate">{`(${x}, ${y})`}</div>
);

export default Terrain;