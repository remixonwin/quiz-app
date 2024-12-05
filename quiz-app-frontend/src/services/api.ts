import axios from 'axios';
import { Quiz, Question, QuizSubmission } from '../types';
import { v4 as uuidv4 } from 'uuid';

const API_BASE_URL = process.env.REACT_APP_API_BASE_URL || 'http://localhost:8080/api';

// Add request interceptor for token
axios.interceptors.request.use((config) => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

export const quizService = {
  getAllQuizzes: async (): Promise<Quiz[]> => {
    const response = await axios.get(`${API_BASE_URL}/quizzes`);
    return response.data;
  },

  getQuizDetails: async (quizId: string): Promise<Quiz> => { // Changed `quizId` type to `string` for UUID
    const response = await axios.get(`${API_BASE_URL}/quizzes/${quizId}`);
    // Transform the response to match Quiz type
    const { quiz, questions, answers } = response.data;
    return {
      ...quiz,
      questions: questions.map((q: Question) => ({
        ...q,
        answers: answers.filter((a: any) => a.question_id === q.id).map((a: any) => ({
          id: a.id,
          answer_text: a.answer_text,
          is_correct: a.is_correct,
          created_at: a.created_at,
          updated_at: a.updated_at
        }))
      }))
    };
  },

  submitQuiz: async (submission: QuizSubmission): Promise<{score: number}> => {
    const response = await axios.post(`${API_BASE_URL}/quizzes/submit`, submission);
    return response.data;
  }
};

export const authService = {
  login: async (username: string, password: string): Promise<{token: string}> => {
    try {
      const response = await axios.post(`${API_BASE_URL}/login`, { username, password });
      localStorage.setItem('token', response.data.token); // Store token
      return response.data;
    } catch (error) {
      console.error('Login failed', error);
      throw error;
    }
  },

  register: async (username: string, email: string, password: string): Promise<{token: string}> => {
    try {
      const response = await axios.post(`${API_BASE_URL}/register`, { username, email, password });
      localStorage.setItem('token', response.data.token); // Store token
      return response.data;
    } catch (error) {
      console.error('Registration failed', error);
      throw error;
    }
  }
};
