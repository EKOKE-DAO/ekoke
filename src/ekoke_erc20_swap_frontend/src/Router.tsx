import * as React from 'react';
import { Route, Routes, useLocation } from 'react-router-dom';
import { IntlProvider } from 'react-intl';

import { getIdFromHash } from './js/utils/routes';
import CookieBar from './js/components/CookieBar';
import Fallback from './js/pages/Fallback';
import Footer from './js/components/Footer';
import SeoEngine from './js/components/SeoEngine';
import TRANSLATIONS, { getNavigatorLanguage } from './js/utils/translations';
import {
  hasCookiePreferences,
  isAnalyticsCookiesConsentGiven,
} from './js/utils/cookies';
import { initGaConsent } from './js/utils/analytics';

const Home = React.lazy(() => import('./js/pages/Home'));
const NotFound = React.lazy(() => import('./js/pages/NotFound'));

const App = () => {
  const language = getNavigatorLanguage();
  const { pathname } = useLocation();
  const [hasCookieBar, setHasCookieBar] = React.useState(
    !hasCookiePreferences(),
  );

  // init GA consent
  initGaConsent(false, isAnalyticsCookiesConsentGiven());

  React.useEffect(() => {
    const hash = getIdFromHash();
    if (!hash) {
      window.scrollTo(0, 0);
    } else {
      document.getElementById(hash)?.scrollIntoView({ behavior: 'smooth' });
    }
  }, [pathname]);

  const onCookieBarClose = () => {
    setHasCookieBar(false);
  };

  const cookieBar = hasCookieBar ? (
    <CookieBar onClose={onCookieBarClose} />
  ) : (
    <></>
  );

  return (
    <IntlProvider locale={language} messages={TRANSLATIONS[language]}>
      <SeoEngine />
      <React.Suspense fallback={<Fallback />}>
        <main>
          <Routes>
            <Route path="/" element={<Home />} />
            {/* catch all */}
            <Route path="/*" element={<NotFound />} />
          </Routes>
        </main>
        <Footer />
        {cookieBar}
      </React.Suspense>
    </IntlProvider>
  );
};

export default App;
