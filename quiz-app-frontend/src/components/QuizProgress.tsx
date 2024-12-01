import React from 'react';
import {
  Box,
  LinearProgress,
  Typography,
  useTheme,
  Stepper,
  Step,
  StepLabel,
  Paper,
} from '@mui/material';

interface QuizProgressProps {
  currentQuestion: number;
  totalQuestions: number;
  timeRemaining?: number;
  showStepper?: boolean;
}

const QuizProgress: React.FC<QuizProgressProps> = ({
  currentQuestion,
  totalQuestions,
  timeRemaining,
  showStepper = false,
}) => {
  const theme = useTheme();
  const progress = (currentQuestion / totalQuestions) * 100;

  const formatTime = (seconds: number): string => {
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = seconds % 60;
    return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`;
  };

  return (
    <Paper 
      elevation={2} 
      sx={{ 
        p: 2, 
        mb: 3,
        background: theme.palette.background.default 
      }}
    >
      <Box sx={{ width: '100%' }}>
        <Box
          sx={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            mb: 1,
          }}
        >
          <Typography variant="body2" color="text.secondary">
            Question {currentQuestion} of {totalQuestions}
          </Typography>
          {timeRemaining !== undefined && (
            <Typography
              variant="body2"
              color={timeRemaining < 30 ? 'error' : 'text.secondary'}
              sx={{ fontFamily: 'monospace' }}
            >
              Time: {formatTime(timeRemaining)}
            </Typography>
          )}
        </Box>

        <LinearProgress
          variant="determinate"
          value={progress}
          sx={{
            height: 8,
            borderRadius: 4,
            mb: showStepper ? 3 : 0,
            '& .MuiLinearProgress-bar': {
              borderRadius: 4,
            },
          }}
        />

        {showStepper && (
          <Stepper
            activeStep={currentQuestion - 1}
            alternativeLabel
            sx={{ mt: 2 }}
          >
            {Array.from({ length: totalQuestions }, (_, i) => (
              <Step key={i}>
                <StepLabel>{i + 1}</StepLabel>
              </Step>
            ))}
          </Stepper>
        )}
      </Box>
    </Paper>
  );
};

export default QuizProgress;
