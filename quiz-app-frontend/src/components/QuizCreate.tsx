import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  Container,
  Typography,
  Grid,
  TextField,
  Button,
  IconButton,
} from '@mui/material';
import { Add as AddIcon } from '@mui/icons-material';
import axios from 'axios';
import { Quiz, Question } from '../types/quiz';
import { QuestionForm } from './QuestionForm';

const QuizCreate: React.FC = () => {
  const navigate = useNavigate();
  const [quiz, setQuiz] = useState<Quiz>({
    title: '',
    description: '',
    questions: [],
  });

  const addQuestion = () => {
    setQuiz({
      ...quiz,
      questions: [
        ...quiz.questions,
        {
          question_text: '',
          question_type: 'multiple_choice',
          answers: [
            { answer_text: '', is_correct: false },
            { answer_text: '', is_correct: false },
          ],
        },
      ],
    });
  };

  const handleQuestionChange = (index: number, updatedQuestion: Question) => {
    const newQuestions = [...quiz.questions];
    newQuestions[index] = updatedQuestion;
    setQuiz({
      ...quiz,
      questions: newQuestions,
    });
  };

  const removeQuestion = (index: number) => {
    setQuiz({
      ...quiz,
      questions: quiz.questions.filter((_, i) => i !== index),
    });
  };

  const handleSubmit = async () => {
    try {
      await axios.post('/api/quizzes', quiz);
      navigate('/quizzes');
    } catch (error) {
      console.error('Failed to create quiz:', error);
    }
  };

  const isValid = () => {
    return (
      quiz.title.trim() !== '' &&
      quiz.questions.length > 0 &&
      quiz.questions.every(
        (q) =>
          q.question_text.trim() !== '' &&
          q.answers.length >= 2 &&
          q.answers.some((a) => a.is_correct) &&
          q.answers.every((a) => a.answer_text.trim() !== '')
      )
    );
  };

  return (
    <Container maxWidth="md" sx={{ py: 4 }}>
      <Typography variant="h4" component="h1" gutterBottom>
        Create New Quiz
      </Typography>

      <Grid container spacing={3}>
        <Grid item xs={12}>
          <TextField
            fullWidth
            label="Quiz Title"
            value={quiz.title}
            onChange={(e) => setQuiz({ ...quiz, title: e.target.value })}
          />
        </Grid>

        <Grid item xs={12}>
          <TextField
            fullWidth
            label="Description"
            multiline
            rows={3}
            value={quiz.description}
            onChange={(e) => setQuiz({ ...quiz, description: e.target.value })}
          />
        </Grid>

        <Grid item xs={12}>
          <Typography variant="h6" gutterBottom>
            Questions
          </Typography>
          {quiz.questions.map((question, index) => (
            <QuestionForm
              key={index}
              question={question}
              onQuestionChange={(updatedQuestion) =>
                handleQuestionChange(index, updatedQuestion)
              }
              onRemove={() => removeQuestion(index)}
            />
          ))}
        </Grid>

        <Grid item xs={12}>
          <IconButton onClick={addQuestion} color="primary" sx={{ mb: 2 }}>
            <AddIcon /> Add Question
          </IconButton>
        </Grid>

        <Grid item xs={12}>
          <Button
            variant="contained"
            color="primary"
            onClick={handleSubmit}
            disabled={!isValid()}
            fullWidth
          >
            Create Quiz
          </Button>
        </Grid>
      </Grid>
    </Container>
  );
};

export default QuizCreate;
