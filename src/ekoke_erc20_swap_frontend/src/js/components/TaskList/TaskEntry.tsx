import * as React from 'react';
import * as Icon from 'react-feather';
import { Task, TaskStatus } from './TaskList';
import Container from '../reusable/Container';

interface Props {
  task: Task;
  state: TaskStatus;
}

const TaskEntry = ({ task, state }: Props) => (
  <Container.FlexRow className="items-center justify-between w-full border-b border-gray-300 py-4">
    <Container.Container>
      <span className="text-text text-lg">{task.label}</span>
    </Container.Container>
    <Container.Container>
      {state === TaskStatus.Pending && (
        <Icon.Clock className="text-text" size={24} />
      )}
      {state === TaskStatus.Running && (
        <Icon.Loader className="text-text animate-spin" size={24} />
      )}
      {state === TaskStatus.Success && (
        <Icon.CheckCircle className="text-green-700" size={24} />
      )}
      {state === TaskStatus.Error && (
        <Icon.XCircle className="text-red-700" size={24} />
      )}
      {state === TaskStatus.Aborted && (
        <Icon.Clock className="text-text" size={24} />
      )}
    </Container.Container>
  </Container.FlexRow>
);

export default TaskEntry;
