import * as React from 'react';

const Default = (props: React.HTMLProps<HTMLAnchorElement>) => (
  <a
    href={props.href}
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

const Button = (props: React.HTMLProps<HTMLAnchorElement>) => (
  <a
    href={props.href}
    className={`${props.className} font-medium bg-brand text-white hover:underline focus:ring-4 focus:ring-brand rounded-full`}
    itemScope={props.itemScope}
    itemType={props.itemType}
    itemProp={props.itemProp}
    target={props.target}
    onClick={props.onClick}
  >
    {props.children}
  </a>
);

const Paragraph = (props: React.HTMLProps<HTMLAnchorElement>) => (
  <a
    href={props.href}
    className={`${props.className} font-medium text-brand cursor-pointer underline hover:no-underline`}
    itemScope={props.itemScope}
    itemType={props.itemType}
    itemProp={props.itemProp}
    target={props.target}
    onClick={props.onClick}
  >
    {props.children}
  </a>
);

const IconLink = (props: React.HTMLProps<HTMLAnchorElement>) => (
  <a
    href={props.href}
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
