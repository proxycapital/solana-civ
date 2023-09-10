import React, { useState } from 'react';
import TopMenu from '../components/TopMenu';
import GameMap from '../components/GameMap';
import Console from '../components/Console';

interface Message {
  time: string;
  message: string;
  type?: 'error' | undefined;
}

const GamePage: React.FC = () => {
  const [debug, setDebug] = useState(false);
  const [messages, setMessages] = useState<Array<Message>>([]);

  const logMessage = (message: string, type?: 'error' | undefined) => {
    const now = new Date();
    const time = `${now.getHours()}:${now.getMinutes()}, ${now.getDate()} ${now.toLocaleString('default', { month: 'short' })} ${now.getFullYear()}`;
    
    setMessages(prevMessages => {
      const newMessages = [...prevMessages, { time, message, type }];
      
      // Only keep the last 10 messages
      if (newMessages.length > 10) {
        newMessages.shift();
      }
  
      return newMessages;
    });
  };

  return (
    <div className="full-screen">
      <TopMenu debug={debug} setDebug={setDebug} />
      {debug && <Console messages={messages} />}
      <GameMap debug={debug} logMessage={logMessage} />
    </div>
  );
};

export default GamePage;
