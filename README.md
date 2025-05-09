<h1 align="center">steamworks-ts</h1>
<p align="center">
This package provides typed NodeJS bindings to the <a href="https://partner.steamgames.com/doc/sdk">Steamworks SDK</a>.
<br><br>
<a href="https://www.npmjs.com/package/steamworks-ts">
  <img src="https://img.shields.io/npm/v/%40ldlework%2Fsteamworks-ts?label=NPM" alt="NPM Version" /></a>
<a href="LICENSE">
  <img src="https://img.shields.io/github/license/dustinlacewell/steamworks-ts?label=License" alt="License" />
</a>
<br><br>
<a href="https://github.com/dustinlacewell/steamworks-ts/actions/workflows/build.yml">
  <img src="https://github.com/dustinlacewell/steamworks-ts/actions/workflows/build.yml/badge.svg" alt="Build" /></a>
<a href="https://github.com/dustinlacewell/steamworks-ts/actions/workflows/docs.yml">
  <img src="https://github.com/dustinlacewell/steamworks-ts/actions/workflows/docs.yml/badge.svg" alt="Docs" />
</a>
<br><br>
<a href="https://steamworks.ldlework.com">Documentation</a>
</p>

## Usage

```ts
import { SteamClient } from 'steamworks-ts';

const steam = new SteamClient();
const { steamId, accountId, name, loggedOn } = steam.getCurrentUser();
```

## Building locally

You must have a [Rust Toolchain](https://www.rust-lang.org/tools/install) and [NodeJS](https://nodejs.org/):

```bash
$ npm i
$ npm run dev
```

This should produce `steamworks-ts.<platform>-<arch>.node`.

## The `steam_appid.txt` File

Steamworks requires a file named `steam_appid.txt` containing the AppID of the Steam app Steamworks is meant to interact with. If this file doesn't exist, you'll get a warning and one will be created automatically:

```bash
$ npx tsx examples/user_info.ts 
Warning, creating default steam_appid.txt with 480 (Spacewar)
...
```

> Spacewar is an example application used to help developers understand the usage of the Steamworks API.

## Examples

After [building locally](#building-locally), you can run examples with `tsx`:

```
bash
$ npx tsx examples/user_info.ts
```

## License
This package is licensed under [MIT](./LICENSE-MIT).

## Help, I can't run my game/tool!
If you are seeing errors like `STATUS_DLL_NOT_FOUND`, `Image not found` etc. your platform is not likely supported by steamworks-ts or you forgot to [build](#building-locally).

<br><br>
<p align="center">
<table align="center" style="border: none;" cellspacing="0" cellpadding="0" border="0">
  <tr>
    <td colspan="2">Built ontop of <a href="https://github.com/Noxime/steamworks-rs">steamworks-rs</a></td>
  </tr>
  <tr>
    <td>Built with the <i>awesome</i> <a href="https://napi.rs/">napi-rs</a> project.</td>
    <td><sub><img src="https://napi.rs/img/favicon.png" width="24" /></sub></td>
  </tr>
</table>
</p>
