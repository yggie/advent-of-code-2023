use super::{problem_2, problem_3};

#[test]
fn test_problem_2_part_1_with_example() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let solution = problem_2::solve_part_1(input);

    assert_eq!(solution, 8, "does not match the provided example");
}

#[test]
fn test_problem_2_part_2_with_example() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let solution = problem_2::solve_part_2(input);

    assert_eq!(solution, 2286, "does not match the provided example");
}

#[test]
fn test_problem_3_part_1_with_example() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let solution = problem_3::solve_part_1(input);

    assert_eq!(solution, 4361, "does not match the example");
}

#[test]
fn test_problem_3_part_1_with_test_case_1() {
    let input = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78..........
.......23...
....90*12...
............
2.2......12.
.*.........*
1.1.......56";

    let solution = problem_3::solve_part_1(input);

    assert_eq!(solution, 413);
}

#[test]
fn test_problem_3_part_1_with_test_case_2() {
    let input = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";

    let solution = problem_3::solve_part_1(input);

    assert_eq!(solution, 925);
}

#[test]
fn test_problem_3_part_2_with_example() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let solution = problem_3::solve_part_2(input);

    assert_eq!(solution, 467835, "must match example");
}
