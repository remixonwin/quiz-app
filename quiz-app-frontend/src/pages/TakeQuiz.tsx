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
  answer_id: number;
  is_correct?: boolean;
}

const TakeQuiz: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [currentQuestionIndex, setCurrentQuestionIndex] = useState(0);
  const [answers, setAnswers] = useState<Answer[]>([]);
  const [timeRemaining] = useState<number>(1800); // 30 minutes

  const { data: quiz, loading, error, refetch } = useDataFetching<Quiz>(
    () => quizService.getQuizDetails(Number(id)),
    [id]
  );

  const handleSubmit = useCallback(async () => {
    if (!quiz?.questions?.length) return;

    try {
      await quizService.submitQuiz({
        quiz_id: Number(id),
        answers,
      });
      navigate('/quiz/results');
    } catch (error) {
      console.error('Failed to submit quiz:', error);
    }
  }, [quiz, id, answers, navigate]);

  const handleAnswer = (answerId: number) => {
    if (!quiz?.questions?.length) return;

    const currentQuestion = quiz.questions[currentQuestionIndex];
    if (!currentQuestion?.id) return;

    const question_id = currentQuestion.id;
    setAnswers((prev) => {
      const newAnswers = [...prev];
      const existingIndex = newAnswers.findIndex(
        (a) => a.question_id === question_id
      );

      if (existingIndex >= 0) {
        newAnswers[existingIndex].answer_id = answerId;
      } else {
        newAnswers.push({ question_id, answer_id: answerId });
      }

      return newAnswers;
    });
  };

  const getCurrentAnswer = (): number | undefined => {
    if (!quiz?.questions?.length) return undefined;
    
    const currentQuestion = quiz.questions[currentQuestionIndex];
    if (!currentQuestion?.id) return undefined;

    const question_id = currentQuestion.id;
    return answers.find((a) => a.question_id === question_id)?.answer_id;
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

  if (!quiz || !quiz.questions?.length) {
    return <ErrorMessage message="No questions found in this quiz" />;
  }

  const currentQuestion = quiz.questions[currentQuestionIndex];

  return (
    <Container maxWidth="md">
      <Box sx={{ my: 4 }}>
        <QuizProgress
          currentQuestion={currentQuestionIndex + 1}
          totalQuestions={quiz.questions.length}
          timeRemaining={timeRemaining}
        />

        <QuestionDisplay
          questionText={currentQuestion.question_text}
          answers={currentQuestion.answers}
          isMultipleChoice={currentQuestion.question_type === 'multiple_choice'}
          currentAnswer={getCurrentAnswer()}
          onAnswerChange={handleAnswer}
          questionNumber={currentQuestionIndex + 1}
        />

        <Box sx={{ display: 'flex', justifyContent: 'space-between', mt: 2 }}>
          <Button
            onClick={handlePrev}
            disabled={currentQuestionIndex === 0}
            variant="contained"
          >
            Previous
          </Button>

          {currentQuestionIndex === quiz.questions.length - 1 ? (
            <Button
              onClick={handleSubmit}
              variant="contained"
              color="primary"
              disabled={answers.length !== quiz.questions.length}
            >
              Submit Quiz
            </Button>
          ) : (
            <Button
              onClick={handleNext}
              variant="contained"
              endIcon={<NextIcon />}
            >
              Next
            </Button>
          )}
        </Box>
      </Box>
    </Container>
  );
};

export default TakeQuiz;
