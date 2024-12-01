import React from 'react';
import { render, screen } from '@testing-library/react';
import QuizProgress from '../../components/QuizProgress';

describe('QuizProgress', () => {
  it('renders progress information correctly', () => {
    render(
      <QuizProgress
        currentQuestion={3}
        totalQuestions={10}
        timeRemaining={300}
      />
    );

    expect(screen.getByText('Question 3 of 10')).toBeInTheDocument();
    expect(screen.getByText('Time: 5:00')).toBeInTheDocument();
  });

  it('shows stepper when showStepper is true', () => {
    render(
      <QuizProgress
        currentQuestion={1}
        totalQuestions={5}
        showStepper={true}
      />
    );

    // Check if we have 5 step labels by targeting the MuiStepLabel-label class
    const steps = screen.getAllByText(/^[1-5]$/, { selector: '.MuiStepLabel-label' });
    expect(steps).toHaveLength(5);
  });

  it('changes time color to error when time is low', () => {
    render(
      <QuizProgress
        currentQuestion={1}
        totalQuestions={5}
        timeRemaining={25}
      />
    );

    const timeElement = screen.getByText('Time: 0:25');
    expect(timeElement).toHaveStyle({ color: 'rgb(211, 47, 47)' }); // Material-UI's error color
  });

  it('does not show time when timeRemaining is not provided', () => {
    render(
      <QuizProgress
        currentQuestion={1}
        totalQuestions={5}
      />
    );

    expect(screen.queryByText(/Time:/)).not.toBeInTheDocument();
  });
});
