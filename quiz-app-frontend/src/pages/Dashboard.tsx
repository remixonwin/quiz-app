import React, { useState, useEffect } from 'react';
import {
  Container,
  Grid,
  Typography,
  Box,
  Button,
  TextField,
  InputAdornment,
} from '@mui/material';
import { Add as AddIcon, Search as SearchIcon } from '@mui/icons-material';
import { useNavigate } from 'react-router-dom';
import QuizCard from '../components/QuizCard';
import QuizStats from '../components/QuizStats';
import { Quiz } from '../types';
import axios from 'axios';

const Dashboard: React.FC = () => {
  const navigate = useNavigate();
  const [quizzes, setQuizzes] = useState<Quiz[]>([]);
  const [searchTerm, setSearchTerm] = useState('');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Mock user ID - replace with actual auth
  const currentUserId = 1;

  const stats = {
    totalParticipants: 150,
    averageScore: 75.5,
    averageTime: 12.3,
    completionRate: 85,
  };

  useEffect(() => {
    const fetchQuizzes = async () => {
      try {
        const response = await axios.get<Quiz[]>('http://localhost:8080/api/quizzes');
        setQuizzes(response.data);
        setError(null);
      } catch (error) {
        console.error('Failed to fetch quizzes:', error);
        setError('Failed to load quizzes. Please try again later.');
      } finally {
        setLoading(false);
      }
    };

    fetchQuizzes();
  }, []);

  const handleDelete = async (id: number) => {
    try {
      await axios.delete(`http://localhost:8080/api/quizzes/${id}`);
      setQuizzes(quizzes.filter(quiz => quiz.id !== id));
    } catch (error) {
      console.error('Failed to delete quiz:', error);
    }
  };

  const filteredQuizzes = quizzes.filter(quiz =>
    quiz.title.toLowerCase().includes(searchTerm.toLowerCase())
  );

  return (
    <Container maxWidth="lg" sx={{ py: 4 }}>
      <Box sx={{ mb: 4 }}>
        <QuizStats {...stats} />
      </Box>

      {loading && (
        <Box display="flex" justifyContent="center" my={4}>
          <Typography>Loading quizzes...</Typography>
        </Box>
      )}

      {error && (
        <Box display="flex" justifyContent="center" my={4}>
          <Typography color="error">{error}</Typography>
        </Box>
      )}

      {!loading && !error && (
        <>
          <Box
            sx={{
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
              mb: 3,
            }}
          >
            <Typography variant="h4" component="h1" gutterBottom>
              My Quizzes
            </Typography>
            <Button
              variant="contained"
              color="primary"
              startIcon={<AddIcon />}
              onClick={() => navigate('/quiz/create')}
            >
              Create Quiz
            </Button>
          </Box>

          <TextField
            fullWidth
            variant="outlined"
            placeholder="Search quizzes..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            sx={{ mb: 4 }}
            InputProps={{
              startAdornment: (
                <InputAdornment position="start">
                  <SearchIcon />
                </InputAdornment>
              ),
            }}
          />

          <Grid container spacing={3}>
            {filteredQuizzes.map((quiz) => (
              <Grid item xs={12} sm={6} md={4} key={quiz.id}>
                <QuizCard
                  quiz={quiz}
                  onDelete={handleDelete}
                  isOwner={quiz.created_by === currentUserId}
                />
              </Grid>
            ))}
          </Grid>
        </>
      )}
    </Container>
  );
};

export default Dashboard;
