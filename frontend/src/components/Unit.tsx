interface UnitProps {
  x: number;
  y: number;
  type: string;
  isSelected: boolean;
  onClick: (x: number, y: number) => void;
}

const Unit: React.FC<UnitProps> = ({ x, y, type, isSelected, onClick }) => {

  const handleClick = () => {
    onClick(x, y);
  };

  return (
    <div className={`unit ${isSelected ? 'selected' : ''}`} onClick={handleClick}>
      <img src={`/${type}.png`} alt={type} />
    </div>
  );
};

export default Unit;
