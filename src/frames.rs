/// Walking animation frames (ASCII art).
/// Each frame represents one step in the walking cycle.
pub const FRAMES: &[&str] = &[
    // Frame 0: standing / right foot forward
    r"
   O
  /|\
  / \
 /   \
",
    // Frame 1: mid-step right
    r"
   O
  /|\
   |
  / \
",
    // Frame 2: legs together
    r"
   O
  /|\
   |
   |
",
    // Frame 3: mid-step left
    r"
   O
  /|\
   |
  \ /
",
    // Frame 4: left foot forward
    r"
   O
  /|\
  \ /
 \   /
",
    // Frame 5: mid-step back
    r"
   O
  /|\
   |
  \ /
",
    // Frame 6: legs together again
    r"
   O
  /|\
   |
   |
",
    // Frame 7: mid-step right again
    r"
   O
  /|\
   |
  / \
",
];
