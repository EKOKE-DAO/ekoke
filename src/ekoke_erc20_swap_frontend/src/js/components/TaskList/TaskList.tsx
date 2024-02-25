import * as React from 'react';
import Container from '../reusable/Container';
import Button from '../reusable/Button';
import TaskEntry from './TaskEntry';

export type Result = true | { error: string };

export enum TaskStatus {
  Pending,
  Running,
  Success,
  Error,
  Aborted,
}

export interface Task {
  label: string;
  action: () => Promise<Result>;
}

interface TaskListProps {
  onDone: () => void;
  run: boolean;
  tasks: Task[];
  title: string;
}

const TaskList = ({ onDone, run, tasks, title }: TaskListProps) => {
  const [tasksStatus, setTasksStatus] = React.useState<TaskStatus[]>(
    tasks.map(() => TaskStatus.Pending),
  );
  const [error, setError] = React.useState<string | null>(null);

  React.useEffect(() => {
    if (!run) {
      return;
    }
    // check if a task is running
    const running = tasksStatus.some((status) => status === TaskStatus.Running);
    if (running) {
      return;
    }
    // get the first pending task
    const index = tasksStatus.indexOf(TaskStatus.Pending);
    if (index === -1) {
      return;
    }

    const task = tasks[index];
    // set the task status to running
    setTasksStatus((prev) => {
      const next = [...prev];
      next[index] = TaskStatus.Running;
      return next;
    });

    // run the task
    task
      .action()
      .then((result) => {
        setTasksStatus((prev) => {
          const newStates = [...prev];
          newStates[index] =
            result === true ? TaskStatus.Success : TaskStatus.Error;
          if (result !== true) {
            setError(result.error);
            // abort all the tasks
            for (let i = index + 1; i < tasks.length; i++) {
              newStates[i] = TaskStatus.Aborted;
            }
          }

          return newStates;
        });
      })
      .catch(() => {
        setTasksStatus((prev) => {
          const newStates = [...prev];
          newStates[index] = TaskStatus.Error;
          // abort all the tasks
          for (let i = index + 1; i < tasks.length; i++) {
            newStates[i] = TaskStatus.Aborted;
          }
          return newStates;
        });
      });
  }, [tasksStatus, run]);

  React.useEffect(() => {
    setTasksStatus(tasks.map(() => TaskStatus.Pending));
  }, [tasks]);

  const allTasksCompleted = tasksStatus.every(
    (status) => status !== TaskStatus.Pending && status !== TaskStatus.Running,
  );

  if (!run) {
    return null;
  }

  return (
    <Container.Container className="h-screen left-0 overflow-hidden fixed right-0 top-0 w-screen z-50">
      <Container.Container className="bg-gray-800/60 h-screen w-screen" />
      <Container.Container className="bg-white bottom-0 h-fit sm:h-[70vh] sm:rounded-t-xl left-0 m-auto p-8 fixed right-0 top-0 sm:top-auto sm:bottom-0 w-fit min-w-[25%] sm:w-full">
        <Container.FlexCols className="gap-4 text-lg p-4">
          <span className="text-lg text-center block text-text">{title}</span>
          {tasks.map((task, index) => (
            <TaskEntry key={index} task={task} state={tasksStatus[index]} />
          ))}
        </Container.FlexCols>
        {allTasksCompleted && (
          <Container.FlexCols className="justify-center items-center m-auto w-3/6">
            {error && <span className="text-lg text-red-700">{error}</span>}
            <Button.Cta onClick={onDone}>{error ? 'Close' : 'Done'}</Button.Cta>
          </Container.FlexCols>
        )}
      </Container.Container>
    </Container.Container>
  );
};

export default TaskList;
