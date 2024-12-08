import React from 'react';
import { screen, fireEvent, act } from '@testing-library/react';
import { renderWithRouter } from '../utils/test-utils';
import QuizCard from './QuizCard';

// Mock navigation
const mockNavigate = jest.fn();
jest.mock('react-router-dom', () => ({
    ...jest.requireActual('react-router-dom'),
    useNavigate: () => mockNavigate
}));

describe('QuizCard', () => {
    const mockQuiz = {
        id: 1,
        title: 'Test Quiz',
        description: 'A test quiz description',
        category: 'Test',
        created_by: 1,
        questions: [
            {
                id: 1,
                question_text: 'Test Question',
                question_type: 'multiple_choice' as const,
                text: 'Test Question',
                is_multiple_choice: true,
                answers: [],
                options: []
            }
        ],
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
    };

    const mockOnDelete = jest.fn();

    beforeEach(() => {
        jest.clearAllMocks();
    });

    test('renders quiz information correctly', async () => {
        await renderWithRouter(<QuizCard quiz={mockQuiz} onDelete={mockOnDelete} />);

        expect(screen.getByTestId(`quiz-title-${mockQuiz.title}`)).toHaveTextContent(mockQuiz.title);
        expect(screen.getByText(mockQuiz.description)).toBeInTheDocument();
        expect(screen.getByText(mockQuiz.category)).toBeInTheDocument();
        expect(screen.getByText('1 Questions')).toBeInTheDocument();
    });

    test('navigates to take quiz page when play button is clicked', async () => {
        await renderWithRouter(<QuizCard quiz={mockQuiz} onDelete={mockOnDelete} />);

        const playButton = screen.getByRole('button', { name: 'Take Quiz' });
        fireEvent.click(playButton);

        expect(mockNavigate).toHaveBeenCalledWith('/quiz/take/1');
    });

    test('shows edit button only when isOwner is true', async () => {
        const { rerender: rerenderComponent } = await renderWithRouter(
            <QuizCard quiz={mockQuiz} onDelete={mockOnDelete} isOwner={false} />
        );

        expect(screen.queryByRole('button', { name: 'Edit Quiz' })).not.toBeInTheDocument();

        await act(async () => {
            await rerenderComponent(
                <QuizCard quiz={mockQuiz} onDelete={mockOnDelete} isOwner={true} />
            );
        });

        expect(screen.getByRole('button', { name: 'Edit Quiz' })).toBeInTheDocument();
    });

    test('calls onDelete when delete button is clicked', async () => {
        await renderWithRouter(<QuizCard quiz={mockQuiz} onDelete={mockOnDelete} isOwner={true} />);

        const deleteButton = screen.getByRole('button', { name: 'Delete Quiz' });
        fireEvent.click(deleteButton);

        expect(mockOnDelete).toHaveBeenCalledWith(mockQuiz.id);
    });
});
