# Movement, Occupancy & Pathfinding

### ZZ Sim Colony -- Scalable Pawn Navigation Model

## 🎯 Design Goals

Our navigation system is designed for:

-   **1 pawn per tile**
-   Predictable performance at **hundreds or thousands of pawns**
-   Minimal per-tick cost (movement must be cheap)
-   Rare and controlled path recalculation
-   Deterministic and debuggable behavior

The core principle:

> **Planning is expensive. Moving is cheap.**

------------------------------------------------------------------------

# Tile Occupancy Model

## One Pawn Per Tile

We enforce a strict invariant:

Each walkable tile can contain at most one pawn.

### Occupancy Grid

We maintain a global occupancy grid:

`occupancy[tile] = Option<PawnId>`

This allows O(1) checks:

-   Is tile free?
-   Who occupies it?
-   Reserve tile for next step

### Movement Operation (O(1))

When a pawn wants to move:

1.  Read next tile from its path
2.  Check `occupancy[next_tile]`
3.  If empty:
    -   Remove pawn from current tile
    -   Insert pawn into next tile
4.  If occupied:
    -   Trigger obstruction handling

This keeps per-tick movement:

O(number_of_pawns)

No scanning. No pathfinding. No global logic.

------------------------------------------------------------------------

# Pathfinding Model

## Planning Happens Before Walking

A pawn **never pathfinds while moving**.

When a pawn receives a job:

1.  Compute full path to destination
2.  Store path as: `Vec<Tile>`
3.  Begin consuming path step by step

Movement system only reads from this vector.

## When Do We Pathfind?

Pathfinding is triggered only when:

-   Pawn receives a new job
-   Destination changes
-   Path becomes invalid (rare)
-   Pawn is considered "stuck"

Pathfinding is **never** triggered every tick.

## Pathfinding Budget (Optional now, but necessary in the future)

To avoid frame spikes:

-   Maintain a queue of pathfinding requests
-   Process only N requests per tick
-   Others wait

This converts spikes into stable performance.

------------------------------------------------------------------------

# Sudden Obstruction Handling

Even with planning, the world is dynamic:

-   Another pawn may step into your next tile
-   A door may close
-   A new wall may appear

We resolve this in lightweight steps.

## Step 1: Wait Briefly

If next tile is occupied:

-   Increment blocked_ticks
-   Wait 1 tick
-   Try again next tick

Many collisions resolve naturally.

## Step 2: Limited Retries

If blocked for MAX_WAIT_TICKS (e.g., 5 ticks):

-   Attempt small local resolution

## Step 3: Local Move-Out-of-Way

Instead of replanning immediately:

1.  Check 4 neighboring tiles
2.  If one is free and walkable:
    -   Step aside
    -   Reset blocked counter
    -   Retry original path next tick

This simulates natural crowd flow without global repath.

Cost: constant time (max 4 checks)

## Step 4: Repath with Backoff

If still blocked after side-step:

-   Request path recalculation
-   Enter backoff state:
    -   Retry after 10 ticks
    -   If still stuck → 20 ticks
    -   Then 40, etc.

This prevents "repath storms."

#  Why This Scales

The key is separating systems:

  System              Frequency       Complexity
  ------------------- --------------- -----------------------
  Movement            Every tick      O(n)
  Obstruction check   Rare per pawn   O(1)
  Pathfinding         Event-driven    Expensive but limited
  Job search          Event-driven    Indexed, not scanning

At 1000 pawns:

-   Movement remains cheap.
-   Only a small fraction pathfind per tick.
-   No global rescans.
-   No O(n²) interactions.

------------------------------------------------------------------------

# Stuck Detection

A pawn is considered stuck if:

-   It failed movement for X retries
-   It has not changed tile for Y ticks

If stuck:

-   Cancel current path
-   Mark job as temporarily unreachable
-   Enter idle state
-   Retry job search later

This prevents infinite loops.

------------------------------------------------------------------------

# Optional Future Improvements

For very large colonies:

-   Hierarchical pathfinding (chunk-based)
-   Path caching
-   Flow fields for common destinations
-   Lower update frequency for idle pawns (LOD simulation)
-   Priority lanes for high-traffic areas

------------------------------------------------------------------------

# Core Philosophy

The system is built around three principles:

1.  **Movement must be extremely cheap**
2.  **Replanning must be rare**
3.  **Crowd conflicts must resolve locally**

If those three are respected, scaling to thousands of pawns becomes
feasible.
