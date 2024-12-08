
import React from 'react';
import { Container, Typography } from '@mui/material';

const QuizList: React.FC = () => {
  return (
    <Container maxWidth="lg" sx={{ py: 4 }}>
      <Typography variant="h4" component="h1" gutterBottom>
        Quiz List
      </Typography>
      {/* Add quiz list content here */}
    </Container>
  );
};

export default QuizList;