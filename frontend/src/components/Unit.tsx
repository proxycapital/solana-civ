interface UnitProps {
  x: number;
  y: number;
  type: string;
  npc?: boolean | undefined;
  health?: number;
  isSelected: boolean;
  onClick: (x: number, y: number) => void;
}

const Unit: React.FC<UnitProps> = ({ x, y, type, npc, health, isSelected, onClick }) => {
  const handleClick = () => {
    onClick(x, y);
  };

  return (
    <div className={`unit ${isSelected ? "selected" : ""} ${npc ? "npc" : ""}`} onClick={handleClick}>
      {health && health < 100 && (
        <div className="health-bar">
          <div className="health" style={{ width: `${health}%` }}></div>
        </div>
      )}
      <img src={`/${type}.png`} alt={type} />
    </div>
  );
};

export default Unit;
