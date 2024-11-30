import React from 'react';
import {
    Grid,
    TextField,
    IconButton,
    FormControlLabel,
    Checkbox,
    Typography,
} from '@mui/material';
import { Delete as DeleteIcon, Add as AddIcon } from '@mui/icons-material';
import { Question, Answer } from '../types/quiz';

interface QuestionFormProps {
    question: Question;
    onQuestionChange: (question: Question) => void;
    onRemove: () => void;
}

export const QuestionForm: React.FC<QuestionFormProps> = ({
    question,
    onQuestionChange,
    onRemove,
}) => {
    const handleAnswerChange = (index: number, field: keyof Answer, value: string | boolean) => {
        const newAnswers = [...question.answers];
        newAnswers[index] = {
            ...newAnswers[index],
            [field]: value,
        };
        onQuestionChange({
            ...question,
            answers: newAnswers,
        });
    };

    const addAnswer = () => {
        onQuestionChange({
            ...question,
            answers: [...question.answers, { answer_text: '', is_correct: false }],
        });
    };

    const removeAnswer = (index: number) => {
        onQuestionChange({
            ...question,
            answers: question.answers.filter((_, i) => i !== index),
        });
    };

    return (
        <Grid container spacing={2} sx={{ mb: 4, p: 2, bgcolor: 'background.paper', borderRadius: 1 }}>
            <Grid item xs={12}>
                <TextField
                    fullWidth
                    label="Question Text"
                    value={question.question_text}
                    onChange={(e) =>
                        onQuestionChange({
                            ...question,
                            question_text: e.target.value,
                        })
                    }
                />
            </Grid>
            
            <Grid item xs={12}>
                <Typography variant="subtitle1" sx={{ mt: 2, mb: 1 }}>
                    Answers
                </Typography>
                {question.answers.map((answer, index) => (
                    <Grid container spacing={2} key={index} alignItems="center">
                        <Grid item xs={8}>
                            <TextField
                                fullWidth
                                label={`Answer ${index + 1}`}
                                value={answer.answer_text}
                                onChange={(e) =>
                                    handleAnswerChange(index, 'answer_text', e.target.value)
                                }
                            />
                        </Grid>
                        <Grid item xs={3}>
                            <FormControlLabel
                                control={
                                    <Checkbox
                                        checked={answer.is_correct}
                                        onChange={(e) =>
                                            handleAnswerChange(
                                                index,
                                                'is_correct',
                                                e.target.checked
                                            )
                                        }
                                    />
                                }
                                label="Correct"
                            />
                        </Grid>
                        <Grid item xs={1}>
                            <IconButton
                                onClick={() => removeAnswer(index)}
                                disabled={question.answers.length <= 2}
                            >
                                <DeleteIcon />
                            </IconButton>
                        </Grid>
                    </Grid>
                ))}
            </Grid>

            <Grid item xs={12} container justifyContent="space-between">
                <IconButton onClick={addAnswer} color="primary">
                    <AddIcon />
                </IconButton>
                <IconButton onClick={onRemove} color="error">
                    <DeleteIcon />
                </IconButton>
            </Grid>
        </Grid>
    );
};
