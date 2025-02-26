import React from "react";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import WalletContextProvider from "./components/WalletConnect";

const App: React.FC = () => {
  return (
    <WalletContextProvider>
      <Router>
        <div className="flex flex-col min-h-screen bg-black text-white">
          <p>Hello team...init</p>
        </div>
      </Router>
    </WalletContextProvider>
  );
};

export default App;
