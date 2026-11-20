//! Versioned station saves are replay journals over an identified scenario.
use super::*;
use serde::{Deserialize, Serialize};

const SAVE_VERSION: u32 = 1;
const SCENARIO: &str = "six-week-station";
const MAX_BYTES: usize = 8 * 1024 * 1024;
const MAX_COMMANDS: usize = 20_000;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StationSave {
    version: u32,
    scenario: String,
    commands: Vec<Command>,
}

#[derive(Serialize)]
struct StationSaveRef<'a> {
    version: u32,
    scenario: &'static str,
    commands: &'a [Command],
}

impl Session {
    pub fn save_game(&self) -> Result<String, PlayerError> {
        let save = serde_json::to_string(&StationSaveRef {
            version: SAVE_VERSION,
            scenario: SCENARIO,
            commands: &self.journal,
        })
        .map_err(|_| PlayerError::new("save_encode", "Unable to encode this station's history."))?;
        if save.len() > MAX_BYTES || self.journal.len() > MAX_COMMANDS {
            return Err(PlayerError::new(
                "save_limit",
                "The station history exceeds the supported save size.",
            ));
        }
        Ok(save)
    }

    pub fn restore_game(&mut self, json: &str) -> Result<PlayerReport, PlayerError> {
        if json.len() > MAX_BYTES {
            return Err(PlayerError::new(
                "save_limit",
                "The station save exceeds the supported size.",
            ));
        }
        let save: StationSave = serde_json::from_str(json)
            .map_err(|_| PlayerError::new("save_format", "Choose a valid station save file."))?;
        if save.version != SAVE_VERSION || save.scenario != SCENARIO {
            return Err(PlayerError::new(
                "save_version",
                "This station save uses a different version or scenario.",
            ));
        }
        if save.commands.len() > MAX_COMMANDS {
            return Err(PlayerError::new(
                "save_limit",
                "The station save contains too many actions.",
            ));
        }
        let mut restored = Self::new()?;
        for mut command in save.commands.iter().cloned() {
            restored.reissue_save_token(&mut command)?;
            restored.apply(command).map_err(|_| {
                PlayerError::new(
                    "save_replay",
                    "The saved station history could not be replayed.",
                )
            })?;
        }
        restored.journal = std::sync::Arc::new(
            save.commands
                .into_iter()
                .map(without_session_token)
                .collect(),
        );
        let report = restored.report();
        *self = restored;
        Ok(report)
    }

    fn reissue_save_token(&self, command: &mut Command) -> Result<(), PlayerError> {
        let missing = || PlayerError::new("save_replay", "A saved deal has no matching offer.");
        match command {
            Command::AcceptOffer {
                airing_id,
                offer_id,
            } => {
                *offer_id = self
                    .pending
                    .as_ref()
                    .filter(|pending| pending.quote.airing_id.0 == *airing_id)
                    .ok_or_else(missing)?
                    .token
                    .clone();
            }
            Command::AcceptMinuteOffer {
                airing_id, token, ..
            } => {
                *token = self
                    .minute_pending
                    .as_ref()
                    .filter(|(id, _, _)| id.0 == *airing_id)
                    .ok_or_else(missing)?
                    .1
                    .clone();
            }
            Command::BuyRights { programme, token } => {
                *token = self
                    .rights_market()
                    .titles
                    .into_iter()
                    .find(|title| title.programme == *programme)
                    .and_then(|title| title.acquisition)
                    .ok_or_else(missing)?
                    .token;
            }
            Command::AcceptResale { programme, token } => {
                *token = self
                    .rights_market()
                    .titles
                    .into_iter()
                    .find(|title| title.programme == *programme)
                    .and_then(|title| title.resale)
                    .ok_or_else(missing)?
                    .token;
            }
            _ => {}
        }
        Ok(())
    }
}

pub(super) fn without_session_token(mut command: Command) -> Command {
    match &mut command {
        Command::AcceptOffer { offer_id, .. } => offer_id.clear(),
        Command::AcceptMinuteOffer { token, .. }
        | Command::BuyRights { token, .. }
        | Command::AcceptResale { token, .. } => token.clear(),
        _ => {}
    }
    command
}
