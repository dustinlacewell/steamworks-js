import * as steamworks from '../index.js';

async function main() {
  const steam = new steamworks.SteamClient();
  const id = process.argv.length === 3 ? Number(process.argv[2]) : 3422500423
  await steam.workshop.subscribeToItem(id)
  console.log('Subscribed to item:', id)
}

main().catch(console.error);
