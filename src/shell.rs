use std::{env, io::Error};

/// Key for environment variable that determines which configuration file
/// Starship will use.
pub const ENV_KEY_STARSHIP_CONFIG: &str = "STARSHIP_CONFIG";

const ENV_KEY_STARSHIP_SHELL: &str = "STARSHIP_SHELL";
const ENV_SHELL_VALUE_ZSH: &str = "zsh";
const ENV_SHELL_VALUE_FISH: &str = "fish";
const ENV_KEY_ALLOWED_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ_1234567890";

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
    ///             Note that `value` must not contain single quotes as otherwise it
    ///             cannot be escaped properly.
    ///
    /// # Returns
    ///
    /// A string that contains the command to set the environment variable _name_
    /// to _value_ for the current shell (_self_). Returns `Error` if command cannot
    /// be built.
    ///
    /// # Examples
    ///
    /// ```
    /// let fish_command = Shell::Fish.build_env_command("OHMY", "GOSH");
    /// assert!(fish_command.is_ok());
    /// assert_eq!(fish_command.unwrap(), "set -Ux 'OHMY' 'GOSH'");
    ///
    /// let zsh_command = Shell::Zsh.build_env_command("LOOKAT", "HER B*TT");
    /// assert!zsh_command.is_ok());
    /// assert_eq!(zsh_command.unwrap(), "export 'LOOKAT'='HER B*TT'");
    ///
    /// // Will fail because of single quotes:
    /// assert!(Shell::Zsh.build_env_command("THIS_DUDE", "NAMED 'MICHAEL'").is_err());
    /// ```
    fn build_env_command(&self, name: &str, value: &str) -> Result<String, Error>;
}

impl CommandBuilder for Shell {
    fn build_env_command(&self, name: &str, value: &str) -> Result<String, Error> {
        if !is_valid_environment_variable_name(name) {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid environment variable name: '{name}'"),
            ));
        }

        /* We quote the given (unsafe!) parameters with single quotes for given reasons:

        "All characters enclosed between a pair of single quotes ('')
        that is not preceded by a '$' are quoted."
        - https://zsh.sourceforge.io/Doc/Release/Shell-Grammar.html#Reserved-Words

        "Sometimes features such as parameter expansion and character escapes get
        in the way. When that happens, the user can write a parameter within quotes,
        either ' (single quote) or " (double quote). There is one important
        difference between single quoted and double quoted strings: When using double
        quoted string, variable expansion still takes place. Other than that, a quoted
        parameter will not be parameter expanded, may contain spaces, and escape
        sequences are ignored."
        - https://fishshell.com/docs/2.0/index.html */

        if value.contains('\'') {
            /* A single quote cannot appear within single quotes [...]"
            https://zsh.sourceforge.io/Doc/Release/Shell-Grammar.html#Reserved-Words
            Same goes for fish. */
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Value must not contain single quotes",
            ));
        }

        match self {
            Self::Fish => Ok(format!("set -Ux '{name}' '{value}'")),
            Self::Zsh => Ok(format!("export '{name}'='{value}'")),
        }
    }
}

/// Returns whether a given `name` is a valid environment variable name.
/// It is considered valid if it only contains uppercase ASCII characters,
/// underscores, digits and does **not** start with a digit.
///
/// # Arguments
///
/// * `name` - Name of the possible environment variable name
///
/// # Example
///
/// ```
/// assert!(is_valid_environment_variable_name("HOME"));
/// assert!(!is_valid_environment_variable_name("KE$HA"));
/// assert!(!is_valid_environment_variable_name("brat and its a variable name"));
/// ```
///
/// # See
/// [IEEE Std 1003.1](https://pubs.opengroup.org/onlinepubs/000095399/basedefs/xbd_chap08.html)
fn is_valid_environment_variable_name(name: &str) -> bool {
    for character in name.chars() {
        if !ENV_KEY_ALLOWED_CHARS.contains(character) {
            return false;
        }
    }

    let Some(first_char) = name.chars().next() else {
        return false;
    };

    !first_char.is_ascii_digit()
}

mod tests {
    use super::*;

    #[test]
    fn test_is_valid_environment_variable_name() {
        assert!(is_valid_environment_variable_name(ENV_KEY_STARSHIP_CONFIG));
        assert!(!is_valid_environment_variable_name("365_PARTY_GIRL"));
        assert!(!is_valid_environment_variable_name("CLUB CLASSICS"));
        assert!(!is_valid_environment_variable_name("'=1;"));
    }

    #[test]
    fn test_build_env_command() {
        assert!(Shell::Fish
            .build_env_command("TALK", "TALK")
            .is_ok_and(|v| v == "set -Ux 'TALK' 'TALK'"));
        assert!(Shell::Zsh
            .build_env_command("TALK", "TALK")
            .is_ok_and(|v| v == "export 'TALK'='TALK'"));

        assert!(Shell::Zsh.build_env_command("365_PARTY", "GIRL").is_err());
        assert!(Shell::Zsh.build_env_command("CLUB", "'CLASSICS'").is_err());
    }

    #[test]
    fn test_get_current_works() {
        env::set_var("STARSHIP_SHELL", "fish");

        let shell = Shell::get_current();
        assert!(shell.is_some());

        let shell = shell.unwrap();
        assert!(matches!(shell, Shell::Fish));
    }

    #[test]
    fn test_get_current_is_none_on_unknown() {
        env::set_var("STARSHIP_SHELL", "bash");

        let shell = Shell::get_current();
        assert!(shell.is_none());
    }

    #[test]
    fn test_get_current_is_none_on_unset() {
        env::remove_var("STARSHIP_SHELL");

        let shell = Shell::get_current();
        assert!(shell.is_none());
    }
}
