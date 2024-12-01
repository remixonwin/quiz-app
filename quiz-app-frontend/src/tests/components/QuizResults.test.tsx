import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { BrowserRouter } from 'react-router-dom';
import QuizResults from '../../components/QuizResults';

describe('QuizResults', () => {
  const mockResults = {
    totalQuestions: 5,
    correctAnswers: 3,
    questionResults: [
      {
        question: 'What is 2+2?',
        userAnswer: '4',
        correctAnswer: '4',
        isCorrect: true,
      },
      {
        question: 'What is the capital of France?',
        userAnswer: 'London',
        correctAnswer: 'Paris',
        isCorrect: false,
      },
    ],
  };

  const mockOnRetry = jest.fn();

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('renders score and summary correctly', () => {
    render(
      <BrowserRouter>
        <QuizResults {...mockResults} onRetry={mockOnRetry} />
      </BrowserRouter>
    );

    expect(screen.getByText('60%')).toBeInTheDocument();
    expect(screen.getByText('You got 3 out of 5 questions correct')).toBeInTheDocument();
  });

  it('displays correct and incorrect answers properly', () => {
    render(
      <BrowserRouter>
        <QuizResults {...mockResults} onRetry={mockOnRetry} />
      </BrowserRouter>
    );

    expect(screen.getByText('What is 2+2?')).toBeInTheDocument();
    expect(screen.getByText('Correct: 4')).toBeInTheDocument();
    
    expect(screen.getByText('What is the capital of France?')).toBeInTheDocument();
    expect(screen.getByText('Your answer: London')).toBeInTheDocument();
    expect(screen.getByText('Correct answer: Paris')).toBeInTheDocument();
  });

  it('calls onRetry when retry button is clicked', () => {
    render(
      <BrowserRouter>
        <QuizResults {...mockResults} onRetry={mockOnRetry} />
      </BrowserRouter>
    );

    fireEvent.click(screen.getByText('Try Again'));
    expect(mockOnRetry).toHaveBeenCalled();
  });

  it('shows appropriate score color based on performance', () => {
    const highScore = {
      ...mockResults,
      correctAnswers: 5,
      totalQuestions: 5,
    };

    const { rerender } = render(
      <BrowserRouter>
        <QuizResults {...highScore} onRetry={mockOnRetry} />
      </BrowserRouter>
    );

    // Check high score color
    expect(screen.getByText('100%')).toHaveStyle({
      color: 'rgb(46, 125, 50)', // Material-UI's success.main color
    });

    // Check low score color
    const lowScore = {
      ...mockResults,
      correctAnswers: 1,
      totalQuestions: 5,
    };

    rerender(
      <BrowserRouter>
        <QuizResults {...lowScore} onRetry={mockOnRetry} />
      </BrowserRouter>
    );

    expect(screen.getByText('20%')).toHaveStyle({
      color: 'rgb(211, 47, 47)', // Material-UI's error.main color
    });
  });
});
