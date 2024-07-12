# apas_75
_... connecting Starships!_

## About
The goal is to have a small program which updates the [Starship](https://starship.rs/)
configuration after each command, based on multiple Starship config files.
Similar the `ZSH_THEME="random"`-config of [_Oh My Zsh_](https://ohmyz.sh/).

## Status
Basic functionality should work. This is the bare minimum.
Still working on making this program really useable.

For concrete TODOs, see [TODO.md](TODO.md).

## Usage
1. Create folder `~/.config/apas_75/`. (You also can set a custom folder to use in step 3.)
2. Move Starship themes in the new (custom) folder.
3. Ensure that your shell executes the stdout of _apas\_75_ before or after each command.
   This differs for each shell.

   e.g. _fish_:
   ```fish
   function call_apas_75 --on-event fish_preexec
      eval $(apas_75 [custom config path])
   end
   ```

### Arguments
* `[custom config path]` - Optional. If defined, _apas\_75_ will use this folder instead of 
  the default (`~/.config/apas_75/`).

## Currently supported shells
Currently only:

- `zsh` and
- `fish`

I'll may extend this, depending on who else will use it someday.

## What does _APAS-75_ mean?
The [APAS-75](https://en.wikipedia.org/wiki/Androgynous_Peripheral_Attach_System#APAS-75)
is a spacecraft docking mechanism, developed by soviet and american engineers. It was
built to connect an Apollo and a Soyuz spacecraft during the _Apollo–Soyuz Test Project_.
By "adapting" multiple _Starship_ theme files, you could say that this project aims for
something similar.

## Testimonials

> "this could have been a 10 line bash script!" ~ my co-worker

## License
This code is licensed under the MIT-License (see [LICENSE](LICENSE)).
