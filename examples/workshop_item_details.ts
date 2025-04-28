import * as steamworks from '../index.js';

async function main() {
  const steam = new steamworks.SteamClient();
  // Replace with a real, known-good workshop item ID for your environment
  const validItemId = 3146412764;
  const details = await steam.workshop.getItem(validItemId);
  if (details) {
    console.log('Workshop item details:', details);
  } else {
    console.log('No details found for valid workshop item!');
  }
}

main().catch(console.error);
