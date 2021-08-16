struct Cell {
    visited: u8,
    pipes: u8,
    x: usize,
    y: usize,
}

const TOP: u8 = 1 << 0;
const RIGHT: u8 = 1 << 1;
const BOTTOM: u8 = 1 << 2;
const LEFT: u8 = 1 << 3;

fn to_mask(c: char) -> u8 {
    match c {
        '┗' => TOP | RIGHT,
        '┓' => BOTTOM | LEFT,
        '┏' => BOTTOM | RIGHT,
        '┛' => TOP | LEFT,
        '━' => LEFT | RIGHT,
        '┃' => BOTTOM | TOP,
        '┣' => BOTTOM | TOP | RIGHT,
        '┫' => BOTTOM | TOP | LEFT,
        '┳' => LEFT | RIGHT | BOTTOM,
        '┻' => LEFT | RIGHT | TOP,
        '╋' => LEFT | RIGHT | TOP | BOTTOM,
        _ => 0
    }
}

fn to_cells(pipe_map: &[&str]) -> Vec<Vec<Cell>> {
    return pipe_map.iter().enumerate().map(|(y, str)| {
        str.chars().enumerate().map(|(x, c)| { Cell { visited: 0, pipes: to_mask(c), x, y } }).collect()
    }).collect();
}


fn is_leaking(x: usize, y: usize, origin: u8, map: &mut Vec<Vec<Cell>>) -> bool {
    {
        let mut cell = &mut map[y][x];
        if (cell.visited & origin) != 0 {
            return false;
        }
        cell.visited |= origin;
    }

    let pipes = map[y][x].pipes;

    return (pipes & origin) == 0 ||
        (origin != TOP && (pipes & TOP) != 0 && y > 0 && is_leaking(x, y - 1, BOTTOM, map)) ||
        (origin != BOTTOM && (pipes & BOTTOM) != 0 && y < (map.len() - 1) && is_leaking(x, y + 1, TOP, map)) ||
        (origin != LEFT && (pipes & LEFT) != 0 && x > 0 && is_leaking(x - 1, y, RIGHT, map)) ||
        (origin != RIGHT && (pipes & RIGHT) != 0 && x < (map[0].len() - 1) && is_leaking(x + 1, y, LEFT, map));
}

pub fn check_pipe(pipe_map: &[&str]) -> bool {
    let mut cells = to_cells(pipe_map);
    let map_width = cells[0].len();
    let map_height = cells.len();
    !((0..map_height).any(|y| {
        (cells[y][0].pipes & LEFT) != 0 && is_leaking(0, y, LEFT, &mut cells) ||
            (cells[y].last().unwrap().pipes & RIGHT) != 0 && is_leaking(map_width - 1, y, RIGHT, &mut cells)
    }) || (0..map_width).any(|x| {
        (cells[0][x].pipes & TOP) != 0 && is_leaking(x, 0, TOP, &mut cells) ||
            (cells.last().unwrap()[x].pipes & BOTTOM) != 0 && is_leaking(x, map_height - 1, BOTTOM, &mut cells)
    }))
}