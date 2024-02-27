import * as React from 'react';
import Container from './Container';

const BlankPage = (props: React.HTMLProps<HTMLDivElement>) => {
  return (
    <div className={`${props.className} w-screen min-h-screen bg-page`}>
      {props.children}
    </div>
  );
};

const BrandPage = (props: React.HTMLProps<HTMLDivElement>) => {
  return (
    <BlankPage className={`${props.className} relative bg-brand py-32`}>
      <Container.Container className="z-1 relative">
        {props.children}
      </Container.Container>
    </BlankPage>
  );
};

export default {
  BlankPage,
  BrandPage,
};
