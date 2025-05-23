import React from 'react';
import ConfigViewer from './components/ConfigViewer';
import './App.css';

function App() {
    return (
        <div className="App">
            <header className="App-header">
                <h1>Der Nebenkosten-Knecht</h1>
            </header>
            <main>
                <ConfigViewer />
            </main>
        </div>
    );
}

export default App;