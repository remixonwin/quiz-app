import React from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import {
  Container,
  Box,
  Typography,
  Button,
} from '@mui/material';
import {
  Home as HomeIcon,
  List as ListIcon,
} from '@mui/icons-material';
import QuizResults from '../components/QuizResults';

interface QuizResultsPageProps {}

const QuizResultsPage: React.FC<QuizResultsPageProps> = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const { results } = location.state || {
    results: {
      totalQuestions: 0,
      correctAnswers: 0,
      questionResults: [],
    },
  };

  const handleRetry = () => {
    // Navigate back to the quiz with reset state
    navigate(-1);
  };

  if (!results) {
    return (
      <Container maxWidth="md" sx={{ py: 4 }}>
        <Typography variant="h5" align="center">
          No results available
        </Typography>
        <Box sx={{ display: 'flex', justifyContent: 'center', mt: 2 }}>
          <Button
            variant="contained"
            startIcon={<HomeIcon />}
            onClick={() => navigate('/')}
          >
            Go to Home
          </Button>
        </Box>
      </Container>
    );
  }

  return (
    <Container maxWidth="md" sx={{ py: 4 }}>
      <QuizResults
        totalQuestions={results.totalQuestions}
        correctAnswers={results.correctAnswers}
        questionResults={results.questionResults}
        onRetry={handleRetry}
      />

      <Box
        sx={{
          display: 'flex',
          justifyContent: 'center',
          gap: 2,
          mt: 4,
        }}
      >
        <Button
          variant="outlined"
          startIcon={<HomeIcon />}
          onClick={() => navigate('/')}
        >
          Home
        </Button>
        <Button
          variant="outlined"
          startIcon={<ListIcon />}
          onClick={() => navigate('/quizzes')}
        >
          All Quizzes
        </Button>
      </Box>
    </Container>
  );
};

export default QuizResultsPage;
