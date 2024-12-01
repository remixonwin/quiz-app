import React from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  CircularProgress,
  Button,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  Divider,
} from '@mui/material';
import {
  CheckCircle as CorrectIcon,
  Cancel as WrongIcon,
  Replay as RetryIcon,
  Home as HomeIcon,
} from '@mui/icons-material';
import { useNavigate } from 'react-router-dom';

interface QuizResultsProps {
  totalQuestions: number;
  correctAnswers: number;
  questionResults: Array<{
    question: string;
    userAnswer: string;
    correctAnswer: string;
    isCorrect: boolean;
  }>;
  onRetry?: () => void;
}

const QuizResults: React.FC<QuizResultsProps> = ({
  totalQuestions,
  correctAnswers,
  questionResults,
  onRetry,
}) => {
  const navigate = useNavigate();
  const score = (correctAnswers / totalQuestions) * 100;

  const getScoreColor = (score: number) => {
    if (score >= 80) return 'success.main';
    if (score >= 60) return 'warning.main';
    return 'error.main';
  };

  return (
    <Box sx={{ maxWidth: 800, mx: 'auto', p: 3 }} data-testid="quiz-results">
      <Card>
        <CardContent>
          <Box
            display="flex"
            flexDirection="column"
            alignItems="center"
            mb={4}
          >
            <Box position="relative" display="inline-flex" mb={2}>
              <CircularProgress
                variant="determinate"
                value={score}
                size={120}
                thickness={4}
                sx={{ color: getScoreColor(score) }}
              />
              <Box
                sx={{
                  top: 0,
                  left: 0,
                  bottom: 0,
                  right: 0,
                  position: 'absolute',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                }}
              >
                <Typography
                  variant="h4"
                  component="div"
                  color={getScoreColor(score)}
                  data-testid="quiz-score"
                >
                  {Math.round(score)}%
                </Typography>
              </Box>
            </Box>
            <Typography variant="h5" gutterBottom align="center" data-testid="correct-answers">
              You got {correctAnswers} out of {totalQuestions} questions correct
            </Typography>
          </Box>

          <Divider sx={{ my: 3 }} />

          <List>
            {questionResults.map((result, index) => (
              <React.Fragment key={index}>
                <ListItem data-testid={`question-result-${index}`}>
                  <ListItemIcon>
                    {result.isCorrect ? (
                      <CorrectIcon color="success" />
                    ) : (
                      <WrongIcon color="error" />
                    )}
                  </ListItemIcon>
                  <ListItemText
                    primary={result.question}
                    secondary={
                      <>
                        <Typography component="span" color={result.isCorrect ? 'success.main' : 'error.main'}>
                          Your answer: {result.userAnswer}
                        </Typography>
                        {!result.isCorrect && (
                          <Typography component="span" color="text.secondary">
                            <br />
                            Correct answer: {result.correctAnswer}
                          </Typography>
                        )}
                      </>
                    }
                  />
                </ListItem>
                <Divider />
              </React.Fragment>
            ))}
          </List>

          <Box
            sx={{
              display: 'flex',
              justifyContent: 'center',
              gap: 2,
              mt: 4,
            }}
          >
            {onRetry && (
              <Button
                variant="contained"
                color="primary"
                startIcon={<RetryIcon />}
                onClick={onRetry}
                data-testid="retry-quiz"
              >
                Try Again
              </Button>
            )}
            <Button
              variant="outlined"
              startIcon={<HomeIcon />}
              onClick={() => navigate('/')}
              data-testid="back-to-home"
            >
              Back to Home
            </Button>
          </Box>
        </CardContent>
      </Card>
    </Box>
  );
};

export default QuizResults;
