import React from 'react';
import { Container, Typography } from '@mui/material';

const CreateQuiz: React.FC = () => {
  return (
    <Container maxWidth="lg" sx={{ py: 4 }}>
      <Typography variant="h4" component="h1" gutterBottom>
        Create Quiz
      </Typography>
      {/* Add quiz creation form here */}
    </Container>
  );
};

export default CreateQuiz;