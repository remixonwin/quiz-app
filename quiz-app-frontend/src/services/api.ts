import axios from 'axios';
import { Quiz, Question, Answer, QuizSubmission, SubmittedAnswer } from '../types';

const API_BASE_URL = 'http://localhost:8080/api';

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

  getQuizDetails: async (quizId: number): Promise<Quiz> => {
    const response = await axios.get(`${API_BASE_URL}/quizzes/${quizId}`);
    // Transform the response to match Quiz type
    const { quiz, questions, answers } = response.data;
    return {
      ...quiz,
      questions: questions.map((q: Question) => ({
        ...q,
        answers: answers.filter((a: Answer) => a.question_id === q.id)
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
      return response.data;
    } catch (error) {
      console.error('Login failed', error);
      throw error;
    }
  },

  register: async (username: string, email: string, password: string): Promise<{token: string}> => {
    try {
      const response = await axios.post(`${API_BASE_URL}/register`, { username, email, password });
      return response.data;
    } catch (error) {
      console.error('Registration failed', error);
      throw error;
    }
  }
};
