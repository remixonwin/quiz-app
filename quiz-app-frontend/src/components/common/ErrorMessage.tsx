import React from 'react';
import { Alert, Box, Button } from '@mui/material';
import RefreshIcon from '@mui/icons-material/Refresh';

interface ErrorMessageProps {
  message: string;
  onRetry?: () => void;
}

const ErrorMessage: React.FC<ErrorMessageProps> = ({ message, onRetry }) => {
  return (
    <Box sx={{ p: 2 }}>
      <Alert 
        severity="error"
        action={
          onRetry && (
            <Button
              color="inherit"
              size="small"
              onClick={onRetry}
              startIcon={<RefreshIcon />}
            >
              Retry
            </Button>
          )
        }
      >
        {message}
      </Alert>
    </Box>
  );
};

export default ErrorMessage;
