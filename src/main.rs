use std::{thread, time};
use rand::Rng;

fn print_board(board_size: &Vec<usize>, snake: &Vec<Vec<usize>>, food: &Vec<usize>) {
    let mut res: String = "================\n".to_string();
    for i in 0..board_size[0] {
        for j in 0..board_size[1] {
            if food.len() == 2 && i == food[0] && j == food[1] {
                res += "*";
            } else {
                let mut found: bool = false;
                for k in 0..snake.len() {
                    if i == snake[k][0] && j == snake[k][1] {
                        found = true;
                        if k == 0 {
                            res += "\x1b[41m";
                        } else if k == 1 {
                            res += "\x1b[42m";
                        }
                        res += "#";
                        if k < 2 {
                            res += "\x1b[0m";
                        }
                        break;
                    }
                }
                if !found {
                    res += ".";
                }
            }
        }
        res += "\n";
    }
    res += "================";
    println!("{} {}", res, snake.len());
}

fn move_snake(snake: &mut Vec<Vec<usize>>, mv: &Vec<i32>) {
    let mut nxt_x: usize = snake[0][0];
    let mut nxt_y: usize = snake[0][1];
    snake[0][0] = ((snake[0][0] as i32) + mv[0]) as usize;
    snake[0][1] = ((snake[0][1] as i32 + mv[1])) as usize;
    for i in 1..(snake.len()) {
        let tmp_x: usize = snake[i][0];
        let tmp_y: usize = snake[i][1];
        snake[i][0] = nxt_x;
        snake[i][1] = nxt_y;
        nxt_x = tmp_x;
        nxt_y = tmp_y;
    }
}

fn move_snake_copy(snake: &Vec<Vec<usize>>, mv: &Vec<i32>) -> Vec<Vec<usize>> {
    let mut res: Vec<Vec<usize>> = Vec::new();
    res.push(vec![((snake[0][0] as i32) + mv[0]) as usize, ((snake[0][1] as i32 + mv[1])) as usize]);
    for i in 0..(snake.len()-1) {
        res.push(vec![snake[i][0], snake[i][1]]);
    }
    res
}

fn eat_food(snake: &mut Vec<Vec<usize>>, food: &Vec<usize>) {
    snake.insert(0, vec![food[0], food[1]]);
}

fn occupied(snake: &Vec<Vec<usize>>, x: usize, y: usize) -> bool {
    for i in 0..snake.len() {
        if snake[i][0] == x && snake[i][1] == y {
            return true;
        }
    }
    false
}

fn random_upto(n: usize) -> usize {
    assert!(n > 0, "Input must be greater than zero");
    let mut rng = rand::thread_rng();
    rng.gen_range(0..n)
}

fn spawn_food(board_size: &Vec<usize>, snake: &Vec<Vec<usize>>) -> Vec<usize> {
    loop {
        let x: usize = random_upto(board_size[0]);
        let y: usize = random_upto(board_size[1]);
        if ! occupied(snake, x, y) {
            return vec![x, y];
        }
    }
}

fn need_search(snake: &Vec<Vec<usize>>, food: &Vec<usize>) -> bool {
    let mut curr_x: i32 = snake[0][0] as i32;
    let mut curr_y: i32 = snake[0][1] as i32;
    while curr_x != (food[0] as i32) || curr_y != (food[1] as i32) {
        let bearing_x: i32 = (food[0] as i32) - curr_x;
        let bearing_y: i32 = (food[1] as i32) - curr_y;
        if bearing_x.abs() > bearing_y.abs() {
            curr_x += bearing_x.signum();
        } else {
            curr_y += bearing_y.signum();
        }
        if occupied(snake, curr_x as usize, curr_y as usize) {
            return true;
        }
    }
    false
}

fn find_next_move_dfs(board_size: &Vec<usize>, snake: &Vec<Vec<usize>>, food: &Vec<usize>, depth: u32, max_depth: u32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let bearing_x: i32 = (food[0] as i32) - (snake[0][0] as i32);
    let bearing_y: i32 = (food[1] as i32) - (snake[0][1] as i32);
    if bearing_x.abs() <= 1 && bearing_y.abs() <= 1 {
        return vec![0, 0, 1000];
    }
    let mut current_order: Vec<Vec<i32>> = Vec::new();
    if bearing_x.abs() > bearing_y.abs() {
        current_order.push(vec![bearing_x.signum(), 0, 0]);
        current_order.push(vec![0, 1, 0]);
        current_order.push(vec![0, -1, 0]);
        current_order.push(vec![-1 * bearing_x.signum(), 0, 0]);
    } else {
        current_order.push(vec![0, bearing_y.signum(), 0]);
        current_order.push(vec![1, 0, 0]);
        current_order.push(vec![-1, 0, 0]);
        current_order.push(vec![0, -1 * bearing_y.signum(), 0]);
    }
    let mut max_score: i32 = i32::MIN;
    for i in 0..current_order.len() {
        let new_x: i32 = (snake[0][0] as i32) + current_order[i][0];
        let new_y: i32 = (snake[0][1] as i32) + current_order[i][1];
        if new_x < 0 || new_y < 0 || (new_x as usize) >= board_size[0] || (new_y as usize) >= board_size[1] || occupied(snake, new_x as usize, new_y as usize) {
            continue
        }
        if depth < 1 || ! need_search(snake, food) {
            res = vec![current_order[i][0], current_order[i][1], 100];
            return res;
        }
        let new_snake: Vec<Vec<usize>> = move_snake_copy(snake, &current_order[i]);
        let super_mv: Vec<i32> = find_next_move_dfs(board_size, &new_snake, food, depth - 1, max_depth);
        if super_mv.len() > 0 && max_score < super_mv[2] {
            max_score = super_mv[2];
            res = vec![current_order[i][0], current_order[i][1], super_mv[2] - 1];
            if max_score > (100 - (max_depth as i32) + 4) {
                return res;
            }
        }
    }
    res
}

fn game() {
    let board_size: Vec<usize> = vec![16, 16];
    let mut snake: Vec<Vec<usize>> = Vec::new();
    snake.push(vec![3, 1]);
    snake.push(vec![2, 1]);
    snake.push(vec![1, 1]);
    let mut food: Vec<usize> = spawn_food(&board_size, &snake);
    /* This testcase may be helpful to debug suicidal behaviour after changing the search algorithm
    snake.push(vec![6,8]);
    snake.push(vec![5,8]);
    snake.push(vec![4,8]);
    snake.push(vec![3,8]);
    snake.push(vec![4,9]);
    snake.push(vec![5,9]);
    snake.push(vec![5,10]);
    snake.push(vec![6,10]);
    snake.push(vec![7,10]);
    snake.push(vec![8,10]);
    snake.push(vec![8,9]);
    snake.push(vec![8,8]);
    snake.push(vec![8,7]);
    snake.push(vec![8,6]);
    snake.push(vec![8,5]);
    snake.push(vec![8,4]);
    snake.push(vec![8,3]);
    snake.push(vec![8,2]);
    snake.push(vec![8,1]);
    snake.push(vec![7,1]);
    snake.push(vec![6,1]);
    snake.push(vec![5,1]);
    snake.push(vec![4,1]);
    snake.push(vec![5,2]);
    let mut food: Vec<usize> = vec![13, 8];*/
    print_board(&board_size, &snake, &food);
    loop {
        thread::sleep(time::Duration::from_millis(20));
        if food.len() > 0 {
            let depth: u32 = 30;
            let mv: Vec<i32> = find_next_move_dfs(&board_size, &snake, &food, depth, depth);
            if mv.len() == 0 {
                println!("We lost! Snake is {} squares long.", snake.len());
                break;
            } else if mv[0] == 0 && mv[1] == 0 {
                eat_food(&mut snake, &food);
                food = Vec::new();
            } else {
                //move_snake(&mut snake, &mv);
                move_snake(&mut snake, &mv);
            }
        } else {
            food = spawn_food(&board_size, &snake);
        }
        print_board(&board_size, &snake, &food);
    }
}


fn main() {
    game();
}
