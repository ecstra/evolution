/* tslint:disable */
/* eslint-disable */
export class Agent {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  x: number;
  y: number;
  rotation: number;
}
export class Input {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  x: number;
  y: number;
}
export class Simulation {
  free(): void;
  [Symbol.dispose](): void;
  complete_one_evolution(): string;
  constructor();
  step(): void;
  world(): World;
}
export class World {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  agents: Agent[];
  inputs: Input[];
}
