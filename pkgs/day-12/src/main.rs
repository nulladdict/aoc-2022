use pathfinding::prelude::{bfs, Matrix};

fn main() {
    let (matrix, start, end) = parse(include_str!("in").trim_end());
    dbg!(part1(&matrix, start, end));
    dbg!(part2(&matrix, end));
}

fn parse(input: &str) -> (Matrix<u8>, (usize, usize), (usize, usize)) {
    let mut matrix = Matrix::from_rows(input.lines().map(|row| row.bytes())).unwrap();
    let start = matrix
        .indices()
        .find(|&point| matrix[point] == b'S')
        .unwrap();
    let end = matrix
        .indices()
        .find(|&point| matrix[point] == b'E')
        .unwrap();
    matrix[start] = b'a';
    matrix[end] = b'z';
    (matrix, start, end)
}

fn part1(matrix: &Matrix<u8>, start: (usize, usize), end: (usize, usize)) -> usize {
    bfs(
        &start,
        |&point| {
            matrix
                .neighbours(point, false)
                .filter(move |&neighbour| matrix[neighbour] <= matrix[point] + 1)
        },
        |&point| point == end,
    )
    .unwrap()
    .len()
        - 1
}

fn part2(matrix: &Matrix<u8>, end: (usize, usize)) -> usize {
    bfs(
        &end,
        |&point| {
            matrix
                .neighbours(point, false)
                .filter(move |&neighbour| matrix[point] <= matrix[neighbour] + 1)
        },
        |&point| matrix[point] == b'a',
    )
    .unwrap()
    .len()
        - 1
}
