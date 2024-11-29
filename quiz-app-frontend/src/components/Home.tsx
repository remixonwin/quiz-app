import React from 'react';
import { Container, Typography, Button, Grid, Paper } from '@mui/material';
import { Link } from 'react-router-dom';

const Home: React.FC = () => {
  return (
    <Container maxWidth="md" style={{ marginTop: '2rem' }}>
      <Paper elevation={3} style={{ padding: '2rem' }}>
        <Typography variant="h3" component="h1" gutterBottom>
          Welcome to Quiz App
        </Typography>
        <Typography variant="h5" component="h2" gutterBottom>
          Test your knowledge or create your own quizzes
        </Typography>
        <Grid container spacing={3} style={{ marginTop: '2rem' }}>
          <Grid item xs={6}>
            <Button
              component={Link}
              to="/quizzes"
              variant="contained"
              color="primary"
              fullWidth
            >
              Take a Quiz
            </Button>
          </Grid>
          <Grid item xs={6}>
            <Button
              component={Link}
              to="/create"
              variant="contained"
              color="secondary"
              fullWidth
            >
              Create a Quiz
            </Button>
          </Grid>
        </Grid>
      </Paper>
    </Container>
  );
};

export default Home;
