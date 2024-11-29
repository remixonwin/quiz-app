import axios from 'axios';
import { Quiz, Question, Answer, QuizSubmission } from '../types';

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
    try {
      const response = await axios.get(`${API_BASE_URL}/quizzes`);
      return response.data;
    } catch (error) {
      console.error('Error fetching quizzes', error);
      throw error;
    }
  },

  getQuizDetails: async (quizId: number): Promise<{quiz: Quiz, questions: Question[], answers: Answer[]}> => {
    try {
      const response = await axios.get(`${API_BASE_URL}/quizzes/${quizId}`);
      return response.data;
    } catch (error) {
      console.error(`Error fetching quiz ${quizId} details`, error);
      throw error;
    }
  },

  submitQuiz: async (submission: QuizSubmission): Promise<{score: number}> => {
    try {
      const response = await axios.post(`${API_BASE_URL}/quiz-submissions`, submission);
      return response.data;
    } catch (error) {
      console.error('Error submitting quiz', error);
      throw error;
    }
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
