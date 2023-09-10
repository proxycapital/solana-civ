import React from 'react';
import Modal from '@mui/material/Modal';
import Box from '@mui/material/Box';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';

interface ConstructionOption {
  title: string;
  description: string;
  cost: number;
  image: string;
}

interface VillageModalProps {
  show: boolean;
  onClose: () => void;
  options: ConstructionOption[];
}

const VillageModal: React.FC<VillageModalProps> = ({ show, onClose, options }) => {
  return (
    <Modal
      open={show}
      onClose={onClose}
      aria-labelledby="village-modal-title"
      aria-describedby="village-modal-description"
    >
      <Box className="modal" sx={{ 
        position: 'absolute', 
        top: '50%', 
        left: '50%', 
        transform: 'translate(-50%, -50%)', 
        width: 600, 
        maxHeight: 400,
        overflow: 'auto',
        bgcolor: 'background.paper', 
        boxShadow: 24, 
        p: 4
      }}>
        {options.map((option, index) => (
          <Box key={index} sx={{ display: 'flex', justifyContent: 'space-between', marginBottom: 2 }}>
            <img src={option.image} alt={option.title} width="100" />
            <div style={{width: '50%'}}>
              <Typography variant="body1">{option.title}</Typography>
              <Typography variant="body2">{option.description}</Typography>
            </div>
            <Typography variant="body2">Cost: {option.cost}</Typography>
            <Button style={{maxHeight: "30px"}} variant="contained" color="primary">Build</Button>
          </Box>
        ))}
      </Box>
    </Modal>
  );
};

export default VillageModal;
