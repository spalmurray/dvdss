# dvdss

DVD screensaver for your terminal.

I had been using [https://dvdscreensaver.net/](https://dvdscreensaver.net/) for an on demand screensaver for a while when I decided it would be a fun project to try to implement this to run in a terminal.

Pretty basic program. Only dependencies are `terminal_size` and `ctrlc` (for likely obvious reasons lol).

If you want to tweak things, you can write pixel art made of `#` to `shape` and then run `./shapetopixels.py` to output the text for hardcoding the shape as `GRAPHIC` in `src/main.rs`. You'll also need to update the `GRAPHIC_WIDTH` and `GRAPHIC_HEIGHT` accordingly. `FRAME_DURATION` is also configurable in `main.rs`. Would probably be better to read that in as an environment variable.

## Installation

Clone the repo and slam `cargo install --path .`
