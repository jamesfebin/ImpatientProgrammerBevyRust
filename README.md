# The Impatient Programmer's Guide to Bevy and Rust

<video width="100%" controls src="https://github.com/user-attachments/assets/ab700b79-be21-4f0a-9147-bb6ee4ca1116"></video>

[Read the first 7 chapters free.](https://aibodh.com/books/the-impatient-programmers-guide-to-bevy-and-rust/) The [paid ebook](https://febinjohnjames.gumroad.com/l/the-impatient-programmers-guide-to-bevy-and-rust) includes chapters 8-11, and I'm actively working on more content. As an early access release, it's currently available at a discounted price. [Chapter 12 is also free to read.](https://aibodh.com/posts/bevy-rust-game-development-chapter-12/)

## Chapters

### [Chapter 1: Let There Be a Player](https://aibodh.com/posts/bevy-rust-game-development-chapter-1/)

Learn to build a video game from scratch using Rust and Bevy. This first chapter covers setting up your game world, creating a player character, and implementing movement and animations.

### [Chapter 2: Let There Be a World](https://aibodh.com/posts/bevy-rust-game-development-chapter-2/)

Learn procedural generation techniques to create dynamic game worlds.

### [Chapter 3: Let The Data Flow](https://aibodh.com/posts/bevy-rust-game-development-chapter-3/)

Learn to build a data-driven character system in Bevy. We'll use a RON file to configure character attributes and animations, create a generic animation engine that handles walk, run, and jump animations, and implement character switching.

### [Chapter 4: Let There Be Collisions](https://aibodh.com/posts/bevy-rust-game-development-chapter-4/)

Let's make the player interact with the world properly, no more walking through trees, water, or rocks. We'll implement z-ordering so they can walk behind objects, giving your 2D game true depth. Also, you'll build a collision visualizer for debugging.

### [Chapter 5: Let There Be Pickups](https://aibodh.com/posts/bevy-rust-game-development-chapter-5/)

Build an inventory system to collect items from the world, then zoom in and add smooth camera follow.

### [Chapter 6: Let There Be Particles](https://aibodh.com/posts/bevy-rust-game-development-chapter-6/)

Learn to build a particle system with magical powers and create stunning particle effects. Learn custom shaders, additive blending, and how to make your game feel alive.

### [Chapter 7: Let There Be Enemies](https://aibodh.com/posts/bevy-rust-game-development-chapter-7/)

Build intelligent enemies that hunt and attack the player.

### [Chapter 8: Let There Be Damage](https://febinjohnjames.gumroad.com/l/the-impatient-programmers-guide-to-bevy-and-rust)

Add health, damage, and combat mechanics that make your game feel dangerous.

### [Chapter 9: Let The World Scale](https://febinjohnjames.gumroad.com/l/the-impatient-programmers-guide-to-bevy-and-rust)

Break through Wave Function Collapse size limits by stitching multiple maps into one massive world.

### [Chapter 10: Let There Be Memory](https://febinjohnjames.gumroad.com/l/the-impatient-programmers-guide-to-bevy-and-rust)

Learn to save and load your game so players can pick up exactly where they left off.

### [Chapter 11: Let There Be Sound](https://febinjohnjames.gumroad.com/l/the-impatient-programmers-guide-to-bevy-and-rust)

Learn how to add background music and sound effects to your Bevy game.

### [Chapter 12: Let There Be Networking](https://aibodh.com/posts/bevy-rust-game-development-chapter-12/)

Learn how to build multiplayer networking in Bevy with SpacetimeDB.

## Getting Started

Each chapter has its own directory with a complete, runnable project. Navigate to the chapter directory you want to explore and run:

```bash
cd chapter1  # or chapter2, chapter3, chapter4, chapter5, chapter6, chapter7, chapter12
cargo run
```

Note for Linux users on Wayland: if you see rendering artifacts, run the app with the X11 backend:

```bash
WINIT_UNIX_BACKEND=x11 WAYLAND_DISPLAY= cargo run
```

> **Community Tip**: This optimization was pointed out by one of our [community members](https://github.com/jamesfebin/ImpatientProgrammerBevyRust/issues/1), thank you for helping make this tutorial better!

Bevy's default debug configuration can lead to performance issues: scenes that should run smoothly might drop to unplayable framerates, or large assets might take minutes to load.

The Bevy team [documents this issue](https://bevy.org/learn/quick-start/getting-started/setup/#compile-with-performance-optimizations) and provides a solution.

**Add these optimizations to your `Cargo.toml`:**

```toml
# At the bottom of your Cargo.toml

# Enable a small amount of optimization in the dev profile
[profile.dev]
opt-level = 1

# Enable a large amount of optimization in the dev profile for dependencies
[profile.dev.package."*"]
opt-level = 3
```

## Community

- [Join our Discord community](https://discord.com/invite/cD9qEsSjUH) to get notified when new chapters drop
- Connect with me on [Twitter/X](https://x.com/heyfebin)
- Connect with me on [LinkedIn](https://www.linkedin.com/in/febinjohnjames/)

## Assets

- assets/tile_layers: "16x16 Game Assets" by George Bailey, CC-BY 4.0.

## License

**The tutorial code in this repository is licensed under the MIT License.** See the [LICENSE](LICENSE) file for details.
