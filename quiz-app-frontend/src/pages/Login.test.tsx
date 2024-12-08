import React from 'react';
import { screen, fireEvent, waitFor } from '@testing-library/react';
import { renderWithRouter } from '../utils/test-utils';
import Login from './Login';
import api from '../utils/axios-config';

// Mock the axios config
jest.mock('../utils/axios-config', () => ({
    post: jest.fn(),
    interceptors: {
        request: { use: jest.fn(), eject: jest.fn() },
        response: { use: jest.fn(), eject: jest.fn() }
    }
}));

// Mock navigation
const mockNavigate = jest.fn();
jest.mock('react-router-dom', () => ({
    ...jest.requireActual('react-router-dom'),
    useNavigate: () => mockNavigate,
    useLocation: () => ({ state: null })
}));

describe('Login Component', () => {
    beforeEach(() => {
        jest.clearAllMocks();
    });

    test('displays success message on successful login', async () => {
        // Mock successful API response
        (api.post as jest.Mock).mockResolvedValueOnce({ data: { token: 'mock-token' } });

        await renderWithRouter(<Login />);

        // Fill in the form
        fireEvent.change(screen.getByLabelText(/email address/i), {
            target: { value: 'remixonwin' },
        });
        fireEvent.change(screen.getByLabelText(/password/i), {
            target: { value: 'dragon25' },
        });

        // Click the sign in button
        fireEvent.click(screen.getByRole('button', { name: /sign in/i }));

        // Wait for navigation to occur
        await waitFor(() => {
            expect(mockNavigate).toHaveBeenCalledWith('/');
        });
    });

    test('displays error message on failed login', async () => {
        // Mock failed API response
        (api.post as jest.Mock).mockRejectedValueOnce(new Error('Invalid credentials'));

        await renderWithRouter(<Login />);

        // Fill in the form with wrong credentials
        fireEvent.change(screen.getByLabelText(/email address/i), {
            target: { value: 'wronguser' },
        });
        fireEvent.change(screen.getByLabelText(/password/i), {
            target: { value: 'wrongpass' },
        });

        // Click the sign in button
        fireEvent.click(screen.getByRole('button', { name: /sign in/i }));

        // Wait for error message to appear
        await waitFor(() => {
            expect(screen.getByText(/invalid email or password/i)).toBeInTheDocument();
        });
    });
});
