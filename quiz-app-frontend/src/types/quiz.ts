export interface Answer {
    answer_text: string;
    is_correct: boolean;
}

export interface Question {
    question_text: string;
    question_type: 'multiple_choice' | 'true_false';
    answers: Answer[];
}

export interface Quiz {
    id?: number;
    title: string;
    description: string;
    questions: Question[];
}

export interface QuizFormProps {
    initialData?: Quiz;
    onSubmit: (quiz: Quiz) => void;
    isLoading?: boolean;
}
