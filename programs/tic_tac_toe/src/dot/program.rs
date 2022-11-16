#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{assign, index_assign, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct Game {
    pub player1: Pubkey,
    pub player2: Pubkey,
    pub moves: [u8; 10],
    pub game_status: u8,
    pub next_move: u8,
}

impl<'info, 'entrypoint> Game {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedGame<'info, 'entrypoint>> {
        let player1 = account.player1.clone();
        let player2 = account.player2.clone();
        let moves = Mutable::new(account.moves.clone());
        let game_status = account.game_status;
        let next_move = account.next_move;

        Mutable::new(LoadedGame {
            __account__: account,
            __programs__: programs_map,
            player1,
            player2,
            moves,
            game_status,
            next_move,
        })
    }

    pub fn store(loaded: Mutable<LoadedGame>) {
        let mut loaded = loaded.borrow_mut();
        let player1 = loaded.player1.clone();

        loaded.__account__.player1 = player1;

        let player2 = loaded.player2.clone();

        loaded.__account__.player2 = player2;

        let moves = loaded.moves.borrow().clone();

        loaded.__account__.moves = moves;

        let game_status = loaded.game_status;

        loaded.__account__.game_status = game_status;

        let next_move = loaded.next_move;

        loaded.__account__.next_move = next_move;
    }
}

#[derive(Debug)]
pub struct LoadedGame<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Game>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub player1: Pubkey,
    pub player2: Pubkey,
    pub moves: Mutable<[u8; 10]>,
    pub game_status: u8,
    pub next_move: u8,
}

pub fn play_game_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut game_data: Mutable<LoadedGame<'info, '_>>,
    mut played_by: u8,
    mut move_position: u8,
) -> () {
    if game_data.borrow().game_status == 0 {
        if (game_data.borrow().moves.borrow()[game_data
            .borrow()
            .moves
            .wrapped_index((move_position as i128))]
            == 0)
            && (game_data.borrow().next_move == played_by)
        {
            if game_data.borrow().next_move == 1 {
                index_assign!(
                    game_data.borrow().moves.borrow_mut(),
                    game_data
                        .borrow()
                        .moves
                        .wrapped_index((move_position as i128)),
                    1
                );

                assign!(game_data.borrow_mut().next_move, 2);
            } else {
                if game_data.borrow().next_move == 2 {
                    index_assign!(
                        game_data.borrow().moves.borrow_mut(),
                        game_data
                            .borrow()
                            .moves
                            .wrapped_index((move_position as i128)),
                        2
                    );

                    assign!(game_data.borrow_mut().next_move, 1);
                }
            }
        } else {
            solana_program::msg!("{}", "wrong move");
        }

        let mut game_status = win_check(Mutable::new(
            <_ as TryInto<[_; 10]>>::try_into(
                (game_data
                    .borrow()
                    .moves
                    .borrow()
                    .iter()
                    .map(|element| element.clone()))
                .collect::<Vec<_>>(),
            )
            .unwrap(),
        ));

        if game_status == 0 {
            solana_program::msg!("{}", game_data.borrow().next_move);
        } else {
            if game_status == 1 {
                assign!(game_data.borrow_mut().game_status, 1);

                solana_program::msg!("{}", "player1 wins the Game");
            } else {
                if game_status == 2 {
                    assign!(game_data.borrow_mut().game_status, 2);

                    solana_program::msg!("{}", "player2 win the game");
                } else {
                    if game_status == 3 {
                        assign!(game_data.borrow_mut().game_status, 3);

                        solana_program::msg!("{}", "The Game was draw");
                    } else {
                        solana_program::msg!("{}", "Error Game");
                    }
                }
            }
        }
    } else {
        solana_program::msg!("{}", "Invalid Instruction");
    }
}

pub fn win_check(mut moves: Mutable<[u8; 10]>) -> i8 {
    if (((((((((moves.borrow()[moves.wrapped_index(1)] == 1)
        && (moves.borrow()[moves.wrapped_index(2)] == 1))
        && (moves.borrow()[moves.wrapped_index(3)] == 1))
        || (((moves.borrow()[moves.wrapped_index(1)] == 1)
            && (moves.borrow()[moves.wrapped_index(4)] == 1))
            && (moves.borrow()[moves.wrapped_index(7)] == 1)))
        || (((moves.borrow()[moves.wrapped_index(7)] == 1)
            && (moves.borrow()[moves.wrapped_index(8)] == 1))
            && (moves.borrow()[moves.wrapped_index(9)] == 1)))
        || (((moves.borrow()[moves.wrapped_index(3)] == 1)
            && (moves.borrow()[moves.wrapped_index(6)] == 1))
            && (moves.borrow()[moves.wrapped_index(9)] == 1)))
        || (((moves.borrow()[moves.wrapped_index(1)] == 1)
            && (moves.borrow()[moves.wrapped_index(5)] == 1))
            && (moves.borrow()[moves.wrapped_index(9)] == 1)))
        || (((moves.borrow()[moves.wrapped_index(3)] == 1)
            && (moves.borrow()[moves.wrapped_index(5)] == 1))
            && (moves.borrow()[moves.wrapped_index(7)] == 1)))
        || (((moves.borrow()[moves.wrapped_index(2)] == 1)
            && (moves.borrow()[moves.wrapped_index(5)] == 1))
            && (moves.borrow()[moves.wrapped_index(8)] == 1)))
        || (((moves.borrow()[moves.wrapped_index(4)] == 1)
            && (moves.borrow()[moves.wrapped_index(5)] == 1))
            && (moves.borrow()[moves.wrapped_index(6)] == 1))
    {
        return 1;
    } else {
        if (((((((((moves.borrow()[moves.wrapped_index(1)] == 1)
            && (moves.borrow()[moves.wrapped_index(2)] == 1))
            && (moves.borrow()[moves.wrapped_index(3)] == 1))
            || (((moves.borrow()[moves.wrapped_index(1)] == 1)
                && (moves.borrow()[moves.wrapped_index(4)] == 1))
                && (moves.borrow()[moves.wrapped_index(7)] == 1)))
            || (((moves.borrow()[moves.wrapped_index(7)] == 1)
                && (moves.borrow()[moves.wrapped_index(8)] == 1))
                && (moves.borrow()[moves.wrapped_index(9)] == 1)))
            || (((moves.borrow()[moves.wrapped_index(3)] == 1)
                && (moves.borrow()[moves.wrapped_index(6)] == 1))
                && (moves.borrow()[moves.wrapped_index(9)] == 1)))
            || (((moves.borrow()[moves.wrapped_index(1)] == 1)
                && (moves.borrow()[moves.wrapped_index(5)] == 1))
                && (moves.borrow()[moves.wrapped_index(9)] == 1)))
            || (((moves.borrow()[moves.wrapped_index(3)] == 1)
                && (moves.borrow()[moves.wrapped_index(5)] == 1))
                && (moves.borrow()[moves.wrapped_index(7)] == 1)))
            || (((moves.borrow()[moves.wrapped_index(2)] == 1)
                && (moves.borrow()[moves.wrapped_index(5)] == 1))
                && (moves.borrow()[moves.wrapped_index(8)] == 1)))
            || (((moves.borrow()[moves.wrapped_index(4)] == 1)
                && (moves.borrow()[moves.wrapped_index(5)] == 1))
                && (moves.borrow()[moves.wrapped_index(6)] == 1))
        {
            return 2;
        } else {
            if (((((((((moves.borrow()[moves.wrapped_index(1)] == 1)
                || (moves.borrow()[moves.wrapped_index(1)] == 2))
                && ((moves.borrow()[moves.wrapped_index(2)] == 1)
                    || (moves.borrow()[moves.wrapped_index(2)] == 2)))
                && ((moves.borrow()[moves.wrapped_index(3)] == 1)
                    || (moves.borrow()[moves.wrapped_index(3)] == 2)))
                && ((moves.borrow()[moves.wrapped_index(4)] == 1)
                    || (moves.borrow()[moves.wrapped_index(4)] == 2)))
                && ((moves.borrow()[moves.wrapped_index(5)] == 1)
                    || (moves.borrow()[moves.wrapped_index(5)] == 2)))
                && ((moves.borrow()[moves.wrapped_index(6)] == 1)
                    || (moves.borrow()[moves.wrapped_index(6)] == 2)))
                && ((moves.borrow()[moves.wrapped_index(7)] == 1)
                    || (moves.borrow()[moves.wrapped_index(7)] == 2)))
                && ((moves.borrow()[moves.wrapped_index(2)] == 1)
                    || (moves.borrow()[moves.wrapped_index(8)] == 2)))
                && ((moves.borrow()[moves.wrapped_index(9)] == 1)
                    || (moves.borrow()[moves.wrapped_index(9)] == 2))
            {
                return 3;
            } else {
                return 0;
            }
        }
    }
}

pub fn init_game_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut player1: Pubkey,
    mut player2: Pubkey,
    mut game: Empty<Mutable<LoadedGame<'info, '_>>>,
) -> () {
    let mut game = game.account.clone();

    assign!(game.borrow_mut().player1, player1);

    assign!(game.borrow_mut().player2, player2);

    assign!(game.borrow_mut().game_status, 0);

    assign!(game.borrow_mut().next_move, 1);
}
