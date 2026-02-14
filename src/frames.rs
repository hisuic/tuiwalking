/// Walking animation frames - large realistic stick figure.
/// Each frame is 12 chars wide x 10 lines tall.
pub const FRAMES: &[&str] = &[
    // Frame 0: right leg forward, left arm forward
    concat!(
        "    _O_     \n",
        "   / | \\   \n",
        "  /  |  \\  \n",
        "     |      \n",
        "     |      \n",
        "    / \\    \n",
        "   /   \\   \n",
        "  /     \\  \n",
        " /       |  \n",
        "_/        | \n",
    ),
    // Frame 1: mid stride
    concat!(
        "     O      \n",
        "   / | \\   \n",
        "  /  |  |   \n",
        "     |      \n",
        "     |      \n",
        "    / \\    \n",
        "   /   |    \n",
        "  |    |    \n",
        "  |     \\  \n",
        " _/      \\_ \n",
    ),
    // Frame 2: legs passing
    concat!(
        "     O      \n",
        "    /|\\    \n",
        "   / | \\   \n",
        "     |      \n",
        "     |      \n",
        "     |      \n",
        "    / \\    \n",
        "   |   |    \n",
        "   |   |    \n",
        "  _/   \\_ \n",
    ),
    // Frame 3: left leg forward, right arm forward
    concat!(
        "   _O_      \n",
        "  / | \\    \n",
        " /  |  \\   \n",
        "     |      \n",
        "     |      \n",
        "    / \\    \n",
        "   /   \\   \n",
        "  /     \\  \n",
        " |       \\ \n",
        " |        \\_\n",
    ),
    // Frame 4: mid stride back
    concat!(
        "     O      \n",
        "   / | \\   \n",
        "  |  |  \\  \n",
        "     |      \n",
        "     |      \n",
        "    / \\    \n",
        "   |   \\   \n",
        "   |    |   \n",
        "  /     |   \n",
        " /      \\_ \n",
    ),
    // Frame 5: legs passing again
    concat!(
        "     O      \n",
        "    /|\\    \n",
        "   / | \\   \n",
        "     |      \n",
        "     |      \n",
        "     |      \n",
        "    / \\    \n",
        "   |   |    \n",
        "   |   |    \n",
        "  _/   \\_ \n",
    ),
];

/// Width of each frame in columns.
pub const FRAME_WIDTH: u16 = 12;
