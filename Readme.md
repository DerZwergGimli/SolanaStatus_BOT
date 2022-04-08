# SolanaStatus_BOT

![example workflow](https://github.com/DerZwergGimli/SolanaStatus_BOT/actions/workflows/rust.yml/badge.svg)
![badge_license](https://img.shields.io/github/license/derzwerggimli/SolanaStatus_BOT)
![badge_discord](https://badgen.net/badge/icon/discord?icon=discord&label)

![badge_docker_size](https://badgen.net/docker/pulls/derzwerggimli/sol_status_bot)
![badge_docker_size](https://badgen.net/docker/size/derzwerggimli/sol_status_bot)

This repo contains a Discord bot that is monitoring the SolanaBlockchain.

!["bot icon""](icon.drawio.png)

## Current Feature:

- Display current tps (Transactions per Second)

## View

!["bot View1""](dc_view2.png)

!["bot view2""](dc_view1.png)

## Discord-Settings:

- ### BOT Permissions
    - Manage Role
    - Change Nickname
- ### BOT Roles
    - tickers-red
    - tickers-green

Create 2 Roles in you Discord Server so the bot will change its Nickname color based on that.

!["bot roles""](dc_roles.png)

### ENV

- Required:

```gitignore
RUST_LOG=error
DISCORD_TOKEN=<DISCORD_TOKEN>
LOOP_SLEEP=60
UPDATE_NAME=true
USER_ID=<BOT_USER_ID>
COLOR_THRESHOLD=999
```

- Oprional:

```gitignore
SOLANABEACH_TOKEN=<CAN BE EMPTY>
```

## Deploy via Docker

- Copy the `docker-compose.yaml.sample` to `docker-compose.yaml`
- Add you Token and later and BOT_ID
- Start you BOT with `docker-compose up -d`

### Donations:

- Solana-Wallet: `coffeeplease.sol`
- ETH-Address: `0xB0Be2420cA00C86aD983F246DEF49EA0F9779DCd`
