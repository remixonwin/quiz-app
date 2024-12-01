import React from 'react';
import {
  Card,
  CardContent,
  Typography,
  Grid,
  Box,
  Tooltip,
  IconButton,
  useTheme,
} from '@mui/material';
import {
  People as ParticipantsIcon,
  Stars as ScoreIcon,
  AccessTime as TimeIcon,
  Info as InfoIcon,
} from '@mui/icons-material';

interface QuizStatsProps {
  totalParticipants: number;
  averageScore: number;
  averageTime: number;
  completionRate: number;
}

const QuizStats: React.FC<QuizStatsProps> = ({
  totalParticipants,
  averageScore,
  averageTime,
  completionRate,
}) => {
  const theme = useTheme();

  const formatTime = (minutes: number): string => {
    if (minutes < 1) return '< 1 min';
    return `${Math.round(minutes)} min${minutes !== 1 ? 's' : ''}`;
  };

  const stats = [
    {
      icon: <ParticipantsIcon />,
      value: totalParticipants,
      label: 'Participants',
      tooltip: 'Total number of quiz participants',
      color: theme.palette.primary.main,
    },
    {
      icon: <ScoreIcon />,
      value: `${Math.round(averageScore)}%`,
      label: 'Avg. Score',
      tooltip: 'Average score of all participants',
      color: theme.palette.success.main,
    },
    {
      icon: <TimeIcon />,
      value: formatTime(averageTime),
      label: 'Avg. Time',
      tooltip: 'Average time taken to complete the quiz',
      color: theme.palette.info.main,
    },
    {
      icon: <InfoIcon />,
      value: `${Math.round(completionRate)}%`,
      label: 'Completion',
      tooltip: 'Percentage of participants who completed the quiz',
      color: theme.palette.warning.main,
    },
  ];

  return (
    <Card>
      <CardContent>
        <Typography variant="h6" gutterBottom>
          Quiz Statistics
        </Typography>
        <Grid container spacing={3}>
          {stats.map((stat, index) => (
            <Grid item xs={6} sm={3} key={index}>
              <Box
                sx={{
                  display: 'flex',
                  flexDirection: 'column',
                  alignItems: 'center',
                  textAlign: 'center',
                }}
              >
                <Box
                  sx={{
                    display: 'flex',
                    alignItems: 'center',
                    mb: 1,
                  }}
                >
                  <Box
                    sx={{
                      color: stat.color,
                      display: 'flex',
                      alignItems: 'center',
                    }}
                  >
                    {stat.icon}
                  </Box>
                  <Tooltip title={stat.tooltip}>
                    <IconButton size="small">
                      <InfoIcon fontSize="small" />
                    </IconButton>
                  </Tooltip>
                </Box>
                <Typography variant="h5" component="div" sx={{ mb: 0.5 }}>
                  {stat.value}
                </Typography>
                <Typography
                  variant="body2"
                  color="text.secondary"
                  sx={{ whiteSpace: 'nowrap' }}
                >
                  {stat.label}
                </Typography>
              </Box>
            </Grid>
          ))}
        </Grid>
      </CardContent>
    </Card>
  );
};

export default QuizStats;
