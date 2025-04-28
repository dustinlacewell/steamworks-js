import * as steamworks from '../../../index.js';

async function main() {
  const steam = new steamworks.SteamClient();
  const details = await steam.workshop.getItem(123);
  if (details == null) {
    console.log('Received null as expected for invalid item (not found).');
  } else {
    console.log('Unexpected success: details returned for invalid item:', details);
  }
}

main().catch(console.error);
