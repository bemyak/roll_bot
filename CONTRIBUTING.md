## Contributions are welcome and greatly appreciated!

### Recommended starting gear
- Latest stable Rust toolchain.
  Check the [rustup](https://rustup.rs/) project or install `rust` with your distribution's package manager (should include cargo as well)
- `vscode` or `vscodium` with `rust-analyzer` extension
- `ROLL_BOT_TOKEN` environment variable set. Get yours from the [BotFather](https://t.me/BotFather)
- `ROLL_BOT_USE_TEST_DB` just set this environment variable to any value (e.g. `true`) to place your database under `/test_data` and not span project root

### Main development principles
1. No crashes!  
   The bot is supposed to work in a daemon-like way, so no panics should occur and all errors should be intercepted and properly handled.  
   Backtraces should not be visible to a user, but properly logged.
2. Nice UI and UX.  
   Before implementing a feature, please spend some time thinking on how an end user will use it.  
   Try some designing and experimenting first before settings the things up into code.
3. Clear, simple and idiomatic code is more important then performance.  
   Rust offers multiple ways to achieve the same goal, but only a few of them will look and feel nice.  
   Since the main goal of this project is learning, please try different approaches.  
   Usually focusing on *data* instead of a *process* is a key to get things rolling in Rust
4. And the most important: **A poorly working thing now is better then the perfect thing never.**  
   Try not to overthink it :)
