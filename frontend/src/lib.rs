#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod downloadable;
mod games;
mod login_register;
mod manipulator;
mod signed_up_teams;
mod teams;
mod teams_playing_players;
mod tournament_applications;
mod tournaments;
mod tournaments_playing_players;
mod user_edit;
mod user_invites;
mod users;
mod pan_zoom;
mod show_brackets;
pub use app::FrontendApp;
