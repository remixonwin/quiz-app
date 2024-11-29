import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import {
  Container,
  Grid,
  Card,
  CardContent,
  CardActions,
  Typography,
} from '@mui/material';
import { Button, CircularProgress } from '@mui/material';
import axios from 'axios';
import { Quiz } from '../types';

const QuizList: React.FC = () => {
  const [quizzes, setQuizzes] = useState<Quiz[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchQuizzes = async () => {
      try {
        const response = await axios.get<Quiz[]>('http://localhost:8080/api/quizzes');
        setQuizzes(response.data);
        setLoading(false);
      } catch (err) {
        setError('Failed to fetch quizzes');
        setLoading(false);
      }
    };

    fetchQuizzes();
  }, []);

  if (loading) {
    return (
      <Container style={{ textAlign: 'center', marginTop: '2rem' }}>
        <CircularProgress />
      </Container>
    );
  }

  if (error) {
    return (
      <Container style={{ textAlign: 'center', marginTop: '2rem' }}>
        <Typography color="error">{error}</Typography>
      </Container>
    );
  }

  return (
    <Container maxWidth="md" style={{ marginTop: '2rem' }}>
      <Typography variant="h4" gutterBottom>
        Available Quizzes
      </Typography>
      <Grid container spacing={3}>
        {quizzes.map((quiz) => (
          <Grid item xs={12} sm={6} md={4} key={quiz.id}>
            <Card>
              <CardContent>
                <Typography variant="h6" component="h2">
                  {quiz.title}
                </Typography>
                <Typography color="textSecondary" gutterBottom>
                  {quiz.description}
                </Typography>
              </CardContent>
              <CardActions>
                <Button
                  component={Link}
                  to={`/quiz/${quiz.id}`}
                  color="primary"
                  variant="contained"
                  fullWidth
                >
                  Take Quiz
                </Button>
              </CardActions>
            </Card>
          </Grid>
        ))}
      </Grid>
    </Container>
  );
};

export default QuizList;
