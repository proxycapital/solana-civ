interface UnitProps {
  x: number;
  y: number;
  type: string;
  isSelected: boolean;
  onClick: (x: number, y: number) => void;
}

const Unit: React.FC<UnitProps> = ({ x, y, type, isSelected, onClick }) => {
  const imageMap = {
    worker: 'builder.png',
    warrior: 'warrior.png',
    archer: 'archer.png',
  };

  const handleClick = () => {
    onClick(x, y);
  };

  return (
    <div className={`unit ${isSelected ? 'selected' : ''}`} onClick={handleClick}>
      <img src={`/${imageMap[type as keyof typeof imageMap]}`} alt={type} />
    </div>
  );
};

export default Unit;
