use array2d::Array2D;

#[derive(Clone, Copy, Default)]
struct Plot {
    position: (usize, usize),
    plant: char,
    up: char,
    down: char,
    left: char,
    right: char,
    fences: usize,
    visited: bool,
}

fn main() {
    // Read Input
    let input = include_str!("../input.txt");

    // Define Variables
    let mut garden_chars: Array2D<char> = Array2D::filled_with(
        '-',
        input.lines().into_iter().clone().next().unwrap().len(),
        input.lines().into_iter().count(),
    );
    let mut garden_plots: Array2D<Plot> = Array2D::filled_with(
        Plot {
            position: (0, 0),
            plant: '-',
            up: '-',
            down: '-',
            left: '-',
            right: '-',
            fences: 0,
            visited: false,
        },
        input.lines().into_iter().clone().next().unwrap().len(),
        input.lines().into_iter().count(),
    );

    // Fill garden with chars
    for line in 0..garden_chars.column_len() {
        for pos in 0..garden_chars.row_len() {
            garden_chars[(line, pos)] = input.lines().nth(line).unwrap().chars().nth(pos).unwrap();
        }
    }

    // Fill garden with plots
    for line in 0..garden_chars.column_len() {
        for pos in 0..garden_chars.row_len() {
            // top left
            if line == 0 && pos == 0 {
                garden_plots[(line, pos)] = Plot {
                    position: (line, pos),
                    plant: garden_chars[(line, pos)],
                    up: '-',
                    down: garden_chars[(line + 1, pos)],
                    left: '-',
                    right: garden_chars[(line, pos + 1)],
                    fences: 0,
                    visited: false,
                };
            }
            // top middle
            else if line == 0 && pos > 0 && pos < garden_chars.row_len() - 1 {
                garden_plots[(line, pos)] = Plot {
                    position: (line, pos),
                    plant: garden_chars[(line, pos)],
                    up: '-',
                    down: garden_chars[(line + 1, pos)],
                    left: garden_chars[(line, pos - 1)],
                    right: garden_chars[(line, pos + 1)],
                    fences: 0,
                    visited: false,
                };
            }
            // top right
            else if line == 0 && pos == garden_chars.row_len() - 1 {
                garden_plots[(line, pos)] = Plot {
                    position: (line, pos),
                    plant: garden_chars[(line, pos)],
                    up: '-',
                    down: garden_chars[(line + 1, pos)],
                    left: garden_chars[(line, pos - 1)],
                    right: '-',
                    fences: 0,
                    visited: false,
                };
            }
            // left middle
            else if line > 0 && line < garden_chars.column_len() - 1 && pos == 0 {
                garden_plots[(line, pos)] = Plot {
                    position: (line, pos),
                    plant: garden_chars[(line, pos)],
                    up: garden_chars[(line - 1, pos)],
                    down: garden_chars[(line + 1, pos)],
                    left: '-',
                    right: garden_chars[(line, pos + 1)],
                    fences: 0,
                    visited: false,
                };
            }
            // left bottom
            else if line == garden_chars.column_len() - 1 && pos == 0 {
                garden_plots[(line, pos)] = Plot {
                    position: (line, pos),
                    plant: garden_chars[(line, pos)],
                    up: garden_chars[(line - 1, pos)],
                    down: '-',
                    left: '-',
                    right: garden_chars[(line, pos + 1)],
                    fences: 0,
                    visited: false,
                };
            }
            // right middle
            else if line > 0
                && line < garden_chars.column_len() - 1
                && pos == garden_chars.row_len() - 1
            {
                garden_plots[(line, pos)] = Plot {
                    position: (line, pos),
                    plant: garden_chars[(line, pos)],
                    up: garden_chars[(line - 1, pos)],
                    down: garden_chars[(line + 1, pos)],
                    left: garden_chars[(line, pos - 1)],
                    right: '-',
                    fences: 0,
                    visited: false,
                };
            }
            // right bottom
            else if line == garden_chars.column_len() - 1 && pos == garden_chars.row_len() - 1 {
                garden_plots[(line, pos)] = Plot {
                    position: (line, pos),
                    plant: garden_chars[(line, pos)],
                    up: garden_chars[(line - 1, pos)],
                    down: '-',
                    left: garden_chars[(line, pos - 1)],
                    right: '-',
                    fences: 0,
                    visited: false,
                };
            }
            // bottom middle
            else if line == garden_chars.column_len() - 1
                && pos > 0
                && pos < garden_chars.row_len() - 1
            {
                garden_plots[(line, pos)] = Plot {
                    position: (line, pos),
                    plant: garden_chars[(line, pos)],
                    up: garden_chars[(line - 1, pos)],
                    down: '-',
                    left: garden_chars[(line, pos - 1)],
                    right: garden_chars[(line, pos + 1)],
                    fences: 0,
                    visited: false,
                };
            }
            // middle middle
            else {
                garden_plots[(line, pos)] = Plot {
                    position: (line, pos),
                    plant: garden_chars[(line, pos)],
                    up: garden_chars[(line - 1, pos)],
                    down: garden_chars[(line + 1, pos)],
                    left: garden_chars[(line, pos - 1)],
                    right: garden_chars[(line, pos + 1)],
                    fences: 0,
                    visited: false,
                };
            }
        }
    }

    // Update fence values on plots
    for line in 0..garden_chars.column_len() {
        for pos in 0..garden_chars.row_len() {
            let mut current_plot = garden_plots[(line, pos)];

            let mut up_fence = 0;
            let mut down_fence = 0;
            let mut left_fence = 0;
            let mut right_fence = 0;

            if current_plot.up == current_plot.plant {
                up_fence = 1;
            }
            if current_plot.down == current_plot.plant {
                down_fence = 1;
            }
            if current_plot.left == current_plot.plant {
                left_fence = 1;
            }
            if current_plot.right == current_plot.plant {
                right_fence = 1;
            }

            current_plot.fences = up_fence + down_fence + left_fence + right_fence;
        }
    }

    // Search for groups
    for line in 0..garden_chars.column_len() {
        for pos in 0..garden_chars.row_len() {
            let mut current_plot = garden_plots[(line, pos)];
            if !current_plot.visited {
                println!("First base!");
                let group_area = calculate_area(&garden_plots, &mut current_plot);
                println!("Group Area: {:?}", group_area);
            }
        }
    }
}

fn calculate_area(garden_plots: &Array2D<Plot>, plot: &mut Plot) -> usize {
    println!(
        "Entered! plot char: {:?} on pos: {:?}",
        plot.plant, plot.position
    );

    plot.visited = true;
    let mut upper_values = 0;
    let mut lower_values = 0;
    let mut left_values = 0;
    let mut right_values = 0;

    // Check upper plot
    if plot.position.0 > 0 {
        let mut upper_plot = garden_plots[(plot.position.0 - 1, plot.position.1)];
        if plot.plant == upper_plot.plant && !upper_plot.visited {
            upper_values = calculate_area(garden_plots, &mut upper_plot);
        }
    }
    // Check lower plot
    if plot.position.0 < garden_plots.column_len() - 1 {
        let mut lower_plot = garden_plots[(plot.position.0 + 1, plot.position.1)];
        if plot.plant == lower_plot.plant && !lower_plot.visited {
            lower_values = calculate_area(garden_plots, &mut lower_plot);
        }
    }
    // Check left plot
    if plot.position.1 > 0 {
        let mut left_plot = garden_plots[(plot.position.0, plot.position.1 - 1)];
        if plot.plant == left_plot.plant && !left_plot.visited {
            left_values = calculate_area(garden_plots, &mut left_plot);
        }
    }
    // Check right plot
    if plot.position.1 < garden_plots.row_len() - 1 {
        let mut right_plot = garden_plots[(plot.position.0, plot.position.1 + 1)];
        if plot.plant == right_plot.plant && !right_plot.visited {
            right_values = calculate_area(garden_plots, &mut right_plot);
        }
    }

    return upper_values + lower_values + left_values + right_values + 1;
}

fn calculate_perimeter(garden_plots: &Array2D<Plot>, plot: Plot) -> usize {
    // TODO
    return 0;
}
