import React, { useState } from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import HomePage from './pages/HomePage';
import GamePage from './pages/GamePage';
import { GameStateProvider } from './context/GameStateContext';


const App: React.FC = () => {
  return (
    <GameStateProvider>
      <Router>
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/game" element={<GamePage />} />
        </Routes>
      </Router>
    </GameStateProvider>
  );
};

export default App;
