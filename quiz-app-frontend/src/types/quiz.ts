export interface Answer {
    id?: number;
    question_id?: number;
    answer_text: string;
    is_correct: boolean;
    created_at?: string;
    updated_at?: string;
}

export interface Question {
    id?: number;
    quiz_id?: number;
    text?: string;
    question_text: string;
    question_type: 'multiple_choice' | 'true_false';
    is_multiple_choice?: boolean;
    answers: Answer[];
    options?: string[];
    created_at?: string;
    updated_at?: string;
}

export interface Quiz {
    id?: number;
    title: string;
    description: string | null;
    category?: string;
    creator_id?: number;
    created_by?: number;
    created_at?: string;
    updated_at?: string;
    questions: Question[];
}

export interface CreateQuiz {
    title: string;
    description: string | null;
    category?: string;
    questions: Question[];
}

export interface QuizFormProps {
    initialData?: CreateQuiz;
    onSubmit: (quiz: CreateQuiz) => void;
    isLoading?: boolean;
}

export interface SubmittedAnswer {
    question_id: number;
    answer_id: number;
    is_correct: boolean;
}
