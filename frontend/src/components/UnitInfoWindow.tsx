import React from 'react';

interface UnitInfoProps {
  type: string;
  remainingMoves: number;
  movementRange: number;
  builds?: number;
  strength?: number;
}

const UnitInfoWindow: React.FC<UnitInfoProps> = ({ type, remainingMoves, movementRange, builds, strength }) => {
  const imageMap = {
    'worker': '/builder.png',
    'warrior': '/warrior.png'
  };
  const displayType = type.charAt(0).toUpperCase() + type.slice(1);
  return (
    <div className="unit-info-window">
      <img src={`${imageMap[type as keyof typeof imageMap]}`} alt={type} />
      <div><strong>{displayType}</strong></div>
      <div>HP: 100/100</div>
      <div>Movements: {remainingMoves}/{movementRange}</div>
      {builds !== undefined && <div>Builds: {builds}/1</div>}
      {strength !== undefined && <div>Strength: {strength}</div>}
    </div>
  );
};

export default UnitInfoWindow;