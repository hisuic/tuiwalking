/// Generates scrolling cityscape scenery.

/// A building defined by its width and height (in rows).
pub struct Building {
    pub width: u16,
    pub height: u16,
    pub style: BuildingStyle,
}

pub enum BuildingStyle {
    Plain,
    Windowed,
    Tower,
}

/// Pre-defined city skyline pattern that repeats.
pub fn generate_buildings() -> Vec<Building> {
    vec![
        Building { width: 8, height: 6, style: BuildingStyle::Windowed },
        Building { width: 4, height: 3, style: BuildingStyle::Plain },
        Building { width: 6, height: 9, style: BuildingStyle::Tower },
        Building { width: 10, height: 5, style: BuildingStyle::Windowed },
        Building { width: 3, height: 2, style: BuildingStyle::Plain },
        Building { width: 7, height: 7, style: BuildingStyle::Windowed },
        Building { width: 5, height: 4, style: BuildingStyle::Plain },
        Building { width: 9, height: 10, style: BuildingStyle::Tower },
        Building { width: 6, height: 3, style: BuildingStyle::Plain },
        Building { width: 8, height: 6, style: BuildingStyle::Windowed },
        Building { width: 4, height: 8, style: BuildingStyle::Tower },
        Building { width: 7, height: 4, style: BuildingStyle::Plain },
        Building { width: 5, height: 5, style: BuildingStyle::Windowed },
        Building { width: 11, height: 7, style: BuildingStyle::Tower },
        Building { width: 6, height: 3, style: BuildingStyle::Plain },
        Building { width: 8, height: 5, style: BuildingStyle::Windowed },
    ]
}

/// Render the skyline into a grid of characters.
/// Returns a Vec of Strings, one per row (from top to bottom), of total `width` columns.
/// `sky_height` is the total height available for buildings.
/// `scroll_offset` shifts the view to the right within the repeating pattern.
pub fn render_skyline(width: u16, sky_height: u16, scroll_offset: usize) -> Vec<String> {
    let buildings = generate_buildings();

    // Calculate total pattern width (buildings + 2-char gap between each)
    let total_pattern_width: usize = buildings.iter().map(|b| b.width as usize + 2).sum();

    let mut rows: Vec<Vec<char>> = vec![vec![' '; width as usize]; sky_height as usize];

    // Fill in buildings at their positions, considering scroll offset
    let mut col_in_pattern: usize = 0;
    for building in buildings.iter() {
        let gap = 2;
        let bw = building.width as usize;
        let bh = (building.height as usize).min(sky_height as usize);

        for local_col in 0..bw {
            // Position in pattern space
            let pat_col = col_in_pattern + local_col;
            // Screen column, wrapping with scroll
            let screen_col = (pat_col + total_pattern_width * 100 - scroll_offset % total_pattern_width) % total_pattern_width;

            if screen_col >= width as usize {
                continue;
            }

            for row_from_bottom in 0..bh {
                let row_idx = sky_height as usize - 1 - row_from_bottom;
                let ch = match building.style {
                    BuildingStyle::Plain => {
                        if row_from_bottom == bh - 1 {
                            '▄'
                        } else if local_col == 0 || local_col == bw - 1 {
                            '│'
                        } else {
                            '█'
                        }
                    }
                    BuildingStyle::Windowed => {
                        if row_from_bottom == bh - 1 {
                            '▄'
                        } else if local_col == 0 || local_col == bw - 1 {
                            '│'
                        } else if row_from_bottom % 2 == 1 && local_col % 2 == 1 {
                            '▪'
                        } else {
                            '█'
                        }
                    }
                    BuildingStyle::Tower => {
                        if row_from_bottom == bh - 1 && local_col == bw / 2 {
                            '▲'
                        } else if row_from_bottom == bh - 1 {
                            '▄'
                        } else if local_col == 0 || local_col == bw - 1 {
                            '│'
                        } else if row_from_bottom % 3 == 1 && local_col % 2 == 1 {
                            '░'
                        } else {
                            '█'
                        }
                    }
                };
                rows[row_idx][screen_col] = ch;
            }
        }
        col_in_pattern += bw + gap;
    }

    rows.iter().map(|r| r.iter().collect()).collect()
}
