import * as React from 'react';
import Button from './Button';

interface Props {
  onClick: () => void;
  disabled?: boolean;
  children: JSX.Element | JSX.Element[] | string;
  transparent?: boolean;
  className?: string;
}

const Cta = (props: Props) => {
  const style = {
    backgroundColor: props.transparent ? 'rgba(0,0,0,0)' : '#31363b',
  };

  return (
    <Button.Primary
      className={`${props.className} text-xl h-fit px-24 py-4 w-full`}
      style={style}
      disabled={props.disabled}
      onClick={props.onClick}
    >
      {props.children}
    </Button.Primary>
  );
};

export const Alternative = (props: Props) => {
  return (
    <Button.Alternative
      className={`${props.className} text-lg h-fit px-24 py-4 w-full`}
      disabled={props.disabled}
      onClick={props.onClick}
    >
      {props.children}
    </Button.Alternative>
  );
};

export default Cta;
