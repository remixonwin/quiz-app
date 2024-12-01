import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Navbar from './components/Navbar';
import Dashboard from './pages/Dashboard';
import QuizDetails from './pages/QuizDetails';
import TakeQuiz from './pages/TakeQuiz';
import QuizResultsPage from './pages/QuizResults';
import QuizCreate from './components/QuizCreate';

const App: React.FC = () => {
  return (
    <Router>
      <div className="App">
        <Navbar />
        <main>
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/quiz/create" element={<QuizCreate />} />
            <Route path="/quiz/:id" element={<QuizDetails />} />
            <Route path="/quiz/take/:id" element={<TakeQuiz />} />
            <Route path="/quiz/:id/results" element={<QuizResultsPage />} />
          </Routes>
        </main>
      </div>
    </Router>
  );
};

export default App;
