# Agent System Rules

## Project Structure

```
src/
├── enemy/
│   ├── components.rs  # Enemy, EnemyStats, EnemyState
│   ├── systems.rs     # spawn_enemies, move_enemies, enemy_ai
│   └── mod.rs         # pub mod components; pub mod systems;
├── player/
│   ├── components.rs
│   ├── systems.rs
│   └── mod.rs
├── npc/
│   ├── components.rs
│   ├── systems.rs
│   └── mod.rs
├── base/
│   ├── components.rs      # Shared components like AnimationIndices
│   ├── resources.rs
│   └── mod.rs
├── health/
│   ├── components.rs      # Health, Dead
│   ├── systems.rs
│   └── mod.rs
├── game/
│   ├── components.rs      # GameState, GameRules
│   ├── systems.rs         # Game state management
│   └── mod.rs
└── main.rs
```

## File Descriptions

### components.rs

- **Pure data structs**: No logic, just fields
- **Entity-specific components**: EnemyStats, PlayerWeapons, NPCDialogue
- **Tag components**: TagEnemy, TagPlayer (for queries)
- **State structs**: EnemyState { moving: bool, facing: Vec3 }

### systems.rs

- **Behavior functions**: spawn_enemies, move_enemy, enemy_ai
- **Plugin definition**: EnemyPlugin with system registration
- **Query entities**: Process components, modify state
- **Handle events**: Collision detection, damage dealing

### resources.rs

- **Global state**: GameRules, SpawnTimers
- **Shared configuration**: Settings that multiple systems need
- **Resource structs**: Data that exists once per app

### mod.rs

- **Module exports**: `pub mod components; pub mod systems;`
- **Re-exports**: `pub use components::*;` if needed

## Component Rules

- **One file per entity type**: `enemy/components.rs`, `player/components.rs`
- **Shared components get their own folder**: `health/`, `base/`, etc.
- **Entity-specific data only**: EnemyStats, PlayerWeapons, NPCDialogue

## System Rules

- **Systems stay with their entity**: enemy systems in `enemy/systems.rs`
- **Shared systems in their own folders**: `health/systems.rs`, `hurt/systems.rs`
- **One system per behavior**: `move_enemies`, `enemy_ai`, `spawn_enemies`

## Plugin Registration

```rust
// main.rs
App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(GameStatePlugin)  // manages states
    .add_plugins(EnemyPlugin)      // enemy-specific
    .add_plugins(PlayerPlugin)     // player-specific
    .add_plugins(HealthPlugin)     // health system
    .run();

// enemy/systems.rs
pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_enemy_timer)
           .add_systems(Update, (spawn_enemies, move_enemy, hurt_enemy)
               .run_if(in_state(GameState::Playing)));
    }
}
```

## Game State Rules

- **State-specific systems**: Use `run_if(in_state(GameState::Playing))`
- **Setup on enter**: `OnEnter(GameState::Playing)` for spawners/timers
- **Cleanup on exit**: `OnExit(GameState::Playing)` for despawning
- **Pause systems**: Separate update sets for Playing vs Paused
