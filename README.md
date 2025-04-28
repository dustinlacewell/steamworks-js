<h1 align="center">steamworks-ts</h1>
<p align="center">
This package provides typed NodeJS bindings to the <a href="https://partner.steamgames.com/doc/sdk">Steamworks SDK</a>.
<br /><br />
<a href="https://www.npmjs.com/package/steamworks-ts">
  <img src="https://img.shields.io/npm/v/%40ldlework%2Fsteamworks-ts?label=NPM" alt="NPM Version" />
</a>
<a href="LICENSE">
  <img src="https://img.shields.io/github/license/dustinlacewell/steamworks-ts?label=License" alt="License" />
</a>
<br /><br />
<a href="https://github.com/dustinlacewell/steamworks-ts/actions/workflows/build.yml">
  <img src="https://github.com/dustinlacewell/steamworks-ts/actions/workflows/build.yml/badge.svg" alt="Build" />
</a>
<a href="https://github.com/dustinlacewell/steamworks-ts/actions/workflows/docs.yml">
  <img src="https://github.com/dustinlacewell/steamworks-ts/actions/workflows/docs.yml/badge.svg" alt="Docs" />
</a>
<br/><br/>
<a href="https://steamworks.ldlework.com">Documentation</a>
</p>

## Usage

```ts
import * as steamworks from 'steamworks-ts';

const steam = new steamworks.SteamClient();
const user = steam.getCurrentUser();
console.log('Current User Info:');
console.log('  Steam ID:', user.steamId);
console.log('  Account ID:', user.accountId);
console.log('  Name:', user.name);
console.log('  Logged On:', user.loggedOn);
```

## License
This package is licensed under  [MIT](./LICENSE-MIT).

## Help, I can't run my game!
If you are seeing errors like `STATUS_DLL_NOT_FOUND`, `Image not found` etc. your platform is not likely supported by steamworks-ts.

<p align="center">
<table align="center" style="border: none;" cellspacing="0" cellpadding="0" border=0">
<tr>
<td>
Built with the <i>awesome</i> <a href="https://napi.rs/">napi-rs</a> project.</td>
<td>
<sub>
<img src="https://napi.rs/img/favicon.png" width="24" />
</sub>
</td>
</tr>
</table>