import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import {
  Container,
  Paper,
  Typography,
  Box,
  Button,
  Divider,
  Chip,
  Grid,
  Skeleton,
} from '@mui/material';
import {
  PlayArrow as PlayIcon,
  Edit as EditIcon,
  Share as ShareIcon,
} from '@mui/icons-material';
import QuizStats from '../components/QuizStats';
import { Quiz } from '../types';
import axios from 'axios';

const QuizDetails: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [quiz, setQuiz] = useState<Quiz | null>(null);
  const [loading, setLoading] = useState(true);

  // Mock stats - replace with actual API data
  const quizStats = {
    totalParticipants: 245,
    averageScore: 82.5,
    averageTime: 15.7,
    completionRate: 91,
  };

  useEffect(() => {
    const fetchQuizDetails = async () => {
      try {
        const response = await axios.get<Quiz>(
          `http://localhost:8080/api/quizzes/${id}`
        );
        setQuiz(response.data);
      } catch (error) {
        console.error('Failed to fetch quiz details:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchQuizDetails();
  }, [id]);

  const handleShare = async () => {
    try {
      await navigator.clipboard.writeText(
        `${window.location.origin}/quiz/take/${id}`
      );
      // Add toast notification here
    } catch (error) {
      console.error('Failed to copy link:', error);
    }
  };

  if (loading) {
    return (
      <Container maxWidth="lg" sx={{ py: 4 }}>
        <Skeleton variant="rectangular" height={200} sx={{ mb: 4 }} />
        <Skeleton variant="rectangular" height={400} />
      </Container>
    );
  }

  if (!quiz) {
    return (
      <Container maxWidth="lg" sx={{ py: 4 }}>
        <Typography variant="h5">Quiz not found</Typography>
      </Container>
    );
  }

  return (
    <Container maxWidth="lg" sx={{ py: 4 }}>
      <Paper elevation={2} sx={{ p: 4, mb: 4 }}>
        <Box
          sx={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'flex-start',
            mb: 3,
          }}
        >
          <Box>
            <Typography variant="h4" component="h1" gutterBottom>
              {quiz.title}
            </Typography>
            <Typography variant="subtitle1" color="text.secondary" paragraph>
              {quiz.description}
            </Typography>
            <Box sx={{ display: 'flex', gap: 1, mb: 2 }}>
              <Chip
                label={`${quiz.questions?.length || 0} Questions`}
                color="primary"
                variant="outlined"
              />
              {quiz.category && (
                <Chip label={quiz.category} color="secondary" variant="outlined" />
              )}
            </Box>
          </Box>
          <Box sx={{ display: 'flex', gap: 1 }}>
            <Button
              variant="contained"
              color="primary"
              startIcon={<PlayIcon />}
              onClick={() => navigate(`/quiz/take/${id}`)}
            >
              Start Quiz
            </Button>
            <Button
              variant="outlined"
              startIcon={<ShareIcon />}
              onClick={handleShare}
            >
              Share
            </Button>
            <Button
              variant="outlined"
              startIcon={<EditIcon />}
              onClick={() => navigate(`/quiz/edit/${id}`)}
            >
              Edit
            </Button>
          </Box>
        </Box>

        <Divider sx={{ my: 3 }} />

        <Grid container spacing={4}>
          <Grid item xs={12}>
            <QuizStats {...quizStats} />
          </Grid>
          
          <Grid item xs={12}>
            <Typography variant="h6" gutterBottom>
              Preview Questions
            </Typography>
            {quiz.questions?.map((question, index) => (
              <Paper
                key={index}
                variant="outlined"
                sx={{ p: 2, mb: 2 }}
              >
                <Typography variant="subtitle1">
                  {index + 1}. {question.text}
                </Typography>
                {!question.is_multiple_choice && (
                  <Typography variant="body2" color="text.secondary">
                    Type: Short Answer
                  </Typography>
                )}
              </Paper>
            ))}
          </Grid>
        </Grid>
      </Paper>
    </Container>
  );
};

export default QuizDetails;
