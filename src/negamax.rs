use crate::position::Position;
use crate::position::HEIGHT;
use crate::position::WIDTH;

fn negamax(p: &Position, mut alpha: i32, mut beta: i32) -> i32 {
    // If there are no more moves left, return 0.
    if p.nb_moves() == (WIDTH * HEIGHT).try_into().unwrap() {
        return 0;
    }

    // For each playable move, return the score of said move.
    for i in 0..WIDTH as u32 {
        if p.can_play(i) && p.is_winning_move(i) {
            return (-((HEIGHT * (WIDTH + 1)) as i32) - p.nb_moves() as i32) / 2;
        }
    }

    let max: i32 = ((WIDTH * (HEIGHT - 1)) as i32 - p.nb_moves() as i32) / 2;

    // let mut best_score = -(WIDTH as i32) * HEIGHT as i32;

    if beta > max {
        beta = max;
        if alpha >= beta {
            return beta;
        }
    }

    // Checks player 1 vs player 2's potential moves and returns the best result.
    for i in 0..WIDTH as u32 {
        if p.can_play(i) {
            let p2 = p.clone();
            p2.play(i);
            let score = -negamax(p2, -beta, -alpha);

            // if score >= beta {
            //     return score;
            // }
            // if score > alpha {
            //     alpha = score;
            // }

            match score {
                score if score >= beta => return score,
                score if score > alpha => alpha = score,
                _ => {}
            }
        }
    }

    alpha
}
