import * as React from 'react';

interface InputProps extends React.HTMLProps<HTMLInputElement> {
  id: string;
  label: string;
  containerClassName?: string;
}

const Input = (props: InputProps) => (
  <div className={`${props.containerClassName} mb-6`}>
    <label
      htmlFor={props.id}
      className="block mb-2 text-sm font-medium text-gray-700 "
    >
      {props.label}
    </label>
    <input
      type={props.type}
      className={`${props.className} bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-gray-800 focus:border-gray-500 block w-full p-2.5 700 600 400  :ring-gray-300 :border-gray-500`}
      readOnly={props.readOnly}
      onChange={props.onChange}
      required={props.required}
      name={props.name}
      value={props.value}
    />
  </div>
);

interface IconInputProps extends InputProps {
  icon: JSX.Element;
}

const IconInput = (props: IconInputProps) => (
  <div className={`${props.containerClassName} mb-6`}>
    <label
      htmlFor={props.id}
      className="block mb-2 text-sm font-medium text-gray-700 "
    >
      {props.label}
    </label>
    <div className="relative">
      <div className="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
        {props.icon}
      </div>
      <input
        type={props.type}
        className={`${props.className} pl-10 block w-full bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-gray-800 focus:border-gray-500 p-2.5 700 600 400  :ring-gray-300 :border-gray-500`}
        readOnly={props.readOnly}
        onChange={props.onChange}
        required={props.required}
        name={props.name}
        value={props.value}
      />
    </div>
  </div>
);

interface TextAreaProps extends React.HTMLProps<HTMLTextAreaElement> {
  id: string;
  label: string;
  containerClassName?: string;
}

const Textarea = (props: TextAreaProps) => (
  <div className={`${props.containerClassName} mb-6`}>
    <label
      htmlFor={props.id}
      className="block mb-2 text-sm font-medium text-gray-700 "
    >
      {props.label}
    </label>
    <textarea
      id="message"
      rows={props.rows}
      className={`${props.className} block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-gray-800 focus:border-gray-500 700 600 400  :gray-blue-300 :border-gray-500`}
      placeholder={props.placeholder}
      required={props.required}
      onChange={props.onChange}
      name={props.name}
      value={props.value}
    ></textarea>
  </div>
);

export default {
  IconInput,
  Input,
  Textarea,
};
