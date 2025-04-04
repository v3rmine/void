use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[clap(verbatim_doc_comment)]
/// Manage authentication
/// Authenticate with Dedale (and logout if you need to). If you do not have an account, start with the AUTH SIGNUP command.
/// If you do have an account, begin with the AUTH LOGIN subcommand.
pub enum Command {
    /// Log in a user
    Login,
    /// Logs out the currently logged in user
    Logout,
    /// Create a new dedale account
    Signup,
    /// Show the current auth token
    Token,
    /// Displays the users email address/service identity currently authenticated and in use.
    Whoami
}