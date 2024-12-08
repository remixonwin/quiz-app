import React from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import Navbar from './components/Navbar';
import Dashboard from './pages/Dashboard';
import QuizDetails from './pages/QuizDetails';
import TakeQuiz from './pages/TakeQuiz';
import QuizResultsPage from './pages/QuizResults';
import QuizCreate from './components/QuizCreate';
import CreateQuiz from './pages/CreateQuiz';
import QuizList from './pages/QuizList';
import Login from './pages/Login';
import Register from './pages/Register';

const PrivateRoute: React.FC<{ element: React.ReactElement }> = ({ element }) => {
  const isAuthenticated = !!localStorage.getItem('token');
  return isAuthenticated ? element : <Navigate to="/login" />;
};

const App: React.FC = () => {
  const isAuthenticated = !!localStorage.getItem('token');

  return (
    <Router>
      <div className="App">
        {isAuthenticated && <Navbar />}
        <main>
          <Routes>
            <Route path="/login" element={<Login />} />
            <Route path="/register" element={<Register />} />
            <Route path="/" element={<PrivateRoute element={<Dashboard />} />} />
            <Route path="/quizzes" element={<PrivateRoute element={<QuizList />} />} />
            <Route path="/create" element={<PrivateRoute element={<CreateQuiz />} />} />
            <Route path="/quiz/create" element={<PrivateRoute element={<QuizCreate />} />} />
            <Route path="/quiz/:id" element={<PrivateRoute element={<QuizDetails />} />} />
            <Route path="/quiz/take/:id" element={<PrivateRoute element={<TakeQuiz />} />} />
            <Route path="/quiz/:id/results" element={<PrivateRoute element={<QuizResultsPage />} />} />
          </Routes>
        </main>
      </div>
    </Router>
  );
};

export default App;
