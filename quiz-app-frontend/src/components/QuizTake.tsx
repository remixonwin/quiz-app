import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import {
  Container,
  Typography,
  Paper,
  RadioGroup,
  FormControlLabel,
  Radio,
} from '@mui/material';
import { Button, CircularProgress } from '@mui/material';
import axios from 'axios';
import { Quiz, Question, Answer } from '../types';

interface QuizWithQuestions extends Quiz {
  questions: (Question & { answers: Answer[] })[];
}

const QuizTake: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [quiz, setQuiz] = useState<QuizWithQuestions | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedAnswers, setSelectedAnswers] = useState<Record<number, number>>({});

  useEffect(() => {
    const fetchQuiz = async () => {
      try {
        const response = await axios.get<QuizWithQuestions>(
          `http://localhost:8080/api/quizzes/${id}`
        );
        setQuiz(response.data);
        setLoading(false);
      } catch (err) {
        setError('Failed to fetch quiz');
        setLoading(false);
      }
    };

    fetchQuiz();
  }, [id]);

  const handleAnswerSelect = (questionId: number, answerId: number) => {
    setSelectedAnswers({
      ...selectedAnswers,
      [questionId]: answerId,
    });
  };

  const handleSubmit = async () => {
    try {
      const submission = {
        quiz_id: Number(id),
        answers: Object.entries(selectedAnswers).map(([questionId, answerId]) => ({
          question_id: Number(questionId),
          answer_id: answerId,
        })),
      };

      const response = await axios.post(
        `http://localhost:8080/api/quizzes/${id}/submit`,
        submission
      );

      // Navigate to results page with the score
      navigate(`/quiz/${id}/results`, { state: { results: response.data } });
    } catch (error) {
      console.error('Failed to submit quiz:', error);
    }
  };

  if (loading) {
    return (
      <Container style={{ textAlign: 'center', marginTop: '2rem' }}>
        <CircularProgress />
      </Container>
    );
  }

  if (error || !quiz) {
    return (
      <Container style={{ textAlign: 'center', marginTop: '2rem' }}>
        <Typography color="error">{error || 'Quiz not found'}</Typography>
      </Container>
    );
  }

  return (
    <Container maxWidth="md" style={{ marginTop: '2rem' }}>
      <Typography variant="h4" gutterBottom>
        {quiz.title}
      </Typography>
      <Typography variant="subtitle1" gutterBottom>
        {quiz.description}
      </Typography>

      {quiz.questions.map((question, index) => (
        <Paper key={question.id} style={{ padding: '2rem', marginBottom: '2rem' }}>
          <Typography variant="h6" gutterBottom>
            Question {index + 1}: {question.question_text}
          </Typography>
          <RadioGroup
            value={selectedAnswers[question.id] || ''}
            onChange={(e) => handleAnswerSelect(question.id, Number(e.target.value))}
          >
            {question.answers.map((answer) => (
              <FormControlLabel
                key={answer.id}
                value={answer.id}
                control={<Radio />}
                label={answer.answer_text}
              />
            ))}
          </RadioGroup>
        </Paper>
      ))}

      <Button
        variant="contained"
        color="primary"
        fullWidth
        onClick={handleSubmit}
        disabled={quiz.questions.length !== Object.keys(selectedAnswers).length}
      >
        Submit Quiz
      </Button>
    </Container>
  );
};

export default QuizTake;
