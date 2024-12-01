import React from 'react';
import { useNavigate } from 'react-router-dom';
import {
  Card,
  CardContent,
  CardActions,
  Typography,
  Chip,
  Box,
  IconButton,
  Tooltip,
} from '@mui/material';
import {
  PlayArrow as PlayIcon,
  Edit as EditIcon,
  Delete as DeleteIcon,
  Share as ShareIcon,
} from '@mui/icons-material';
import { Quiz } from '../types';

interface QuizCardProps {
  quiz: Quiz;
  onDelete?: (id: number) => void;
  isOwner?: boolean;
}

const QuizCard: React.FC<QuizCardProps> = ({ quiz, onDelete, isOwner = false }) => {
  const navigate = useNavigate();

  const handlePlay = () => {
    navigate(`/quiz/take/${quiz.id}`);
  };

  const handleEdit = () => {
    navigate(`/quiz/edit/${quiz.id}`);
  };

  const handleShare = async () => {
    try {
      await navigator.clipboard.writeText(
        `${window.location.origin}/quiz/take/${quiz.id}`
      );
      // You might want to add a toast notification here
    } catch (err) {
      console.error('Failed to copy link:', err);
    }
  };

  return (
    <Card 
      sx={{ 
        height: '100%', 
        display: 'flex', 
        flexDirection: 'column',
        transition: 'transform 0.2s ease-in-out',
        '&:hover': {
          transform: 'translateY(-4px)',
          boxShadow: 3,
        },
      }}
    >
      <CardContent sx={{ flexGrow: 1 }}>
        <Typography gutterBottom variant="h5" component="h2">
          {quiz.title}
        </Typography>
        <Typography variant="body2" color="text.secondary" paragraph>
          {quiz.description || 'No description available'}
        </Typography>
        <Box display="flex" gap={1} flexWrap="wrap">
          <Chip 
            label={`${quiz.questions?.length || 0} Questions`} 
            size="small" 
            color="primary" 
            variant="outlined"
          />
          {quiz.category && (
            <Chip 
              label={quiz.category} 
              size="small" 
              color="secondary" 
              variant="outlined"
            />
          )}
        </Box>
      </CardContent>
      <CardActions sx={{ justifyContent: 'space-between', px: 2, pb: 2 }}>
        <Box>
          <Tooltip title="Take Quiz">
            <IconButton 
              color="primary" 
              onClick={handlePlay}
              size="small"
            >
              <PlayIcon />
            </IconButton>
          </Tooltip>
          <Tooltip title="Share Quiz">
            <IconButton
              onClick={handleShare}
              size="small"
            >
              <ShareIcon />
            </IconButton>
          </Tooltip>
        </Box>
        {isOwner && (
          <Box>
            <Tooltip title="Edit Quiz">
              <IconButton
                onClick={handleEdit}
                size="small"
                color="info"
              >
                <EditIcon />
              </IconButton>
            </Tooltip>
            <Tooltip title="Delete Quiz">
              <IconButton
                onClick={() => quiz.id && onDelete?.(quiz.id)}
                size="small"
                color="error"
              >
                <DeleteIcon />
              </IconButton>
            </Tooltip>
          </Box>
        )}
      </CardActions>
    </Card>
  );
};

export default QuizCard;
