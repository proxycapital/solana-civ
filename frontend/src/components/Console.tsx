import React from "react";
import "../App.css";

interface ConsoleProps {
  messages: Array<{ time: string; message: string; type?: "error" | undefined }>;
}

const Console: React.FC<ConsoleProps> = ({ messages }) => {
  return (
    <div className="console-container">
      <pre>
        {messages.map((msg, index) => (
          <div
            key={index}
            className={msg.type === "error" ? "console-message console-message-error" : "console-message"}
          >
            [{msg.time}] {msg.message}
          </div>
        ))}
      </pre>
    </div>
  );
};

export default Console;
