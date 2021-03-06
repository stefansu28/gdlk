import React from 'react';
import HardwareSpecListView from './hardware/HardwareSpecListView';
import {
  Grid,
  Card,
  CardContent,
  CardHeader,
  Typography,
} from '@material-ui/core';
import Link from './common/Link';

const HomePage: React.FC = () => {
  return (
    <Grid container>
      <Grid item xs={12}>
        <Card>
          <CardHeader
            title={
              <Typography variant="h1">
                GDLK Development Language Kit
              </Typography>
            }
          />
          <CardContent>
            <Typography>
              GDLK is a programming-based puzzle game. Use the GDLK language to
              solve puzzles and progress to more complex hardware. Get started
              by <Link to="/docs">reading the docs</Link>, or jump into the
              first puzzle below.
            </Typography>
          </CardContent>
        </Card>
      </Grid>

      <Grid item md={4} sm={8} xs={12}>
        <HardwareSpecListView />
      </Grid>
    </Grid>
  );
};

export default HomePage;
