import React, { useState, useEffect } from 'react';

interface TerrainProps {
  x: number;
  y: number;
  type: string;
  isInRange: boolean;
  debug: boolean;
}

function weightedRandomSelect() {
  const weightedImages = [
    1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 4, 4, 4, 4, 5, 6, 7, 8, 8, 8 // 2 and 4 appear more frequently
  ];

  const randomIndex = Math.floor(Math.random() * weightedImages.length);
  return weightedImages[randomIndex];
}

const Terrain: React.FC<TerrainProps> = ({ x, y, type, isInRange, debug }) => {
  const [imageIndex, setImageIndex] = useState<number | null>(null);

  useEffect(() => {
    // Place the city in (1, 1) tile
    if (imageIndex === null) {
      const i = x === y && x === 1 ? 10 : weightedRandomSelect();
      setImageIndex(i);
    }
  }, [imageIndex]);

  return (
    <div>
      { imageIndex !== null && (
        <img src={`/terrain/Layer ${imageIndex}.png`} className="terrain" alt={type} draggable="false" />
      )}
      {debug && <div className="tile-coordinate">{`(${x}, ${y})`}</div>}
    </div>
  );
};

export default Terrain;
