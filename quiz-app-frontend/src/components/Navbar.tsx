import React from 'react';
import { Link } from 'react-router-dom';
import { AppBar, Toolbar, Typography, Button } from '@mui/material';

const Navbar: React.FC = () => {
  return (
    <AppBar position="static">
      <Toolbar>
        <Typography variant="h6" style={{ flexGrow: 1 }}>
          <Link to="/" style={{ color: 'white', textDecoration: 'none' }}>
            Quiz App
          </Link>
        </Typography>
        <Button color="inherit" component={Link} to="/quizzes">
          Browse Quizzes
        </Button>
        <Button color="inherit" component={Link} to="/create">
          Create Quiz
        </Button>
      </Toolbar>
    </AppBar>
  );
};

export default Navbar;
