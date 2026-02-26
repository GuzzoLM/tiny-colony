# Task & Action System Design

## Overview
The Task system defines how pawns perform work in the simulation.

A Task represents a high-level intent (e.g. Chop Tree, Haul Wood, Build Wall).
A Task is composed of a sequence of Actions, which are atomic, interruptible, and deterministic steps.

This structure ensures:
* Clear execution flow
* Easy debugging
* Interruptibility
* Failure handling
* Scalability for large pawn counts
* Future extensibility (stewards, coordination, AI behaviors)

## Core Concepts
### Task
A Task is a container of ordered Actions.
It represents a complete unit of work from start to finish.

Example:
ChopTree Task
  1. MoveTo(Tree)
  2. Chop(Tree)
  3. MoveTo(Stockpile)
  4. Drop(Wood)
  → Completed

A Task has:
* actions: Vec<Action>
* current_action_index
* status: Pending | Running | Completed | Failed
* Optional metadata (target id, priority, assigned pawn, etc.)

A Task progresses only when its current Action reports Success.

### Action
An Action is a single atomic step that:

* Can run across multiple ticks
* Can fail
* Can be interrupted safely
* Has deterministic behavior

Actions should be meaningful units of work.

Good examples:
* MoveTo(target)
* Chop(target)
* Drop(item)
* Wait(duration)

Bad examples:
* MoveOneTile
* RotateLeft
* PlayAnimationFrame

#### Action Lifecycle
Each Action implements:
```Rust
fn tick(&mut self, world: &mut World, pawn: EntityId) -> ActionResult
```

Where:
```Rust
enum ActionResult {
    InProgress,
    Success,
    Failed,
}
```

Behavior:
* InProgress → continue next tick
* Success → Task advances to next Action
* Failed → Task terminates (or triggers retry logic)

#### Task Execution Flow
Task is assigned to pawn.
Pawn begins executing first Action.

Each tick:
* Current Action tick() runs.
* If Success, advance to next Action.
* If Failed, mark Task as failed.
* When all Actions succeed → Task is completed.

#### Planning vs Execution
Planning and Execution are separated.

##### Planner
Responsible for:
* Selecting targets
* Creating concrete Tasks
* Deciding priorities

Example:
Planner decides:
  Pawn 3 should chop Tree #42

Planner creates:
  ChopTree(tree_id = 42)

##### Task
Responsible only for execution of predefined Actions.
Tasks should not perform heavy search or global planning logic.

This separation improves:
* Performance
* Determinism
* Scalability
* Debuggability

Example: Chop Tree

Planner selects Tree #42.

Task created:
Task: ChopTree(tree_id=42)

Actions:
  1. MoveTo(42)
  2. PerformWork(target=42, duration=3s)
  3. MoveToNearestStockpile
  4. DropItem

Execution guarantees:
* Pawn walks to tree
* Chops over multiple ticks
* Picks up resulting wood
* Delivers it
* Marks Task completed

## Failure Scenarios
Actions may fail due to:
* Target destroyed
* Path blocked
* No stockpile space
* Pawn interrupted

Possible strategies:
* Immediate Task failure
* Retry current Action
* Re-plan via Planner
* Fallback behavior (Idle/Wander)

Initial implementation can simply:
Fail the Task on any Action failure

## Design Principles
Actions must be atomic but meaningful
Tasks are deterministic sequences
Planning is external to Tasks
Actions must be interruptible
Tasks must be debuggable (log current action index)
Avoid per-tick expensive searches inside Actions

### Future Extensions (Not Required Initially)
Soft/Hard target claims
Task priorities
Partial progress memory
Parallel composite tasks
Behavior trees on top of Task system
Steward coordination modifiers
Task cancellation & reassignment

## Minimal Initial Implementation Scope
For first version:
Sequential Tasks only
No branching
No retries
Fail-fast on errors
No coordination logic yet
Keep it simple.

## Summary
The Task & Action system provides:
Clean execution flow
Modular behavior
Predictable scaling
Easy extension for complex colony behaviors
This system forms the backbone of pawn behavior in the simulation.