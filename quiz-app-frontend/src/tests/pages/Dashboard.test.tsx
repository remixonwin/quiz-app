import React from 'react';
import { screen, fireEvent, waitFor } from '@testing-library/react';
import { render } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import axiosInstance from '../../utils/axios-config';
import Dashboard from '../../pages/Dashboard';
import Login from '../../pages/Login';
import Register from '../../pages/Register';

// Mock axios config
jest.mock('../../utils/axios-config');
const mockedAxios = axiosInstance as jest.Mocked<typeof axiosInstance>;

// Mock navigation
const mockNavigate = jest.fn();
jest.mock('react-router-dom', () => ({
  ...jest.requireActual('react-router-dom'),
  useNavigate: () => mockNavigate,
  useLocation: () => ({
    state: null
  })
}));

describe('Dashboard Tests', () => {
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

    render(
      <MemoryRouter>
        <Dashboard />
      </MemoryRouter>
    );

    // Check loading state is shown initially
    expect(await screen.findByText(/loading quizzes/i)).toBeInTheDocument();

    // Wait for quizzes to load
    await waitFor(() => {
      expect(screen.getByText('Math Quiz')).toBeInTheDocument();
      expect(screen.getByText('History Quiz')).toBeInTheDocument();
    });

    // Verify loading state is gone
    expect(screen.queryByText(/loading quizzes/i)).not.toBeInTheDocument();
  });

  it('filters quizzes based on search input', async () => {
    mockedAxios.get.mockResolvedValueOnce({ data: mockQuizzes });

    render(
      <MemoryRouter>
        <Dashboard />
      </MemoryRouter>
    );

    // Wait for quizzes to load
    await screen.findByText('Math Quiz');

    const searchInput = screen.getByPlaceholderText(/search quizzes/i);
    fireEvent.change(searchInput, { target: { value: 'math' } });

    // Verify filtered results
    expect(screen.getByText('Math Quiz')).toBeInTheDocument();
    expect(screen.queryByText('History Quiz')).not.toBeInTheDocument();
  });

  it('navigates to create quiz page when create button is clicked', async () => {
    mockedAxios.get.mockResolvedValueOnce({ data: mockQuizzes });

    render(
      <MemoryRouter>
        <Dashboard />
      </MemoryRouter>
    );

    // Wait for loading state to finish
    await waitFor(() => {
      expect(screen.queryByText('Loading quizzes...')).not.toBeInTheDocument();
    });

    // Wait for loading state to finish
    await waitFor(() => {
      expect(screen.queryByText('Loading quizzes...')).not.toBeInTheDocument();
    });

    // Wait for loading state to finish
    await waitFor(() => {
      expect(screen.queryByText('Loading quizzes...')).not.toBeInTheDocument();
    });

    // Find and click the create button
    const createButton = await screen.findByRole('button', { name: /create quiz/i });
    // Use data-testid to find the quiz title
    const mathQuizTitle = screen.getByTestId('quiz-title-Math Quiz');
    expect(mathQuizTitle).toBeInTheDocument();
    fireEvent.click(createButton);

    expect(mockNavigate).toHaveBeenCalledWith('/quiz/create');
  });

  it('handles quiz deletion and updates UI', async () => {
    // Mock successful API calls
    mockedAxios.get.mockResolvedValueOnce({ data: mockQuizzes });
    mockedAxios.delete.mockResolvedValueOnce({ data: { message: 'Quiz deleted successfully' } });

    render(
      <MemoryRouter>
        <Dashboard />
      </MemoryRouter>
    );

    // Wait for loading state to finish
    await waitFor(() => {
      expect(screen.queryByText('Loading quizzes...')).not.toBeInTheDocument();
    });

    // Wait for quizzes to be displayed
    await waitFor(() => {
      expect(screen.getByText('Math Quiz')).toBeInTheDocument();
    });

    // Find and click delete button for Math Quiz
    const deleteButtons = screen.getAllByRole('button', { name: /delete/i });
    fireEvent.click(deleteButtons[0]);

    // First, verify that the delete API was called with the correct ID
    expect(mockedAxios.delete).toHaveBeenCalledWith('/api/quizzes/1');

    // Wait for the quiz to be removed
    await waitFor(
      () => {
        const mathQuiz = screen.queryByText('Math Quiz');
        expect(mathQuiz).not.toBeInTheDocument();
      },
      { timeout: 1000 }
    );

    // Verify the other quiz is still there
    expect(screen.getByText('History Quiz')).toBeInTheDocument();
  });

  it('displays error message when API call fails', async () => {
    // Mock console.error to prevent test output pollution
    const consoleSpy = jest.spyOn(console, 'error').mockImplementation(() => {});
    
    const errorMessage = 'Network Error';
    mockedAxios.get.mockRejectedValueOnce(new Error(errorMessage));

    render(
      <MemoryRouter>
        <Dashboard />
      </MemoryRouter>
    );

    await waitFor(() => {
      expect(screen.getByText(/Failed to fetch quizzes. Please ensure you are logged in./i)).toBeInTheDocument();
    });

    // UI should not show quizzes when there's an error
    expect(screen.queryByText('Math Quiz')).not.toBeInTheDocument();
    expect(screen.queryByText('History Quiz')).not.toBeInTheDocument();

    // Clean up console mock
    consoleSpy.mockRestore();
  });

  it('logs in successfully', async () => {
    const mockLoginResponse = { token: 'mockToken' };
    mockedAxios.post.mockResolvedValueOnce({ data: mockLoginResponse });

    render(
      <MemoryRouter>
        <Login />
      </MemoryRouter>
    );

    // Find inputs using getByRole with name (label)
    fireEvent.change(screen.getByRole('textbox', { name: /email address/i }), {
      target: { value: 'testuser@example.com' }
    });
    // Find password field by its label text
    const passwordInput = screen.getByLabelText(/^password/i);
    fireEvent.change(passwordInput, { target: { value: 'password' } });
    
    // Find and click the submit button
    const loginButton = screen.getByRole('button', { name: /sign in/i });
    fireEvent.click(loginButton);

    await waitFor(() => {
      expect(mockedAxios.post).toHaveBeenCalledWith('/api/auth/login', {
        email: 'testuser@example.com',
        password: 'password'
      });
      expect(mockNavigate).toHaveBeenCalledWith('/');
    });
  });

  it('registers successfully', async () => {
    const mockRegisterResponse = { message: 'Registration successful' };
    mockedAxios.post.mockResolvedValueOnce({ data: mockRegisterResponse });

    render(
      <MemoryRouter>
        <Register />
      </MemoryRouter>
    );

    // Fill in the registration form
    fireEvent.change(screen.getByRole('textbox', { name: /username/i }), {
      target: { value: 'newuser' }
    });
    fireEvent.change(screen.getByRole('textbox', { name: /email address/i }), {
      target: { value: 'newuser@example.com' }
    });
    
    // Find password fields by their label text
    const passwordInput = screen.getByLabelText(/^password/i);
    const confirmPasswordInput = screen.getByLabelText(/^confirm password/i);
    fireEvent.change(passwordInput, { target: { value: 'password' } });
    fireEvent.change(confirmPasswordInput, { target: { value: 'password' } });

    // Find and click the submit button with exact text
    const registerButton = screen.getByRole('button', { name: 'Register' });
    fireEvent.click(registerButton);

    await waitFor(() => {
      expect(mockedAxios.post).toHaveBeenCalledWith('/api/auth/register', {
        username: 'newuser',
        email: 'newuser@example.com',
        password: 'password'
      });
      expect(mockNavigate).toHaveBeenCalledWith('/login', { 
        state: { message: 'Registration successful! Please login.' } 
      });
    });
  });
});
