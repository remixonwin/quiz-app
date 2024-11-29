import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Navbar from './components/Navbar';
import Home from './components/Home';
import QuizList from './components/QuizList';
import QuizCreate from './components/QuizCreate';
import QuizTake from './components/QuizTake';

// Ensure future flags are correctly applied
const App: React.FC = () => {
  return (
    <Router future={{ v7_startTransition: true, v7_relativeSplatPath: true }}>
      <div className="App">
        <Navbar />
        <div className="container">
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/quizzes" element={<QuizList />} />
            <Route path="/create" element={<QuizCreate />} />
            <Route path="/quiz/:id" element={<QuizTake />} />
          </Routes>
        </div>
      </div>
    </Router>
  );
};

export default App;
