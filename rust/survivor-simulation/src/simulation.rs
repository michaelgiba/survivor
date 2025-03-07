use std::collections::HashMap;

use crate::llm::MockLLMClient;
use crate::llm::LLM;
use anyhow::Result;
use rand::Rng;

type RoundNumber = i32;
type PlayerNumber = i32;

pub struct Message {
    pub sender_player_number: PlayerNumber,
    pub receiver_player_number: PlayerNumber,
    pub text: String,
}

// -------- Players and Game State --------

pub struct SurvivorContestant {
    player_number: PlayerNumber,
    context: Vec<String>,
}

enum ContestantState {
    Active,
    Eliminated,
}

#[derive(Debug, Clone, Copy)]
pub struct Vote {
    from_player: PlayerNumber,
    vote_for: PlayerNumber,
}

#[derive(Debug, Clone)]
pub struct RoundResult {
    eliminated_player: PlayerNumber,
    vote_counts: Vec<(PlayerNumber, i32)>,
}

pub struct GameState {
    contestants: Vec<(SurvivorContestant, ContestantState)>,
}

impl GameState {
    pub fn new(num_players: i32) -> Result<Self> {
        if (num_players < 3) {
            anyhow::bail!("Number of players must be at least 3");
        }

        let contestants = (0..num_players)
            .map(|i| {
                (
                    SurvivorContestant {
                        player_number: i,
                        context: Vec::new(),
                    },
                    ContestantState::Active,
                )
            })
            .collect();

        Ok(Self { contestants })
    }

    pub fn active_players(&self) -> Vec<PlayerNumber> {
        self.contestants
            .iter()
            .filter_map(|(contestant, state)| match state {
                ContestantState::Active => Some(contestant.player_number),
                ContestantState::Eliminated => None,
            })
            .collect()
    }

    pub fn is_game_over(&self) -> bool {
        self.active_players().len() <= 2
    }

    pub fn add_message_to_player_context(
        &mut self,
        player: PlayerNumber,
        message: String,
    ) -> Result<()> {
        if let Some((contestant, _)) = self
            .contestants
            .iter_mut()
            .find(|(c, _)| c.player_number == player)
        {
            contestant.context.push(message);
            Ok(())
        } else {
            anyhow::bail!("Player not found")
        }
    }

    pub fn eliminate_player(&mut self, player: PlayerNumber) -> Result<()> {
        if let Some((_, state)) = self
            .contestants
            .iter_mut()
            .find(|(c, _)| c.player_number == player)
        {
            *state = ContestantState::Eliminated;
            Ok(())
        } else {
            anyhow::bail!("Player not found")
        }
    }

    pub fn get_player_context(&self, player: PlayerNumber) -> Result<&Vec<String>> {
        self.contestants
            .iter()
            .find_map(|(contestant, _)| {
                if contestant.player_number == player {
                    Some(&contestant.context)
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow::anyhow!("Player not found"))
    }
}

// -------- State Structs --------

// Base trait for all states
pub trait SimulationState {
    fn execute(
        &self,
        game_state: &mut GameState,
        llm_client: &mut dyn LLM,
    ) -> Result<Box<dyn SimulationState>>;
    fn name(&self) -> &'static str;
}

// Private Chat Phase
pub struct PrivateChatState {
    round: RoundNumber,
}

impl PrivateChatState {
    fn new(round: RoundNumber) -> Self {
        Self { round }
    }
}

impl SimulationState for PrivateChatState {
    fn name(&self) -> &'static str {
        "Private Chat"
    }

    fn execute(
        &self,
        game_state: &mut GameState,
        llm_client: &mut dyn LLM,
    ) -> Result<Box<dyn SimulationState>> {
        println!("\n--- Round {} Private Chat Phase ---", self.round);

        let active = game_state.active_players();

        for &sender in &active {
            let mut messages_sent = 0;

            for &receiver in &active {
                if sender != receiver && messages_sent < 3 {
                    let context = game_state.get_player_context(sender)?;
                    let prompt = format!(
                        "You are Player {} in a Survivor game. Based on your context, write a private message to Player {}.",
                        sender, receiver
                    );

                    if let Ok(message_text) = llm_client.generate(&prompt, context) {
                        // Add message to both players' context
                        let formatted_message =
                            format!("Player {} to Player {}: {}", sender, receiver, message_text);

                        game_state
                            .add_message_to_player_context(sender, formatted_message.clone())?;
                        game_state.add_message_to_player_context(receiver, formatted_message)?;

                        messages_sent += 1;
                    }
                }
            }
        }

        // Move to next state
        Ok(Box::new(PublicStatementState::new(self.round)))
    }
}

// Public Statement Phase
pub struct PublicStatementState {
    round: RoundNumber,
}

impl PublicStatementState {
    fn new(round: RoundNumber) -> Self {
        Self { round }
    }
}

impl SimulationState for PublicStatementState {
    fn name(&self) -> &'static str {
        "Public Statement"
    }

    fn execute(
        &self,
        game_state: &mut GameState,
        llm_client: &mut dyn LLM,
    ) -> Result<Box<dyn SimulationState>> {
        println!("\n--- Round {} Public Statement Phase ---", self.round);

        let active = game_state.active_players();

        for &player in &active {
            let context = game_state.get_player_context(player)?;
            let prompt = format!(
                "You are Player {} in a Survivor game. Make a public statement to all players based on your context.",
                player
            );

            if let Ok(statement) = llm_client.generate(&prompt, context) {
                let formatted_statement =
                    format!("Player {} public statement: {}", player, statement);

                // Add statement to all players' context
                for &active_player in &active {
                    game_state.add_message_to_player_context(
                        active_player,
                        formatted_statement.clone(),
                    )?;
                }
            }
        }

        // Move to next state
        Ok(Box::new(VotingState::new(self.round)))
    }
}

// Voting Phase
pub struct VotingState {
    round: RoundNumber,
}

impl VotingState {
    fn new(round: RoundNumber) -> Self {
        Self { round }
    }

    fn collect_votes(
        &self,
        game_state: &mut GameState,
        llm_client: &mut dyn LLM,
    ) -> Result<std::collections::HashMap<PlayerNumber, i32>> {
        println!("\n--- Round {} Voting Phase ---", self.round);

        let active = game_state.active_players();
        let mut votes = Vec::new();

        // Each player submits a vote
        for &voter in &active {
            let context = game_state.get_player_context(voter)?;
            let prompt = format!(
                "You are Player {} in a Survivor game. Based on your context, which player would you vote to eliminate? Respond with just the player number.",
                voter
            );

            if let Ok(vote_text) = llm_client.generate(&prompt, context) {
                if let Ok(vote_for) = vote_text.trim().parse::<PlayerNumber>() {
                    if active.contains(&vote_for) && vote_for != voter {
                        let vote = Vote {
                            from_player: voter,
                            vote_for,
                        };

                        // Add vote to voter's context
                        game_state.add_message_to_player_context(
                            voter,
                            format!("Voted for Player {}", vote_for),
                        )?;

                        votes.push(vote);
                    }
                }
            }
        }

        // Tally votes
        let mut vote_counts: std::collections::HashMap<PlayerNumber, i32> =
            std::collections::HashMap::new();

        for vote in &votes {
            *vote_counts.entry(vote.vote_for).or_insert(0) += 1;
        }

        Ok(vote_counts)
    }

    fn chose_who_to_eliminate(&self, vote_counts: HashMap<i32, i32>) -> i32 {
        // Find highest vote count
        let max_votes = vote_counts.values().max().unwrap_or(&0);

        // Get all players with max votes
        let most_voted: Vec<PlayerNumber> = vote_counts
            .iter()
            .filter(|(_, &count)| count == *max_votes)
            .map(|(&player, _)| player)
            .collect();

        // Randomly choose one if there's a tie using updated rand functions
        let mut rng = rand::rng();
        let idx = rng.random_range(0..most_voted.len());
        most_voted[idx]
    }
}

impl SimulationState for VotingState {
    fn name(&self) -> &'static str {
        "Voting"
    }

    fn execute(
        &self,
        game_state: &mut GameState,
        llm_client: &mut dyn LLM,
    ) -> Result<Box<dyn SimulationState>> {
        println!("\n--- Round {} Voting Phase ---", self.round);

        let vote_counts = self.collect_votes(game_state, llm_client)?;

        let eliminated_player = self.chose_who_to_eliminate(vote_counts.clone());

        // Update player state
        game_state.eliminate_player(eliminated_player)?;

        let result = RoundResult {
            eliminated_player,
            vote_counts: vote_counts.into_iter().collect(),
        };

        println!("Player {} has been eliminated", result.eliminated_player);
        println!("Vote counts: {:?}", result.vote_counts);

        // Add elimination result to all players' context
        let elimination_message = format!(
            "Player {} was eliminated in round {}",
            eliminated_player, self.round
        );
        for &player in &game_state.active_players() {
            game_state.add_message_to_player_context(player, elimination_message.clone())?;
        }

        // Determine next state
        if game_state.active_players().len() <= 2 {
            let finalists: Vec<PlayerNumber> = game_state.active_players();
            Ok(Box::new(FinalPleaState::new(finalists[0], finalists[1])))
        } else {
            Ok(Box::new(PrivateChatState::new(self.round + 1)))
        }
    }
}

// Final Plea Phase
pub struct FinalPleaState {
    finalist1: PlayerNumber,
    finalist2: PlayerNumber,
}

impl FinalPleaState {
    fn new(finalist1: PlayerNumber, finalist2: PlayerNumber) -> Self {
        Self {
            finalist1,
            finalist2,
        }
    }
}

impl SimulationState for FinalPleaState {
    fn name(&self) -> &'static str {
        "Final Plea"
    }

    fn execute(
        &self,
        game_state: &mut GameState,
        llm_client: &mut dyn LLM,
    ) -> Result<Box<dyn SimulationState>> {
        println!("\n--- Final Round: Finalists' Plea ---");
        println!(
            "Finalists: Player {} and Player {}",
            self.finalist1, self.finalist2
        );

        // Each finalist makes their final plea
        for &finalist in &[self.finalist1, self.finalist2] {
            let context = game_state.get_player_context(finalist)?.clone();
            let prompt = format!(
                "You are Player {} in the final round of Survivor. Make your final plea to the jury.",
                finalist
            );

            if let Ok(plea) = llm_client.generate(&prompt, &context) {
                let formatted_plea = format!("Player {} final plea: {}", finalist, plea);

                // Clone the list of contestants first
                let player_numbers: Vec<_> = game_state
                    .contestants
                    .iter()
                    .map(|(c, _)| c.player_number)
                    .collect();

                // Then add plea to each player's context
                for player_number in player_numbers {
                    game_state
                        .add_message_to_player_context(player_number, formatted_plea.clone())?;
                }
            }
        }

        // Move to final vote
        Ok(Box::new(FinalVoteState::new(
            self.finalist1,
            self.finalist2,
        )))
    }
}

// Final Vote Phase
pub struct FinalVoteState {
    finalist1: PlayerNumber,
    finalist2: PlayerNumber,
}

impl FinalVoteState {
    fn new(finalist1: PlayerNumber, finalist2: PlayerNumber) -> Self {
        Self {
            finalist1,
            finalist2,
        }
    }
}

impl SimulationState for FinalVoteState {
    fn name(&self) -> &'static str {
        "Final Vote"
    }

    fn execute(
        &self,
        game_state: &mut GameState,
        _llm_client: &mut dyn LLM,
    ) -> Result<Box<dyn SimulationState>> {
        println!("\n--- Game over! ---");
        println!(
            "Finalists: Player {} and Player {}",
            self.finalist1, self.finalist2
        );

        // In a full implementation, we would have eliminated players vote for a winner

        // Simply return GameOverState
        Ok(Box::new(GameOverState {
            finalists: vec![self.finalist1, self.finalist2],
        }))
    }
}

// Game Over State
pub struct GameOverState {
    finalists: Vec<PlayerNumber>,
}

impl SimulationState for GameOverState {
    fn name(&self) -> &'static str {
        "Game Over"
    }

    fn execute(
        &self,
        _game_state: &mut GameState,
        _llm_client: &mut dyn LLM,
    ) -> Result<Box<dyn SimulationState>> {
        // This is a terminal state, we shouldn't be executing it
        anyhow::bail!("Game is already over")
    }
}
// -------- Main Simulation Runner --------

pub struct Simulation {
    game_state: GameState,
    current_state: Box<dyn SimulationState>,
}

impl Simulation {
    pub fn new(num_players: i32) -> Result<Self> {
        let game_state = GameState::new(num_players)?;
        let current_state: Box<dyn SimulationState> = Box::new(PrivateChatState::new(1));

        Ok(Self {
            game_state,
            current_state,
        })
    }

    pub fn run(&mut self, llm_client: &mut dyn LLM) -> Result<Vec<PlayerNumber>> {
        println!(
            "Starting Survivor simulation with {} players",
            self.game_state.contestants.len()
        );

        // Continue until we reach the GameOverState
        while self.current_state.name() != "Game Over" {
            // Get the next state
            self.current_state = self
                .current_state
                .execute(&mut self.game_state, llm_client)?;
        }

        // Access the finalists through active players (only the finalists are left active)
        let finalists = self.game_state.active_players();
        if finalists.len() != 2 {
            anyhow::bail!("Expected exactly 2 finalists at the end of the game");
        }

        Ok(finalists)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_creation() {
        let game = GameState::new(4).unwrap();
        assert_eq!(game.active_players().len(), 4);
        assert_eq!(game.active_players(), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_game_state_min_players() {
        assert!(GameState::new(2).is_err());
        assert!(GameState::new(3).is_ok());
    }

    #[test]
    fn test_player_elimination() {
        let mut game = GameState::new(4).unwrap();
        game.eliminate_player(1).unwrap();
        assert_eq!(game.active_players().len(), 3);
        assert_eq!(game.active_players(), vec![0, 2, 3]);

        // Try to eliminate non-existent player
        assert!(game.eliminate_player(10).is_err());
    }

    #[test]
    fn test_player_context() {
        let mut game = GameState::new(4).unwrap();
        game.add_message_to_player_context(2, "test message".to_string())
            .unwrap();
        let context = game.get_player_context(2).unwrap();
        assert_eq!(context.len(), 1);
        assert_eq!(context[0], "test message");

        // Try to get context for non-existent player
        assert!(game.get_player_context(10).is_err());
    }

    #[test]
    fn test_game_over_condition() {
        let mut game = GameState::new(4).unwrap();
        assert!(!game.is_game_over());

        game.eliminate_player(0).unwrap();
        assert!(!game.is_game_over());

        game.eliminate_player(1).unwrap();
        assert!(game.is_game_over());
    }

    #[test]
    fn test_simulation_flow() {
        let mut mock_llm = MockLLMClient::new();
        let mut sim = Simulation::new(4).unwrap();

        // Mock LLM to return simple responses
        mock_llm.add_response("0".to_string()); // For voting

        let finalists = sim.run(&mut mock_llm).unwrap();
        assert_eq!(finalists.len(), 2);
    }

    #[test]
    fn test_chose_who_to_eliminate() {
        let voting_state = VotingState::new(1);
        let mut vote_counts = HashMap::new();
        vote_counts.insert(0, 2);
        vote_counts.insert(1, 1);

        let eliminated = voting_state.chose_who_to_eliminate(vote_counts);
        assert_eq!(eliminated, 0); // Should eliminate player with most votes
    }
}
