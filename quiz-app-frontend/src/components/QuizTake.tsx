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
import axios from 'axios';
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

      <QuizTake quiz={quiz} onAnswer={handleAnswerSelect} selectedAnswers={selectedAnswers} />

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

export default QuizTakeContainer;
