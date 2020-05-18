# DungeonBot
Build status: [![build status](https://gitlab.com/bemyak/roll_bot/badges/master/build.svg)](https://gitlab.com/bemyak/roll-bot/commits/master)

Telegram Bot for D&D 5e players.

Implemented features:
- [X] Roll a die
- [ ] Search for spell reference
- [ ] Search for monster reference
- [ ] Search for items reference

## Before launch
The only thing needed to get the bot running is to set `ROLL_BOT_TOKEN` environment variable. You can obtain this token from the [BotFather](https://t.me/BotFather)

## Running through Tor Network
Since Telegram might be blocked in some countries, it makes sense to use Tor Network to get messages.

1. Install `tor` package from your linux distribution's repository.
2. Add line `HTTPTunnelPort 9080` somewhere in `/etc/tor/torrc` config file
3. Start `tor` service:  
   `systemctl start tor`  
   You now have two ports open: `9050` SOCKS5 proxy and `9080` -- HTTP.
4. Set `roll_bot_http_proxy` environment variable:  
   `export roll_bot_http_proxy=http://localhost:9080` or `set -x roll_bot_http_proxy http://localhost:9080` if you're using fish shell.

## Is it even legal?
This projects doesn't host any data, everything is fetched from https://5e.tools public API, so all legal questions should be addressed there.
