#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{assign, index_assign, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize, Copy)]
pub enum State {
    game,
    Player1wins,
    Player2wins,
    draw,
}

impl Default for State {
    fn default() -> Self {
        State::game
    }
}

#[account]
#[derive(Debug)]
pub struct Game {
    pub players: [Pubkey; 2],
    pub moves: [u8; 9],
    pub game_status: u8,
    pub next_move: u8,
}

impl<'info, 'entrypoint> Game {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedGame<'info, 'entrypoint>> {
        let players = Mutable::new(account.players.clone());
        let moves = Mutable::new(account.moves.clone());
        let game_status = account.game_status;
        let next_move = account.next_move;

        Mutable::new(LoadedGame {
            __account__: account,
            __programs__: programs_map,
            players,
            moves,
            game_status,
            next_move,
        })
    }

    pub fn store(loaded: Mutable<LoadedGame>) {
        let mut loaded = loaded.borrow_mut();
        let players = loaded.players.borrow().clone();

        loaded.__account__.players = players;

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
    pub players: Mutable<[Pubkey; 2]>,
    pub moves: Mutable<[u8; 9]>,
    pub game_status: u8,
    pub next_move: u8,
}

pub fn win_check(mut moves: Mutable<[u8; 9]>) -> State {
    if (((((((((moves.borrow()[moves.wrapped_index(0)] == 1)
        && (moves.borrow()[moves.wrapped_index(1)] == 1))
        && (moves.borrow()[moves.wrapped_index(2)] == 1))
        || (((moves.borrow()[moves.wrapped_index(0)] == 1)
            && (moves.borrow()[moves.wrapped_index(3)] == 1))
            && (moves.borrow()[moves.wrapped_index(6)] == 1)))
        || (((moves.borrow()[moves.wrapped_index(6)] == 1)
            && (moves.borrow()[moves.wrapped_index(7)] == 1))
            && (moves.borrow()[moves.wrapped_index(8)] == 1)))
        || (((moves.borrow()[moves.wrapped_index(2)] == 1)
            && (moves.borrow()[moves.wrapped_index(5)] == 1))
            && (moves.borrow()[moves.wrapped_index(8)] == 1)))
        || (((moves.borrow()[moves.wrapped_index(0)] == 1)
            && (moves.borrow()[moves.wrapped_index(4)] == 1))
            && (moves.borrow()[moves.wrapped_index(8)] == 1)))
        || (((moves.borrow()[moves.wrapped_index(2)] == 1)
            && (moves.borrow()[moves.wrapped_index(4)] == 1))
            && (moves.borrow()[moves.wrapped_index(6)] == 1)))
        || (((moves.borrow()[moves.wrapped_index(1)] == 1)
            && (moves.borrow()[moves.wrapped_index(4)] == 1))
            && (moves.borrow()[moves.wrapped_index(7)] == 1)))
        || (((moves.borrow()[moves.wrapped_index(3)] == 1)
            && (moves.borrow()[moves.wrapped_index(4)] == 1))
            && (moves.borrow()[moves.wrapped_index(5)] == 1))
    {
        return State::Player1wins;
    }

    if (((((((((moves.borrow()[moves.wrapped_index(0)] == 2)
        && (moves.borrow()[moves.wrapped_index(1)] == 2))
        && (moves.borrow()[moves.wrapped_index(2)] == 2))
        || (((moves.borrow()[moves.wrapped_index(0)] == 2)
            && (moves.borrow()[moves.wrapped_index(3)] == 2))
            && (moves.borrow()[moves.wrapped_index(6)] == 2)))
        || (((moves.borrow()[moves.wrapped_index(6)] == 2)
            && (moves.borrow()[moves.wrapped_index(7)] == 2))
            && (moves.borrow()[moves.wrapped_index(8)] == 2)))
        || (((moves.borrow()[moves.wrapped_index(2)] == 2)
            && (moves.borrow()[moves.wrapped_index(5)] == 2))
            && (moves.borrow()[moves.wrapped_index(8)] == 2)))
        || (((moves.borrow()[moves.wrapped_index(0)] == 2)
            && (moves.borrow()[moves.wrapped_index(4)] == 2))
            && (moves.borrow()[moves.wrapped_index(8)] == 2)))
        || (((moves.borrow()[moves.wrapped_index(2)] == 2)
            && (moves.borrow()[moves.wrapped_index(4)] == 2))
            && (moves.borrow()[moves.wrapped_index(6)] == 2)))
        || (((moves.borrow()[moves.wrapped_index(1)] == 2)
            && (moves.borrow()[moves.wrapped_index(4)] == 2))
            && (moves.borrow()[moves.wrapped_index(7)] == 2)))
        || (((moves.borrow()[moves.wrapped_index(3)] == 2)
            && (moves.borrow()[moves.wrapped_index(4)] == 2))
            && (moves.borrow()[moves.wrapped_index(5)] == 2))
    {
        return State::Player2wins;
    }

    if ((((((((moves.borrow()[moves.wrapped_index(0)] > 0)
        && (moves.borrow()[moves.wrapped_index(1)] > 0))
        && (moves.borrow()[moves.wrapped_index(2)] > 0))
        && (moves.borrow()[moves.wrapped_index(3)] > 0))
        && (moves.borrow()[moves.wrapped_index(4)] > 0))
        && (moves.borrow()[moves.wrapped_index(5)] > 0))
        && (moves.borrow()[moves.wrapped_index(6)] > 0))
        && (moves.borrow()[moves.wrapped_index(7)] > 0))
        && (moves.borrow()[moves.wrapped_index(8)] > 0)
    {
        return State::draw;
    } else {
        return State::game;
    }
}

pub fn init_game_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut player1: Pubkey,
    mut player2: Pubkey,
    mut game: Empty<Mutable<LoadedGame<'info, '_>>>,
) -> () {
    let mut game = game.account.clone();

    index_assign!(
        game.borrow().players.borrow_mut(),
        game.borrow().players.wrapped_index(0),
        player1
    );

    index_assign!(
        game.borrow().players.borrow_mut(),
        game.borrow().players.wrapped_index(1),
        player2
    );

    assign!(game.borrow_mut().game_status, 0);

    assign!(game.borrow_mut().next_move, 1);
}

pub fn play_game_handler<'info>(
    mut player: SeahorseSigner<'info, '_>,
    mut game_data: Mutable<LoadedGame<'info, '_>>,
    mut played_by: u8,
    mut move_position: u8,
) -> () {
    if !(game_data.borrow().players.borrow()[game_data
        .borrow()
        .players
        .wrapped_index(((played_by - 1) as i128))]
        == player.key())
    {
        panic!("Invalid Signer");
    }

    if !(played_by == game_data.borrow().next_move) {
        panic!("Invalid Player");
    }

    if !(game_data.borrow().game_status == 0) {
        panic!("Invalid Instruction");
    }

    if !(game_data.borrow().moves.borrow()[game_data
        .borrow()
        .moves
        .wrapped_index(((move_position - 1) as i128))]
        == 0)
    {
        panic!("Invalid move position");
    }

    let mut move_position = move_position - 1;

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

    let mut game_status = win_check(Mutable::new(
        <_ as TryInto<[_; 9]>>::try_into(
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

    if game_status == State::game {
        solana_program::msg!("{}", game_data.borrow().next_move);
    } else {
        if game_status == State::Player1wins {
            assign!(game_data.borrow_mut().game_status, 1);

            solana_program::msg!("{}", "player1 wins the Game");
        } else {
            if game_status == State::Player2wins {
                assign!(game_data.borrow_mut().game_status, 2);

                solana_program::msg!("{}", "player2 wins the Game");
            } else {
                if game_status == State::draw {
                    assign!(game_data.borrow_mut().game_status, 3);

                    solana_program::msg!("{}", "Game Draw");
                } else {
                    solana_program::msg!("{}", "Game Error");
                }
            }
        }
    }
}
