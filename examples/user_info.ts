import * as steamworks from '../index.js';

async function main() {
  const steam = new steamworks.SteamClient();
  const user = steam.getCurrentUser();
  console.log('Current User Info:');
  console.log('  Steam ID:', user.steamId);
  console.log('  Account ID:', user.accountId);
  console.log('  Name:', user.name);
  console.log('  State:', user.state);
  console.log('  Level:', user.level);
  console.log('  Logged On:', user.loggedOn);
}

main().catch(console.error);
