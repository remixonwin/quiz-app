import React from 'react';
import {
  Box,
  Typography,
  RadioGroup,
  FormControlLabel,
  Radio,
  TextField,
  Paper,
} from '@mui/material';

interface QuestionDisplayProps {
  questionText: string;
  options?: string[];
  isMultipleChoice: boolean;
  currentAnswer: string;
  onAnswerChange: (answer: string) => void;
  questionNumber: number;
}

const QuestionDisplay: React.FC<QuestionDisplayProps> = ({
  questionText,
  options = [],
  isMultipleChoice,
  currentAnswer,
  onAnswerChange,
  questionNumber,
}) => {
  return (
    <Paper elevation={2} sx={{ p: 3, mb: 3 }}>
      <Typography variant="h6" gutterBottom>
        Question {questionNumber}
      </Typography>
      <Typography variant="body1" sx={{ mb: 3 }}>
        {questionText}
      </Typography>

      {isMultipleChoice ? (
        <RadioGroup
          value={currentAnswer}
          onChange={(e) => onAnswerChange(e.target.value)}
        >
          {options.map((option, index) => (
            <FormControlLabel
              key={index}
              value={option}
              control={<Radio />}
              label={option}
            />
          ))}
        </RadioGroup>
      ) : (
        <Box sx={{ mt: 2 }}>
          <TextField
            fullWidth
            multiline
            rows={4}
            variant="outlined"
            value={currentAnswer}
            onChange={(e) => onAnswerChange(e.target.value)}
            placeholder="Type your answer here..."
          />
        </Box>
      )}
    </Paper>
  );
};

export default QuestionDisplay;
