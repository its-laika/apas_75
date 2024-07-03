# apas_75
_... connecting Starships!_

## About
The goal is to have a small program which updates the [Starship](https://starship.rs/)
configuration after each command, based on different theme config files that override / 
extend the main configuration.  
Similar the config `ZSH_THEME="random"` of [_Oh My Zsh_](https://ohmyz.sh/).

## Status
Basic functionality should work. This is the bare minimum.
Still working on making this program really useable.

For concrete TODOs, see [TODO.md](TODO.md).

## Usage
**Hint:** Starship configuration needs to be located at `~/.config/starship.toml`.

1. Edit your Starship configuration file. Insert the following indicators at where the custom
   config should be inserted:
   ```toml
   ### apas_75 theme start ###
   ### apas_75 theme end ###
   ```
2. Create custom theme file(s) in `~/.config/` with file names matching `starship-theme-[THEME NAME].toml`.
   Each time _apas\_75_ is called, one of the theme files will be inserted where the indicators of
   step 1 have been set.
3. Run _apas\_75_.
4. If you want to update the Starship config after every command, add _apas\_75_ as a precommand function.
   This differs for each shell. (**TODO: Add examples!**)

## What does _APAS-75_ mean?
The [APAS-75](https://en.wikipedia.org/wiki/Androgynous_Peripheral_Attach_System#APAS-75)
is a spacecraft docking mechanism, developed by soviet and american engineers. It was 
built to connect an Apollo and a Soyuz spacecraft during the _Apollo–Soyuz Test Project_.  
By "adapting" multiple _Starship_ theme files, you could say that this project aims for
something similar.

## License
This code is licensed under the MIT-License (see [LICENSE](LICENSE)).
