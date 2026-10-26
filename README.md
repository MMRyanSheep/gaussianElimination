# gaussianElimination

Algebra II Honors project - using 3D to display how gaussian elimination works.

Run with `cargo run`. Use the top navigation to switch between scenes:

- **Plane** (default): edit the three linear equations and apply them to the planes.
- **Cube**: watch a cyan cube outline smoothly grow and shrink around fixed
  coordinate axes (X red, Y green, Z blue; arrows point in the positive direction).
  The cube has twelve edges and no filled faces. Use **Pause / Play**,
  **Speed**, and **Reset** to control the animation. Drag **Scale** to pause and
  inspect a specific size; the panel also shows side length and volume.

Switching scenes preserves equation inputs and cube controls; the cube animation
pauses while the Plane scene is selected. Drag outside the UI to orbit, and scroll
to zoom.

Validate with `cargo check` and `cargo test`.
