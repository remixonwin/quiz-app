export * from './quiz';

export interface Question {
  id: number;
  quiz_id: number;
  question_text: string;
  question_type: 'multiple_choice' | 'true_false';
  created_at: string;
}

export interface Answer {
  id: number;
  answer_text: string;
  is_correct: boolean;
}

export interface SubmittedAnswer {
  question_id: number;
  answer_id: number;
}

export interface QuizSubmission {
  quiz_id: number;
  answers: SubmittedAnswer[];
}
