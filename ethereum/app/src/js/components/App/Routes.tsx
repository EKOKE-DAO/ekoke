import * as React from 'react';
import { Route as RouterRoute, Routes } from 'react-router-dom';

import { Route } from '../../utils/routes';
import Home from './pages/Home';
import Deferred from './pages/Deferred';
import Ekoke from './pages/Ekoke';
import Marketplace from './pages/Marketplace';
import RewardPool from './pages/RewardPool';
import WaitForMetamask from './pages/WaitForMetamask';
import EkokePresale from './pages/EkokePresale';

const AppRouter = () => (
  <>
    <main>
      <Routes>
        <RouterRoute path={Route.url(Route.HOME)} element={<Home />} />
        <RouterRoute
          path={Route.url(Route.DEFERRED)}
          element={
            <WaitForMetamask>
              <Deferred />
            </WaitForMetamask>
          }
        />
        <RouterRoute
          path={Route.url(Route.EKOKE)}
          element={
            <WaitForMetamask>
              <Ekoke />
            </WaitForMetamask>
          }
        />
        <RouterRoute
          path={Route.url(Route.EKOKE_PRESALE)}
          element={
            <WaitForMetamask>
              <EkokePresale />
            </WaitForMetamask>
          }
        />
        <RouterRoute
          path={Route.url(Route.MARKETPLACE)}
          element={
            <WaitForMetamask>
              <Marketplace />
            </WaitForMetamask>
          }
        />
        <RouterRoute
          path={Route.url(Route.REWARD_POOL)}
          element={
            <WaitForMetamask>
              <RewardPool />
            </WaitForMetamask>
          }
        />
      </Routes>
    </main>
  </>
);

export default AppRouter;
