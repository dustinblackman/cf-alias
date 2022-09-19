![cf-alias](.github/banner.jpg)

[![Release](https://img.shields.io/github/v/release/dustinblackman/cf-alias)](https://github.com/dustinblackman/cf-alias/releases)

> Create Cloudflare email alias' directly from your terminal or Alfred.

- [Overview](#Overview)
- [Install](#Install)
  - [MacOS](#macos)
  - [Debian / Ubuntu](#debian--ubuntu)
  - [Fedora / CentOS](#fedora--centos)
  - [Nix](#nix)
  - [Arch Linux](#arch-linux)
  - [Windows](#windows)
  - [Manual](#manual)
  - [Source](#source)
- [Usage](#Usage)
  - [CLI](#cli)
  - [Alfred](#alfred)

## Overview

Create and manage CloudFlare email alias's easily to reduce the barrier in creating unique emails per online account you own. You can list all alias to easily copy to clipboard, create unique alias of your choosing, or generate random ones!

This was done quickly and just for fun, and is using an undocumented Cloudflare API. Bugs are expected!

![screenshot1](./.github/screenshots/1.jpg)

<!-- command-help start -->

```
cf-alias v0.1.1
CLI interface for Cloudflare Email Routing

USAGE:
    cf-alias <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    alfred        Commands for the Alfred extension
    completion    Generates shell completions
    create        Creates a new forwarding email
    help          Print this message or the help of the given subcommand(s)
    list          List existing email routes.
```

<!-- command-help end -->

## Install

### MacOS

```sh
brew install dustinblackman/tab/cf-alias
```

### Debian / Ubuntu

```sh
curl -s https://dustinblackman.github.io/apt/deb/KEY.gpg | apt-key add -
curl -s https://dustinblackman.github.io/apt/deb/dustinblackman.list > /etc/apt/sources.list.d/dustinblackman.list
sudo apt-get update
sudo apt-get install cf-alias
```

### Fedora / CentOS

```sh
yum-config-manager --add-repo https://dustinblackman.github.io/yum/config.repo
yum install cf-alias
```

### Nix

```sh
nix-env -f '<nixpkgs>' -iA nur.repos.dustinblackman.cf-alias
```

### Arch Linux

```sh
yay -S cf-alias-bin
```

### Windows

```sh
scoop bucket add dustinblackman https://github.com/dustinblackman/scoop-bucket.git
scoop install cf-alias
```

### Manual

Download the pre-compiled binaries and packages from the [releases page](https://github.com/dustinblackman/cf-alias/releases) and
copy to the desired location.

### Source

```sh
git clone https://github.com/dustinblackman/cf-alias.git
cd cf-alias
cargo install --path .
```

## Usage

Create a configuration file in `$HOME/.cf-alias.json` with the following keys. You can find values for each in the
Cloudflare dashboard.

```json
{
  "cloudflare_account_id": "cloudflare-acccount-id-here",
  "cloudflare_forward_email": "mygmail@gmail.com",
  "cloudflare_root_domain": "mydomain.com",
  "cloudflare_token": "cloudflare-api-token",
  "cloudflare_zone": "cloud-zone-id-for-mydomain.com"
}
```

### CLI

- `cf-alias list` to list existing forwarders.
- `cf-alias create --email-prefix github` to create a email fowarder as `github@mydomain.com`
- `cf-alias create --random` to create a randomly generated email forwarder.

### Alfred

Install the Alfred extension by
[downloading](https://github.com/dustinblackman/cf-alias/blob/master/alfred/mx.alfredworkflow?raw=true) the extension, and dragging it in to Alfred to install. Use `mx` as a prefix to bring up all the available commands.

![screenshot1](./.github/screenshots/1.jpg)

You can create a specific email forwarder with `mx create MY-NAME`.

![screenshot2](./.github/screenshots/2.jpg)

cf-alias can also generate you a random email forwarder by running `mx create random`.

![screenshot3](./.github/screenshots/3.jpg)
