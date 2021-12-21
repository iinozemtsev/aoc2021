#![allow(dead_code)]

use std::collections::HashMap;

struct DeterministicDice {
    _current: u64,
    _roll_count: u64,
}

trait Dice {
    fn next(&mut self) -> u64;
    fn roll_count(&self) -> u64;
}
impl DeterministicDice {
    fn new() -> Self {
        DeterministicDice {
            _current: 0,
            _roll_count: 0,
        }
    }
}

impl Dice for DeterministicDice {
    fn next(&mut self) -> u64 {
        self._current = (self._current + 1) % 100;
        self._roll_count += 1;
        return self._current;
    }

    fn roll_count(&self) -> u64 {
        self._roll_count
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct Pawn {
    // Position is storred off-by-one to use mod.
    position: u8,
    score: u64,
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct DiracDice {}

impl DiracDice {
    fn roll(&self, universes: &HashMap<Universe, u128>) -> HashMap<Universe, u128> {
        let mut result: HashMap<Universe, u128> = HashMap::new();
        for (universe, count) in universes {
            if universe.game_over {
                // Pass universes as is.
                result
                    .entry(universe.clone())
                    .and_modify(|c| *c += *count)
                    .or_insert(*count);
            } else {
                for outcome in 1..=3 {
                    if let Some(new_universe) = universe.roll(outcome) {
                        // Add new universes.
                        result
                            .entry(new_universe)
                            .and_modify(|c| *c += *count)
                            .or_insert(*count);
                    }
                }
            }
        }
        result
    }
}

fn part2(p1: u8, p2: u8) -> u128 {
    // Start with a one universe.
    let mut universes = vec![(Universe::new(&vec![p1, p2]), 1)]
        .into_iter()
        .collect::<HashMap<Universe, u128>>();

    let dice = DiracDice {};
    while universes.keys().any(|u| !u.game_over) {
        universes = dice.roll(&universes);
    }
    let mut wins_by_player: HashMap<u8, u128> = HashMap::new();
    for (u, count) in universes {
        wins_by_player
            .entry(u.winner_index)
            .and_modify(|c| *c += count)
            .or_insert(count);
    }
    *wins_by_player.values().max().unwrap()
}
#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct UniverseState {
    // Who's rolling the dice next.
    player_index: u8,
    // What they rolled so far.
    player_rolls: Vec<u8>,
}

impl UniverseState {
    fn new() -> Self {
        UniverseState {
            player_index: 0,
            player_rolls: Vec::new(),
        }
    }
}
#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct Universe {
    players: Vec<Pawn>,
    dice: DiracDice,
    state: UniverseState,
    game_over: bool,
    winner_index: u8,
}

impl Universe {
    fn new(positions: &Vec<u8>) -> Self {
        Universe {
            players: positions.iter().map(|&v| Pawn::new(v)).collect(),
            dice: DiracDice {},
            state: UniverseState::new(),
            game_over: false,
            winner_index: 0,
        }
    }

    fn roll(&self, score: u8) -> Option<Self> {
        // Don't roll if game is over.
        if self.game_over {
            return None;
        }
        let mut copy = self.clone();
        copy.state.player_rolls.push(score);
        if copy.state.player_rolls.len() == 3 {
            // series is over.
            let player: &mut Pawn = copy
                .players
                .get_mut(copy.state.player_index as usize)
                .unwrap();
            // Calculate the new position.
            let sum: u8 = copy.state.player_rolls.iter().sum();
            player.position = (player.position + sum) % 10;
            player.score += (player.position + 1) as u64;
            if player.score >= 21 {
                copy.game_over = true;
                copy.winner_index = copy.state.player_index;
            }
            copy.state.player_rolls.clear();
            copy.state.player_index = (copy.state.player_index + 1) % (copy.players.len() as u8);
        }
        Some(copy)
    }
}
impl Pawn {
    fn new(position: u8) -> Self {
        Pawn {
            position: position - 1,
            score: 0,
        }
    }

    fn turn(&mut self, dice: &mut dyn Dice) {
        let offset: u64 = (0..3).map(|_| dice.next()).sum();
        self.position = ((self.position as u64 + offset) % 10) as u8;
        self.score += (self.position + 1) as u64;
    }
}

fn part1(p1pos: u8, p2pos: u8) -> u64 {
    let mut p1 = Pawn::new(p1pos);
    let mut p2 = Pawn::new(p2pos);
    let mut dice = DeterministicDice::new();
    loop {
        println!("p1: {}, p2: {}", p1.score, p2.score);
        p1.turn(&mut dice);
        if p1.score >= 1000 {
            break;
        }

        p2.turn(&mut dice);
        if p2.score >= 1000 {
            break;
        }
    }

    let loser = if p1.score > p2.score { p2 } else { p1 };
    loser.score * dice.roll_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let practice = super::part1(4, 8);
        assert_eq!(practice, 739785);
        println!("part1: {}", super::part1(8, 9));
    }

    #[test]
    fn part2() {
        let practice = super::part2(4, 8);
        assert_eq!(practice, 444356092776315);
        println!("part2: {}", super::part2(8, 9));
    }
}
