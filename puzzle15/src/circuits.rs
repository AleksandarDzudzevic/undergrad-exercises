use crate::{GameState, Move};
use baa::*;
use patronus::expr::*;
use patronus::sim::interpreter::*;
use patronus::system::*;

/// Generates a counter from 0 to (2**width) - 1.
fn build_counter_0(ctx: &mut Context, width: WidthInt) -> TransitionSystem {
    // define a symbol to hold the count value
    let count = ctx.bv_symbol("count", width);

    // define how the count gets updated:
    // count' := count + 1
    // `ctx.build` is used here, because we are building a nested expression
    //c.one(width) here serves as a reference to a value of 1 and add just adds that to the value that is referenced by expr ref count.
    let count_next = ctx.build(|c| c.add(count, c.one(width)));

    // define the initial value of our count
    let count_init = ctx.zero(width);

    // define the transition system (the abstract circuit representation)
    let mut sys = TransitionSystem::new("counter".to_string());
    sys.add_state(
        ctx,
        State {
            symbol: count,
            init: Some(count_init),
            next: Some(count_next),
        },
    );

    // return the system
    sys
}

/// Task #1: write a function that creates a counter which has a configurable max value.
///          Once the counter reaches that max value, it should stop incrementing.
///          You can assume that the max value will fit into the use specified bit width.
///          Hint: the ctx.bv_ite primitive will choose between two values based on a condition,
///                similar to the terniary expression in e.g. C: `cond? a : b`
///                or multiplexers in hardware: https://en.wikipedia.org/wiki/Multiplexer
///          Hint: to create a constant other than 0 or 1, use:
///                `ctx.bv_lit(&BitVecValue::from_u64(value, width))`
fn build_counter_1(ctx: &mut Context, width: WidthInt, max_value: u64) -> TransitionSystem {
    // define a symbol to hold the count value
    let count = ctx.bv_symbol("count", width);

    // define how the count gets updated:
    // count' := count + 1
    // `ctx.build` is used here, because we are building a nested expression
    let count_init = ctx.zero(width);
    let count_max: ExprRef = ctx.bv_lit(&BitVecValue::from_u64(max_value, width));

    //Played around to see how different comtext methods work, this count_next also works it is
    //just more complicated for no reason

    // let count_next = ctx.build(|c| {
    //     c.bv_ite(
    //         c.bv_equal(c.sub(count_max, count), count_init),
    //         count,
    //         c.add(count, c.one(width)),
    //     )
    // });

    let count_next = ctx.build(|c| {
        c.bv_ite(
            c.bv_equal(count, count_max),
            count,
            c.add(count, c.one(width)),
        )
    });

    // define the initial value of our count
    let count_init = ctx.zero(width);
    // define the transition system (the abstract circuit representation)
    let mut sys = TransitionSystem::new("counter".to_string());
    sys.add_state(
        ctx,
        State {
            symbol: count,
            init: Some(count_init),
            next: Some(count_next),
        },
    );

    // return the system
    sys
}

/// Task #2: now the counter gets an `en` input which controls whether the counter increments or not
fn build_counter_2(ctx: &mut Context, width: WidthInt, max_value: u64) -> TransitionSystem {
    // define a symbol to hold the count value
    let count = ctx.bv_symbol("count", width);

    // NEW: define a symbol for the input
    let en = ctx.bv_symbol("en", 1);
    let count_max: ExprRef = ctx.bv_lit(&BitVecValue::from_u64(max_value, width));

    // define how the count gets updated:
    // count' := count + 1
    // `ctx.build` is used here, because we are building a nested expression
    let count_next = ctx.build(|c| {
        c.bv_ite(
            c.bv_equal(en, c.one(1)),
            c.bv_ite(
                c.bv_equal(count, count_max),
                count,
                c.add(count, c.one(width)),
            ),
            count,
        )
    });

    // define the initial value of our count
    let count_init = ctx.zero(width);

    // define the transition system (the abstract circuit representation)
    let mut sys = TransitionSystem::new("counter".to_string());
    sys.add_state(
        ctx,
        State {
            symbol: count,
            init: Some(count_init),
            next: Some(count_next),
        },
    );
    // NEW: add an input
    sys.add_input(ctx, en);

    // return the system
    sys
}

const MOVES: [Move; 4] = [
    Move::LeftToRight,
    Move::RightToLeft,
    Move::TopToBottom,
    Move::BottomToTop,
];

/// used to index into `positions` array
fn pos_to_index(x: u8, y: u8) -> usize {
    y as usize * 4 + x as usize
}

/// Task #3: implement a circuit that executes the puzzle15 game
fn build_puzzle_15(ctx: &mut Context) -> (TransitionSystem, Vec<ExprRef>, ExprRef) {
    let mut sys = TransitionSystem::new("puzzle15".to_string());

    // we encode the move as an input with 0..3 corresponding to the `[MOVES]`
    let mov = ctx.bv_symbol("move", 2);
    sys.add_input(&ctx, mov);
    let move_left_to_right = ctx.build(|c| c.bv_equal(mov, c.bit_vec_val(0, 2)));
    let move_right_to_left = ctx.build(|c| c.bv_equal(mov, c.bit_vec_val(1, 2)));
    let move_top_to_bottom = ctx.build(|c| c.bv_equal(mov, c.bit_vec_val(2, 2)));
    let move_bottom_to_top = ctx.build(|c| c.bv_equal(mov, c.bit_vec_val(3, 2)));

    // we create one state for every position, we represent the kind of tile with numbers from 0 to 15
    let mut positions = vec![];
    let mut positions_init = vec![];
    let mut is_empty = vec![];
    let init_state = GameState::default();
    for y in 0..4 {
        for x in 0..4 {
            let symbol = ctx.bv_symbol(&format!("pos_{x}_{y}"), 4);
            positions.push(symbol);
            let init_value = BitVecValue::from_u64(init_state.get(x, y).unwrap_or(0) as u64, 4);
            positions_init.push(ctx.bv_lit(&init_value));
            // condition to see if the tile is empty
            is_empty.push(ctx.build(|c| c.bv_equal(symbol, c.zero(4))));
        }
    }

    // define the next state function for every position
    let mut positions_next = vec![];
    for y in 0..4 {
        for x in 0..4 {
            let position = positions[pos_to_index(x, y)];

            // TODO: current we just assign the old tile value, how do we correctly compute the next tile value?
            let mut cond: Vec<(ExprRef, ExprRef)> = vec![];
            if x > 0 {
                cond.push((
                    ctx.and(move_left_to_right, is_empty[pos_to_index(x, y)]),
                    positions[pos_to_index(x - 1, y)],
                ));
                cond.push((
                    ctx.and(move_right_to_left, is_empty[pos_to_index(x - 1, y)]),
                    ctx.zero(4),
                ));
            }
            if x < 3 {
                cond.push((
                    ctx.and(move_right_to_left, is_empty[pos_to_index(x, y)]),
                    positions[pos_to_index(x + 1, y)],
                ));
                cond.push((
                    ctx.and(move_left_to_right, is_empty[pos_to_index(x + 1, y)]),
                    ctx.zero(4),
                ));
            }
            if y > 0 {
                cond.push((
                    ctx.and(move_top_to_bottom, is_empty[pos_to_index(x, y)]),
                    positions[pos_to_index(x, y - 1)],
                ));
                cond.push((
                    ctx.and(move_bottom_to_top, is_empty[pos_to_index(x, y - 1)]),
                    ctx.zero(4),
                ));
            }
            if y < 3 {
                cond.push((
                    ctx.and(move_bottom_to_top, is_empty[pos_to_index(x, y)]),
                    positions[pos_to_index(x, y + 1)],
                ));
                cond.push((
                    ctx.and(move_top_to_bottom, is_empty[pos_to_index(x, y + 1)]),
                    ctx.zero(4),
                ));
            }
            let position_next = ctx.build(|c| {
                let mut result = position;
                for (condition, value) in cond.iter() {
                    //Idea is to basically go through all conditions in the cond vector and if condition holds assign the new value otherwise keep the result
                    result = c.bv_ite(*condition, *value, result);
                }
                result
            });
            positions_next.push(position_next);
        }
    }

    // create states
    for (pos, (next, init)) in positions
        .iter()
        .zip(positions_next.into_iter().zip(positions_init.into_iter()))
    {
        sys.add_state(
            &ctx,
            State {
                symbol: *pos,
                next: Some(next),
                init: Some(init),
            },
        );
    }

    (sys, positions, mov)
}

#[cfg(test)]
mod tests {

    use patronus::sim;

    use super::*;

    #[test]
    fn test_counter_0() {
        let mut ctx = Context::default();
        let counter = build_counter_0(&mut ctx, 2);

        // we can turn the counter into a human-readable string representation using
        // `serialize_to_str`
        let counter_str = "\
counter
state count : bv<2>
  [init] 2'b00
  [next] add(count, 2'b01)
";
        assert_eq!(counter.serialize_to_str(&ctx), counter_str);

        // we can execute the counter with a simulator
        let mut sim = Interpreter::new(&ctx, &counter);
        // this will load the init value into count
        sim.init();

        // In order to inspect the count state, we need to retrieve the symbol that represents it.
        let count = counter
            .states()
            .find(|(_, s)| ctx.get_symbol_name(s.symbol) == Some("count"))
            .unwrap()
            .1
            .symbol;

        // Now we can read the value and see how it advances everytime we take a step
        assert_eq!(sim.get(count).unwrap().to_u64().unwrap(), 0);
        sim.step();
        assert_eq!(sim.get(count).unwrap().to_u64().unwrap(), 1);
        sim.step();
        assert_eq!(sim.get(count).unwrap().to_u64().unwrap(), 2);
        sim.step();
        assert_eq!(sim.get(count).unwrap().to_u64().unwrap(), 3);
        sim.step();
        assert_eq!(sim.get(count).unwrap().to_u64().unwrap(), 0);
    }

    #[test]
    fn test_counter_1() {
        let mut ctx = Context::default();
        let counter = build_counter_1(&mut ctx, 32, 7);
        let count = counter
            .states()
            .find(|(_, s)| ctx.get_symbol_name(s.symbol) == Some("count"))
            .unwrap()
            .1
            .symbol;
        // we print out the counter to help you debug
        println!("{}", counter.serialize_to_str(&ctx));

        let mut sim = Interpreter::new(&ctx, &counter);
        sim.init();

        for ii in 0..16 {
            if ii <= 7 {
                assert_eq!(sim.get(count).unwrap().to_u64().unwrap(), ii);
            } else {
                assert_eq!(sim.get(count).unwrap().to_u64().unwrap(), 7);
            }
            sim.step();
        }
    }

    #[test]
    fn test_counter_2() {
        let mut ctx = Context::default();
        let max_value = 123;
        let counter = build_counter_2(&mut ctx, 32, max_value);
        let count = counter
            .states()
            .find(|(_, s)| ctx.get_symbol_name(s.symbol) == Some("count"))
            .unwrap()
            .1
            .symbol;
        let en = counter.get_signals(|i| i.is_input())[0].0;
        // we print out the counter to help you debug
        println!("{}", counter.serialize_to_str(&ctx));

        let mut sim = Interpreter::new(&ctx, &counter);
        sim.init();
        let mut reference_count = 0;

        for ii in 0..20000 {
            assert_eq!(sim.get(count).unwrap().to_u64().unwrap(), reference_count);
            let enable = ii % 2 == 0;
            sim.set(en, &enable.into());
            sim.step();
            if reference_count < max_value {
                reference_count += enable as u64;
            }
        }
    }

    fn move_to_code(m: Move) -> u64 {
        match m {
            Move::LeftToRight => 0,
            Move::RightToLeft => 1,
            Move::TopToBottom => 2,
            Move::BottomToTop => 3,
        }
    }
    impl GameState {
        /// Creates a GameState from a circuit's simulation state.
        pub fn from_circuit(ctx: &Context, positions: &[ExprRef], simulator: &Interpreter) -> Self {
            let mut board = [[None; 4]; 4];
            for x in 0..4 {
                // Iterate columns first to match circuit's column-major
                for y in 0..4 {
                    // Assign to row-major GameState format
                    let pos_index = y * 4 + x; // Use column-major indexing
                    let value = simulator
                        .get(positions[pos_index])
                        .unwrap()
                        .to_u64()
                        .unwrap();
                    board[x][y] = if value == 0 { None } else { Some(value as u8) };
                }
            }
            GameState { board }
        }
    }

    #[test]
    fn test_puzzle15() {
        let mut ctx = Context::default();
        let (sys, positions, mov) = build_puzzle_15(&mut ctx);
        // we print out the puzzle to help you debug
        println!("{}", sys.serialize_to_str(&ctx));
        let mut simulator = Interpreter::new(&ctx, &sys);
        simulator.init(); //loads values initially assigned

        let circuit_game_state = GameState::from_circuit(&ctx, &positions, &simulator);
        let default_game_state = GameState::default();
        print!("{}\n", circuit_game_state);
        print!("{}\n", default_game_state);
        assert_eq!(circuit_game_state, default_game_state);

        let move_seq = [Move::TopToBottom, Move::LeftToRight, Move::LeftToRight];
        for m in move_seq {
            println!("doing move: {:?}", m);
            let symbolic_move = BitVecValue::from_u64(move_to_code(m), 2);
            simulator.set(mov, &symbolic_move);
            simulator.step()
        }
        let expected_state = GameState {
            board: [
                [Some(1), Some(5), Some(9), Some(13)],
                [Some(2), Some(6), None, Some(14)],
                [Some(3), Some(7), Some(10), Some(15)],
                [Some(4), Some(8), Some(11), Some(12)],
            ],
        };

        let circuit_game_state = GameState::from_circuit(&ctx, &positions, &simulator);
        println!("Final Circuit State:\n{}", circuit_game_state);
        println!("Expected State:\n{}", expected_state);
        assert_eq!(circuit_game_state, expected_state);
    }
}
