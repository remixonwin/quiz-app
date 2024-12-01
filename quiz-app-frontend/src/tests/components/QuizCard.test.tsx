import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { BrowserRouter } from 'react-router-dom';
import QuizCard from '../../components/QuizCard';
import { Quiz } from '../../types';

// Mock navigation
const mockNavigate = jest.fn();
jest.mock('react-router-dom', () => ({
  ...jest.requireActual('react-router-dom'),
  useNavigate: () => mockNavigate,
}));

describe('QuizCard', () => {
  const mockQuiz: Quiz = {
    id: 1,
    title: 'Test Quiz',
    description: 'Test Description',
    created_by: 1,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
    questions: [],
  };

  const mockOnDelete = jest.fn();

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('renders quiz information correctly', () => {
    render(
      <BrowserRouter>
        <QuizCard quiz={mockQuiz} />
      </BrowserRouter>
    );

    expect(screen.getByText(mockQuiz.title)).toBeInTheDocument();
    if (mockQuiz.description) {
      expect(screen.getByText(mockQuiz.description)).toBeInTheDocument();
    }
    expect(screen.getByText('0 Questions')).toBeInTheDocument();
  });

  it('shows owner actions when isOwner is true', () => {
    render(
      <BrowserRouter>
        <QuizCard quiz={mockQuiz} isOwner={true} onDelete={mockOnDelete} />
      </BrowserRouter>
    );

    expect(screen.getByRole('button', { name: 'Edit Quiz' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Delete Quiz' })).toBeInTheDocument();
  });

  it('does not show owner actions when isOwner is false', () => {
    render(
      <BrowserRouter>
        <QuizCard quiz={mockQuiz} isOwner={false} />
      </BrowserRouter>
    );

    expect(screen.queryByRole('button', { name: 'Edit Quiz' })).not.toBeInTheDocument();
    expect(screen.queryByRole('button', { name: 'Delete Quiz' })).not.toBeInTheDocument();
  });

  it('calls onDelete when delete button is clicked', () => {
    render(
      <BrowserRouter>
        <QuizCard quiz={mockQuiz} isOwner={true} onDelete={mockOnDelete} />
      </BrowserRouter>
    );

    fireEvent.click(screen.getByRole('button', { name: 'Delete Quiz' }));
    expect(mockOnDelete).toHaveBeenCalledWith(mockQuiz.id);
  });

  it('navigates to take quiz page when play button is clicked', () => {
    render(
      <BrowserRouter>
        <QuizCard quiz={mockQuiz} />
      </BrowserRouter>
    );

    fireEvent.click(screen.getByRole('button', { name: 'Take Quiz' }));
    expect(mockNavigate).toHaveBeenCalledWith(`/quiz/take/${mockQuiz.id}`);
  });
});
