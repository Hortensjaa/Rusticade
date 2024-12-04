# Rusticade - 2D Platformer Game Engine

This project is a 2D platformer game engine built using the [ggez](https://ggez.rs/) library. It provides a high-level framework for designing and implementing 2D platformer games with predefined classes for game logic and in-game objects.

## Features

### High-Level Game Classes
- **Config**: A configuration class for defining game-wide settings.
- **Game**: The core game class that handles game logic and updates.

### Predefined Object Types
Each object in the game has the following components:
- **Physics**: Handles movement, gravity, and collisions.
- **Graphics**: Represents the object's visual appearance.
- **Properties**: A customizable dictionary storing numeric values associated with the object.
- **Additional Options**: Type-dependent features such as player health or item effects.

#### Object Types
1. **Player**:
   - A controllable character that interacts with the environment using physics and collision rules.
   - Default attributes:
     - `hp`: Health points. If it drops to 0, the game ends.
     - `score`: Determines the player's win condition if a target score is set in the `Config`.
   - Physics and graphics are customizable.

2. **Platform**:
   - Static objects with barriers.
   - Default behavior includes a top-side barrier, with options for side and bottom barriers.
   - Can trigger custom actions upon collision (different actions for collisions from different directions).

3. **Item**:
   - Static objects without barriers.
   - Executes an action upon collision and disappears afterward.

4. **Creature**:
   - Non-static entities that follow a predefined path (list of vectors).
   - Ignores physical laws such as gravity and collisions.
   - Triggers a custom action upon collision. If the action returns `false`, the creature is removed ("defeated").

## Advanced Features
1. **Customizable Properties**:
   - Every object type includes a mutable dictionary (`properties`) for storing state; .
   - Actions can be dependent on state, giving you big freedom in customizing your game experience with less boiler code.

2. **Graphics Customization**:
   - Default visuals are represented as colored rectangles.
   - Supports custom images defined by the user.
   - Remeber to add your resources folder with game_context(res_path).

3. **Flying Mode**:
   - Disables gravity for player - this feature let you create "horizontal" games like Pacman or Snake.

4. **Extended Game Logic**:
   - Users can add additional logic to the `Game::update()` method, executed before (Game::action_before) and after (Game::action_after) the main game update loop - it allows you to e.g. automatically move player or platforms, because it has complete control after objects added to your game.

## Example use cases
- Create a simple platformer where a player collect items and avoids enemies.
- Make area with different types of platforms (some of them can work like trampolines, doors, teleports or have counters of times you can hit them) - player's goal will be to make it to finish platform; you can add damage effect from falling with huge speed (player.physics.vy)
- Create 2d labirynth with creatures that patrol specific paths and react to player actions - use flying mode to have "bird's-eye view impression
- Use Game::action_after and Game::action_before, to automatically move platforms from up to down and generate new ones (like in Icy Tower); you will also need to set bottom of screen as "deadly point" for player.

## Getting Started
### Prerequisites
- Rust programming language installed.
- The `ggez` library added to your project dependencies.

### Installation
1. Clone this repository:
   ```bash
   git clone https://github.com/Hortensjaa/Rusticade
   ```
2. Build and run the examples:
   ```bash
   cargo run --example basic_game
   ```
3. Run tests:
   ```bash
   cargo test
   ```


