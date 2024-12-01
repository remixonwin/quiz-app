import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { BrowserRouter } from 'react-router-dom';
import axios from 'axios';
import Dashboard from '../../pages/Dashboard';
import { act } from 'react-dom/test-utils';

// Mock axios
jest.mock('axios');
const mockedAxios = axios as jest.Mocked<typeof axios>;

// Mock navigation
const mockNavigate = jest.fn();
jest.mock('react-router-dom', () => ({
  ...jest.requireActual('react-router-dom'),
  useNavigate: () => mockNavigate,
}));

describe('Dashboard', () => {
  const mockQuizzes = [
    {
      id: 1,
      title: 'Math Quiz',
      description: 'Test your math skills',
      created_by: 1,
      questions: [],
      category: 'Mathematics',
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    },
    {
      id: 2,
      title: 'History Quiz',
      description: 'Learn about history',
      created_by: 1,
      questions: [],
      category: 'History',
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    },
  ];

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('shows loading state and then displays quizzes', async () => {
    // Setup delayed response to simulate loading
    mockedAxios.get.mockImplementationOnce(() => 
      new Promise(resolve => 
        setTimeout(() => resolve({ data: mockQuizzes }), 100)
      )
    );

    await act(async () => {
      render(
        <BrowserRouter>
          <Dashboard />
        </BrowserRouter>
      );
    });

    // Check loading state is shown
    expect(screen.getByText(/loading quizzes/i)).toBeInTheDocument();

    // Wait for quizzes to load
    await waitFor(() => {
      expect(screen.getByText('Math Quiz')).toBeInTheDocument();
      expect(screen.getByText('History Quiz')).toBeInTheDocument();
    });

    // Loading state should be gone
    expect(screen.queryByText(/loading quizzes/i)).not.toBeInTheDocument();
  });

  it('filters quizzes based on search input', async () => {
    mockedAxios.get.mockResolvedValueOnce({ data: mockQuizzes });

    await act(async () => {
      render(
        <BrowserRouter>
          <Dashboard />
        </BrowserRouter>
      );
    });

    await waitFor(() => {
      expect(screen.getByText('Math Quiz')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText(/search quizzes/i);
    await act(async () => {
      fireEvent.change(searchInput, { target: { value: 'math' } });
    });

    expect(screen.getByText('Math Quiz')).toBeInTheDocument();
    expect(screen.queryByText('History Quiz')).not.toBeInTheDocument();
  });

  it('navigates to create quiz page when create button is clicked', async () => {
    mockedAxios.get.mockResolvedValueOnce({ data: mockQuizzes });

    await act(async () => {
      render(
        <BrowserRouter>
          <Dashboard />
        </BrowserRouter>
      );
    });

    await waitFor(() => {
      expect(screen.getByText(/create quiz/i)).toBeInTheDocument();
    });

    const createButton = screen.getByText(/create quiz/i);
    await act(async () => {
      fireEvent.click(createButton);
    });

    expect(mockNavigate).toHaveBeenCalledWith('/quiz/create');
  });

  it('handles quiz deletion and updates UI', async () => {
    mockedAxios.get.mockResolvedValueOnce({ data: mockQuizzes });
    mockedAxios.delete.mockResolvedValueOnce({});

    await act(async () => {
      render(
        <BrowserRouter>
          <Dashboard />
        </BrowserRouter>
      );
    });

    // Wait for quizzes to load
    await waitFor(() => {
      expect(screen.getByText('Math Quiz')).toBeInTheDocument();
    });

    // Find and click delete button for Math Quiz
    const deleteButtons = screen.getAllByRole('button', { name: 'Delete Quiz' });
    await act(async () => {
      fireEvent.click(deleteButtons[0]);
    });

    // Verify API call
    expect(mockedAxios.delete).toHaveBeenCalledWith('http://localhost:8080/api/quizzes/1');

    // Verify quiz is removed from UI
    await waitFor(() => {
      expect(screen.queryByText('Math Quiz')).not.toBeInTheDocument();
      expect(screen.getByText('History Quiz')).toBeInTheDocument();
    });
  });

  it('displays error message when API call fails', async () => {
    // Mock console.error to prevent test output pollution
    const consoleSpy = jest.spyOn(console, 'error').mockImplementation(() => {});
    
    const errorMessage = 'Network Error';
    mockedAxios.get.mockRejectedValueOnce(new Error(errorMessage));

    await act(async () => {
      render(
        <BrowserRouter>
          <Dashboard />
        </BrowserRouter>
      );
    });

    await waitFor(() => {
      expect(screen.getByText(/failed to load quizzes/i)).toBeInTheDocument();
    });

    // UI should not show quizzes when there's an error
    expect(screen.queryByText('Math Quiz')).not.toBeInTheDocument();
    expect(screen.queryByText('History Quiz')).not.toBeInTheDocument();

    // Clean up console mock
    consoleSpy.mockRestore();
  });
});
