import * as steamworks from '../index.js';

async function main() {
  const steam = new steamworks.SteamClient();
  const friends = steam.getFriends();
  console.log(`\nYou have ${friends.length} friends:`)

  // Display friends
  for (const friend of friends.slice(0, 10)) { // Show first 10 friends
    console.log(`- ${friend.name} (${friend.steamId})`)
    console.log(`  State: ${friend.state}`)

    if (friend.gamePlayed) {
      console.log(`  Currently playing: ${friend.gamePlayed}`)
    }

    // Try to get rich presence of friend
    const richPresence = steam.getFriendRichPresence(friend.steamId, 'status')
    if (richPresence) {
      console.log(`  Rich presence status: ${richPresence}`)
    }
  }

  if (friends.length > 10) {
    console.log(`...and ${friends.length - 10} more.`)
  }
}

main().catch(console.error);
