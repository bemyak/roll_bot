# DungeonBot
Build status: [![build status](https://gitlab.com/bemyak/roll_bot/badges/master/pipeline.svg)](https://gitlab.com/bemyak/roll-bot/commits/master)

Telegram Bot for D&D 5e players. You can test it [here](https://t.me/roll_bot).

Implemented features:
- [X] Roll a die
- [X] Search for spell reference
- [X] Search for monster reference
- [X] Search for items reference

## Before launch
The only thing needed to get the bot running is to set `ROLL_BOT_TOKEN` environment variable. You can obtain this token from the [BotFather](https://t.me/BotFather)

## Running through Tor Network
Since Telegram might be blocked in some countries, it makes sense to use Tor Network to get messages.

1. Install `tor` package from your linux distribution's repository.
2. Add line `HTTPTunnelPort 9080` somewhere in `/etc/tor/torrc` config file
3. Start `tor` service:  
   `systemctl start tor`  
   You now have two ports open: `9050` as SOCKS5 proxy and `9080` as HTTP. Unfortunately, we don't support SOCKS5 proxy at the moment.
4. Set `roll_bot_http_proxy` environment variable:  
   `export roll_bot_http_proxy=http://localhost:9080` or `set -x roll_bot_http_proxy http://localhost:9080` if you're using fish shell.

# Data Source Concerns/Issues?
This project's data is pulled from a [3rd party source](https://5e.tools), and typos or questions are best presented there.
