use std::{env, io::Error};

/// Key for environment variable that determines which configuration file
/// Starship will use.
pub const ENV_KEY_STARSHIP_CONFIG: &str = "STARSHIP_CONFIG";

const ENV_KEY_STARSHIP_SHELL: &str = "STARSHIP_SHELL";
const ENV_SHELL_VALUE_ZSH: &str = "zsh";
const ENV_SHELL_VALUE_FISH: &str = "fish";

/// Representation of supported shells
pub enum Shell {
    Fish,
    Zsh,
}

impl Shell {
    /// # Returns
    ///
    /// Returns a representation of currently used shell based on
    /// the `STARSHIP_SHELL` environment variable. Returns `None` if shell
    /// is not supported or Starship has not set environment variable.
    ///
    /// # Expamples
    ///
    /// ## On `zsh`:
    /// ```
    /// assert_eq!(Shell::get_current(), Some(Zsh));
    /// ```
    ///
    /// ## On `PowerShell`:
    /// ```
    /// assert_eq!(Shell::get_current(), None);
    /// ```
    pub fn get_current() -> Option<Self> {
        let starship_shell = env::var(ENV_KEY_STARSHIP_SHELL).ok()?;

        match starship_shell.as_str() {
            ENV_SHELL_VALUE_FISH => Some(Self::Fish),
            ENV_SHELL_VALUE_ZSH => Some(Self::Zsh),
            _ => None,
        }
    }

    /// Returns (trimmed) value for given environment variable name.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the environment variable (e.g. `"HOME"`)
    ///
    /// # Returns
    ///
    /// The trimmed value of the environment variable, if set.
    /// Returns `None` if variable is not set.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(Shell::get_env("HOME"), "/home/roman");
    /// assert_eq!(Shell::get_env("MY_ANACONDA_DONT_WANT"), None); // unless you got buns, hun.
    /// ```
    pub fn get_env(name: &str) -> Option<String> {
        let value = env::var(name).ok()?;
        Some(String::from(value.trim()))
    }
}

/// Trait for creating shell commands.
pub trait CommandBuilder {
    /// Builds a shell command to set an environment variable.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the environment variable (e.g. `"STARSHIP_CONFIG"`)
    /// * `value` - Value of the environment variable (e.g. `"/home/nicki/.config/starship.toml"`)
    ///
    /// # Returns
    ///
    /// A string that contains the command to set the environment variable _name_
    /// to _value_ for the current shell (_self_). Returns `Error` if command cannot
    /// be built.
    ///
    /// # Warning
    ///
    /// Currently, **no escaping or validation** happens! Handle with absolute care!
    /// Make sure that you can trust the arguments that you pass!
    ///
    /// # Examples
    ///
    /// ```
    /// let fish_command = Shell::Fish::build_env_command("OH_MY", "GOSH")
    ///     .expect("Could not build env command");
    ///
    /// let zsh_command = Shell::Zsh::build_env_command("LOOK_AT", "HER B_TT")
    ///     .expect("Could not build env command");
    ///
    /// assert_eq!(fish_command, "set -Ux 'OH_MY' 'GOSH'");
    /// assert_eq!(zsh_command, "export 'LOOK_AT'='HER B_TT'");
    /// ```
    ///
    /// # TODO
    ///
    /// See [Warning](#warning). Make sure to escape / validate the given arguments.
    fn build_env_command(&self, name: &str, value: &str) -> Result<String, Error>;
}

impl CommandBuilder for Shell {
    fn build_env_command(&self, name: &str, value: &str) -> Result<String, Error> {
        // TODO: Escape / validate arguments

        match self {
            Self::Fish => Ok(format!("set -Ux '{name}' '{value}'")),
            Self::Zsh => Ok(format!("export '{name}'='{value}'")),
        }
    }
}
