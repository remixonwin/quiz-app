import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  Container,
  Typography,
  Grid,
} from '@mui/material';
import { TextField, Button, IconButton, Paper } from '@mui/material';
import { Add as AddIcon, Delete as DeleteIcon } from '@mui/icons-material';
import axios from 'axios';

interface QuestionForm {
  question_text: string;
  question_type: 'multiple_choice' | 'true_false';
  answers: { answer_text: string; is_correct: boolean }[];
}

interface Answer {
  answer_text: string;
  is_correct: boolean;
}

const QuizCreate: React.FC = () => {
  const navigate = useNavigate();
  const [title, setTitle] = useState<string>('');
  const [description, setDescription] = useState<string>('');
  const [questions, setQuestions] = useState<QuestionForm[]>([]);

  const addQuestion = () => {
    setQuestions([
      ...questions,
      {
        question_text: '',
        question_type: 'multiple_choice',
        answers: [
          { answer_text: '', is_correct: false },
          { answer_text: '', is_correct: false },
        ],
      },
    ]);
  };

  const removeQuestion = (index: number) => {
    setQuestions(questions.filter((_, i) => i !== index));
  };

  const addAnswer = (questionIndex: number) => {
    const newQuestions = [...questions];
    newQuestions[questionIndex].answers.push({
      answer_text: '',
      is_correct: false,
    });
    setQuestions(newQuestions);
  };

  const updateQuestion = (index: number, field: keyof QuestionForm, value: any) => {
    const newQuestions = [...questions];
    newQuestions[index][field] = value;
    setQuestions(newQuestions);
  };

  const updateAnswer = (questionIndex: number, answerIndex: number, field: keyof Answer, value: string | boolean) => {
    const newQuestions = [...questions];
    newQuestions[questionIndex].answers[answerIndex][field] = value as never;
    setQuestions(newQuestions);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const response = await axios.post('http://localhost:8080/api/quizzes', {
        title,
        description,
        questions,
      });
      navigate(`/quiz/${response.data.id}`);
    } catch (error) {
      console.error('Failed to create quiz:', error);
    }
  };

  return (
    <Container maxWidth="md" style={{ marginTop: '2rem' }}>
      <form onSubmit={handleSubmit}>
        <Typography variant="h4" gutterBottom>
          Create New Quiz
        </Typography>
        <Paper style={{ padding: '2rem', marginBottom: '2rem' }}>
          <Grid container spacing={3}>
            <Grid item xs={12}>
              <TextField
                label="Quiz Title"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                fullWidth
                required
              />
            </Grid>
            <Grid item xs={12}>
              <TextField
                label="Description"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                fullWidth
                multiline
                rows={3}
              />
            </Grid>
          </Grid>
        </Paper>

        {questions.map((question, questionIndex) => (
          <Paper key={questionIndex} style={{ padding: '2rem', marginBottom: '2rem' }}>
            <Grid container spacing={3}>
              <Grid item xs={11}>
                <TextField
                  label={`Question ${questionIndex + 1}`}
                  value={question.question_text}
                  onChange={(e) =>
                    updateQuestion(questionIndex, 'question_text', e.target.value)
                  }
                  fullWidth
                  required
                />
              </Grid>
              <Grid item xs={1}>
                <IconButton onClick={() => removeQuestion(questionIndex)}>
                  <DeleteIcon />
                </IconButton>
              </Grid>

              {question.answers.map((answer, answerIndex) => (
                <Grid item xs={12} key={answerIndex}>
                  <Grid container spacing={2}>
                    <Grid item xs={10}>
                      <TextField
                        label={`Answer ${answerIndex + 1}`}
                        value={answer.answer_text}
                        onChange={(e) =>
                          updateAnswer(
                            questionIndex,
                            answerIndex,
                            'answer_text',
                            e.target.value
                          )
                        }
                        fullWidth
                        required
                      />
                    </Grid>
                    <Grid item xs={2}>
                      <Button
                        variant={answer.is_correct ? 'contained' : 'outlined'}
                        color="primary"
                        onClick={() =>
                          updateAnswer(
                            questionIndex,
                            answerIndex,
                            'is_correct',
                            !answer.is_correct
                          )
                        }
                        fullWidth
                      >
                        Correct
                      </Button>
                    </Grid>
                  </Grid>
                </Grid>
              ))}
              <Grid item xs={12}>
                <Button
                  startIcon={<AddIcon />}
                  onClick={() => addAnswer(questionIndex)}
                  variant="outlined"
                >
                  Add Answer
                </Button>
              </Grid>
            </Grid>
          </Paper>
        ))}

        <Button
          startIcon={<AddIcon />}
          onClick={addQuestion}
          variant="contained"
          color="secondary"
          style={{ marginBottom: '2rem' }}
        >
          Add Question
        </Button>

        <Button
          type="submit"
          variant="contained"
          color="primary"
          fullWidth
          disabled={questions.length === 0}
        >
          Create Quiz
        </Button>
      </form>
    </Container>
  );
};

export default QuizCreate;
