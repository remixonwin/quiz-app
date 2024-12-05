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
import { Answer } from '../../types/quiz';

interface QuestionDisplayProps {
  questionText: string;
  answers: Answer[];
  isMultipleChoice: boolean;
  currentAnswer?: number;
  onAnswerChange: (answerId: number) => void;
  questionNumber: number;
}

const QuestionDisplay: React.FC<QuestionDisplayProps> = ({
  questionText,
  answers,
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
          onChange={(e) => onAnswerChange(Number(e.target.value))}
        >
          {answers.map((answer) => (
            <FormControlLabel
              key={answer.id}
              value={answer.id}
              control={<Radio />}
              label={answer.answer_text}
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
            value={currentAnswer !== undefined ? String(currentAnswer) : ''}
            onChange={(e) => onAnswerChange(Number(e.target.value))}
            placeholder="Type your answer here..."
          />
        </Box>
      )}
    </Paper>
  );
};

export default QuestionDisplay;
