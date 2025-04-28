# steamworks
[![NPM Version](https://img.shields.io/npm/v/%40ldlework%2Fsteamworks-ts)](https://www.npmjs.com/package/@ldlework/steamworks-ts)
[![License](https://img.shields.io/github/license/dustinlacewell/steamworks-ts?label=License)](LICENSE)

This package provides TypeScript bindings to the [Steamworks SDK](https://partner.steamgames.com/doc/sdk).

[Documentation](https://steamworks.ldlework.com)

```ts
import * as steamworks from '@ldlework/steamworks-ts';

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
If you are seeing errors like `STATUS_DLL_NOT_FOUND`, `Image not found` etc. your platform is not likely supported by Steamworks-ts.