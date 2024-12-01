import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { act } from 'react-dom/test-utils';
import { BrowserRouter } from 'react-router-dom';
import axios from 'axios';
import TakeQuiz from '../../pages/TakeQuiz';

// Mock axios
jest.mock('axios');
const mockedAxios = axios as jest.Mocked<typeof axios>;

// Mock navigation and params
const mockNavigate = jest.fn();
jest.mock('react-router-dom', () => ({
  ...jest.requireActual('react-router-dom'),
  useNavigate: () => mockNavigate,
  useParams: () => ({ id: '1' }),
}));

// Mock timer
jest.mock('react', () => ({
  ...jest.requireActual('react'),
  useEffect: jest.requireActual('react').useEffect,
  useState: jest.requireActual('react').useState,
}));

describe('TakeQuiz', () => {
  const mockQuiz = {
    id: 1,
    title: 'Test Quiz',
    questions: [
      {
        id: 1,
        text: 'What is 2+2?',
        is_multiple_choice: true,
        options: ['3', '4', '5', '6'],
        correct_answer: '4',
      },
      {
        id: 2,
        text: 'Describe the water cycle',
        is_multiple_choice: false,
      },
    ],
  };

  beforeEach(() => {
    jest.clearAllMocks();
    mockedAxios.get.mockResolvedValue({ data: mockQuiz });
  });

  const renderTakeQuiz = async () => {
    let rendered;
    await act(async () => {
      rendered = render(
        <BrowserRouter>
          <TakeQuiz />
        </BrowserRouter>
      );
      // Wait for initial data fetch
      await new Promise(resolve => setTimeout(resolve, 0));
    });
    return rendered;
  };

  it('loads and displays quiz questions', async () => {
    await renderTakeQuiz();

    await waitFor(() => {
      expect(screen.getByText('What is 2+2?')).toBeInTheDocument();
      expect(screen.getByText('Question 1 of 2')).toBeInTheDocument();
    });
  });

  it('handles multiple choice questions correctly', async () => {
    await renderTakeQuiz();

    await waitFor(() => {
      expect(screen.getByText('What is 2+2?')).toBeInTheDocument();
    });

    const option = screen.getByLabelText('4');
    await act(async () => {
      fireEvent.click(option);
    });

    expect(option).toBeChecked();
  });

  it('handles text input questions correctly', async () => {
    await renderTakeQuiz();

    await waitFor(() => {
      expect(screen.getByText('What is 2+2?')).toBeInTheDocument();
    });

    await act(async () => {
      fireEvent.click(screen.getByText('Next'));
    });

    const textInput = screen.getByPlaceholderText('Type your answer here...');
    await act(async () => {
      fireEvent.change(textInput, { target: { value: 'Water evaporates and forms clouds' } });
    });

    expect(textInput).toHaveValue('Water evaporates and forms clouds');
  });

  it('handles quiz submission', async () => {
    mockedAxios.post.mockResolvedValueOnce({ data: { score: 80 } });

    await renderTakeQuiz();

    await waitFor(() => {
      expect(screen.getByText('What is 2+2?')).toBeInTheDocument();
    });

    await act(async () => {
      fireEvent.click(screen.getByLabelText('4'));
      fireEvent.click(screen.getByText('Next'));
    });

    const textInput = screen.getByPlaceholderText('Type your answer here...');
    await act(async () => {
      fireEvent.change(textInput, { target: { value: 'Water evaporates and forms clouds' } });
    });

    await act(async () => {
      fireEvent.click(screen.getByText('Submit Quiz'));
      // Wait for the submission to complete
      await new Promise(resolve => setTimeout(resolve, 0));
    });

    await waitFor(() => {
      expect(mockedAxios.post).toHaveBeenCalled();
      expect(mockNavigate).toHaveBeenCalledWith('/quiz/1/results', expect.any(Object));
    });
  });

  it('shows error message when quiz fails to load', async () => {
    const consoleSpy = jest.spyOn(console, 'error').mockImplementation(() => {});
    mockedAxios.get.mockRejectedValueOnce(new Error('Failed to load quiz'));

    await renderTakeQuiz();

    await waitFor(() => {
      expect(screen.getByText(/quiz not found/i)).toBeInTheDocument();
    });

    consoleSpy.mockRestore();
  });
});
