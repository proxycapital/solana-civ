import React, { useState } from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import HomePage from './pages/HomePage';
import GamePage from './pages/GamePage';
import { GameStateProvider } from './context/GameStateContext';
import { WorkspaceProvider } from "./context/AnchorContext";

const App: React.FC = () => {
  return (
    <WorkspaceProvider>
      <GameStateProvider>
        <Router>
          <Routes>
            <Route path="/" element={<HomePage />} />
            <Route path="/game" element={<GamePage />} />
          </Routes>
        </Router>
      </GameStateProvider>
    </WorkspaceProvider>
  );
};

export default App;
