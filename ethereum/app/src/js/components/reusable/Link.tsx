import * as React from 'react';
import { Route } from '../../utils/routes';

interface Props extends React.HTMLProps<HTMLAnchorElement> {
  href?: string | Route;
}

const resolve = (href?: string | Route): string | undefined => {
  if (!href) {
    return undefined;
  }

  return Route.isRoute(href) ? Route.url(href as Route) : href;
};

const Default = (props: Props) => (
  <a
    href={resolve(props.href)}
    className={`${props.className} font-medium text-brand cursor-pointer hover:underline`}
    itemScope={props.itemScope}
    itemType={props.itemType}
    itemProp={props.itemProp}
    target={props.target}
    onClick={props.onClick}
  >
    {props.children}
  </a>
);

const Button = (props: Props) => (
  <a
    href={resolve(props.href)}
    className={`${props.className} font-medium px-6 py-3 bg-brandBtn text-white hover:underline focus:ring-4 focus:ring-brand rounded-full`}
    itemScope={props.itemScope}
    itemType={props.itemType}
    itemProp={props.itemProp}
    target={props.target}
    onClick={props.onClick}
  >
    {props.children}
  </a>
);

const Paragraph = (props: Props) => (
  <a
    href={resolve(props.href)}
    className={`${props.className} font-medium text-brandRed cursor-pointer underline hover:no-underline`}
    itemScope={props.itemScope}
    itemType={props.itemType}
    itemProp={props.itemProp}
    target={props.target}
    onClick={props.onClick}
  >
    {props.children}
  </a>
);

const IconLink = (props: Props) => (
  <a
    href={resolve(props.href)}
    className={`${props.className} inline-flex items-center font-medium text-brand hover:underline`}
    itemScope={props.itemScope}
    itemType={props.itemType}
    itemProp={props.itemProp}
    target={props.target}
    onClick={props.onClick}
  >
    {props.children}
  </a>
);

export default {
  Button,
  Default,
  IconLink,
  Paragraph,
};
