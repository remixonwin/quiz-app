import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import {
  Container,
  Typography,
  Paper,
  RadioGroup,
  FormControlLabel,
  Radio,
  Box,
} from '@mui/material';
import { Button, CircularProgress } from '@mui/material';
import api from '../utils/axios-config'; 
import { Quiz, Question, Answer } from '../types/quiz';

interface QuizWithQuestions extends Quiz {
  questions: Question[];
}

interface QuizTakeProps {
  quiz: QuizWithQuestions;
  onAnswer: (questionId: number, answerId: number) => void;
  selectedAnswers: Record<number, number>;
}

const QuizTake: React.FC<QuizTakeProps> = ({ quiz, onAnswer, selectedAnswers }) => {
  const handleAnswerSelect = (questionId: number, answerId: number) => {
    onAnswer(questionId, answerId);
  };

  return (
    <Box data-testid="quiz-container">
      {quiz.questions.map((question: Question) => (
        <Paper key={question.id} elevation={2} sx={{ p: 3, mb: 2 }}>
          <Typography variant="h6" gutterBottom data-testid="quiz-question">
            {question.question_text}
          </Typography>
          <RadioGroup
            value={selectedAnswers[question.id || 0] || ''}
            onChange={(e) => handleAnswerSelect(question.id || 0, Number(e.target.value))}
          >
            {question.answers.map((answer: Answer) => (
              <FormControlLabel
                key={answer.id}
                value={answer.id}
                control={<Radio />}
                label={answer.answer_text}
                data-testid="answer-option"
              />
            ))}
          </RadioGroup>
        </Paper>
      ))}
    </Box>
  );
};

const QuizTakeContainer: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [quiz, setQuiz] = useState<QuizWithQuestions | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedAnswers, setSelectedAnswers] = useState<Record<number, number>>({});
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState(null);

  useEffect(() => {
    const fetchQuiz = async () => {
      try {
        setLoading(true);
        const response = await api.get<QuizWithQuestions>(`/api/quizzes/${id}`);
        setQuiz(response.data);
      } catch (error) {
        console.error('Error fetching quiz:', error);
        setError(error instanceof Error ? error.message : 'Failed to fetch quiz');
      } finally {
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
      setSubmitting(true);
      const response = await api.post(
        `/api/quizzes/${id}/submit`,
        { answers: selectedAnswers }
      );
      setResult(response.data);
      navigate(`/quiz/${id}/results`, { state: { results: response.data } });
    } catch (error) {
      console.error('Error submitting quiz:', error);
      setError(error instanceof Error ? error.message : 'Failed to submit quiz');
    } finally {
      setSubmitting(false);
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

      <QuizTake quiz={quiz} onAnswer={handleAnswerSelect} selectedAnswers={selectedAnswers} />

      <Button
        variant="contained"
        color="primary"
        fullWidth
        onClick={handleSubmit}
        disabled={quiz.questions.length !== Object.keys(selectedAnswers).length || submitting}
      >
        Submit Quiz
      </Button>
    </Container>
  );
};

export default QuizTakeContainer;
