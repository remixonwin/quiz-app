import React, { useState, useCallback } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import {
  Container,
  Box,
  Button,
} from '@mui/material';
import { NavigateNext as NextIcon } from '@mui/icons-material';
import QuizProgress from '../components/QuizProgress';
import QuestionDisplay from '../components/common/QuestionDisplay';
import LoadingSpinner from '../components/common/LoadingSpinner';
import ErrorMessage from '../components/common/ErrorMessage';
import useDataFetching from '../hooks/useDataFetching';
import { quizService } from '../services/api';
import { Quiz } from '../types';

interface Answer {
  question_id: number;
  answer_text: string;
  selected_answer?: string;
}

interface SubmittedAnswer {
  question_id: number;
  answer_id: number;
}

const TakeQuiz: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [currentQuestionIndex, setCurrentQuestionIndex] = useState(0);
  const [answers, setAnswers] = useState<Answer[]>([]);
  const [timeRemaining] = useState<number>(600); // Remove unused setter

  const { data: quiz, loading, error, refetch } = useDataFetching<Quiz>(
    () => quizService.getQuizDetails(Number(id)),
    [id]
  );

  const handleSubmit = useCallback(async () => {
    if (!quiz?.questions?.length) return;

    try {
      const submittedAnswers: SubmittedAnswer[] = answers.map(answer => ({
        question_id: answer.question_id,
        answer_id: parseInt(answer.selected_answer || '0'),
      }));

      await quizService.submitQuiz({
        quiz_id: Number(id),
        answers: submittedAnswers,
      });
      navigate('/quiz/results');
    } catch (error) {
      console.error('Failed to submit quiz:', error);
    }
  }, [quiz, id, answers, navigate]);

  const handleAnswer = (answer: string) => {
    if (!quiz?.questions?.length) return;

    const currentQuestion = quiz.questions[currentQuestionIndex];
    if (!currentQuestion?.id) return;

    setAnswers((prev) => {
      const existingIndex = prev.findIndex(
        (a) => a.question_id === currentQuestion.id
      );

      const newAnswer = {
        question_id: currentQuestion.id,
        answer_text: currentQuestion.question_text,
        selected_answer: answer,
      };

      if (existingIndex >= 0) {
        const newAnswers = [...prev];
        newAnswers[existingIndex] = newAnswer;
        return newAnswers;
      }
      return [...prev, newAnswer];
    });
  };

  const getCurrentAnswer = (): string => {
    if (!quiz?.questions?.length) return '';
    
    const currentQuestion = quiz.questions[currentQuestionIndex];
    if (!currentQuestion?.id) return '';

    return answers.find((a) => a.question_id === currentQuestion.id)?.selected_answer || '';
  };

  const handleNext = () => {
    if (!quiz?.questions?.length) return;
    if (currentQuestionIndex < quiz.questions.length - 1) {
      setCurrentQuestionIndex((prev) => prev + 1);
    }
  };

  const handlePrev = () => {
    if (currentQuestionIndex > 0) {
      setCurrentQuestionIndex((prev) => prev - 1);
    }
  };

  if (loading) {
    return <LoadingSpinner message="Loading quiz..." />;
  }

  if (error) {
    return <ErrorMessage message={error} onRetry={refetch} />;
  }

  if (!quiz) {
    return <ErrorMessage message="Quiz not found" />;
  }

  if (!quiz.questions?.length) {
    return <ErrorMessage message="No questions available for this quiz" />;
  }

  const currentQuestion = quiz.questions[currentQuestionIndex];
  if (!currentQuestion) return null;

  return (
    <Container maxWidth="md" sx={{ py: 4 }}>
      <QuizProgress
        currentQuestion={currentQuestionIndex + 1}
        totalQuestions={quiz.questions.length}
        timeRemaining={timeRemaining}
        showStepper
      />

      <QuestionDisplay
        questionText={currentQuestion.text}
        options={currentQuestion.options}
        isMultipleChoice={currentQuestion.is_multiple_choice}
        currentAnswer={getCurrentAnswer()}
        onAnswerChange={handleAnswer}
        questionNumber={currentQuestionIndex + 1}
        totalQuestions={quiz.questions.length}
      />

      <Box
        sx={{
          display: 'flex',
          justifyContent: 'space-between',
          mt: 4,
        }}
      >
        <Button
          variant="contained"
          color="secondary"
          onClick={handlePrev}
          disabled={currentQuestionIndex === 0}
        >
          Previous
        </Button>
        
        {currentQuestionIndex === quiz.questions.length - 1 ? (
          <Button
            variant="contained"
            color="primary"
            onClick={handleSubmit}
            disabled={answers.length !== quiz.questions.length}
          >
            Submit Quiz
          </Button>
        ) : (
          <Button
            variant="contained"
            color="primary"
            onClick={handleNext}
            endIcon={<NextIcon />}
          >
            Next
          </Button>
        )}
      </Box>
    </Container>
  );
};

export default TakeQuiz;
