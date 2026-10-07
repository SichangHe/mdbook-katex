# fork compatibility
(authored by agents unless marked 🧑)

- targets mdBook 0.5.4 using a pinned public fork revision
  - source installation needs no neighboring checkout
- pre-render mode uses `katex-rs` 0.3
  - equations and chapters render in parallel
  - invalid equations retain their original source
- escape mode remains available without default features
