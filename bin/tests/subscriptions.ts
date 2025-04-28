import * as steamworks from '../../index.js';

async function main() {
  const steam = new steamworks.SteamClient();
  const items = steam.getSubscriptions();
  console.log(`You have ${items.length} subscribed Workshop Items:`)
  const work = items.map(async item => {
    const detail = steam.workshop.getItem(item)
    const install = steam.workshop.getInstallInfo(item)
    const states = steam.workshop.getItemStates(item)
    return Promise.all([detail, install, states])
  })
  const data = await Promise.all(work)
  for (const [detail, install, states] of data) {
    if (detail) {
      console.log(`- ${detail.title} (${detail.itemId}) ${install ? 'installed' : 'not installed'}`);
      console.log(`  States: ${states.map(s => steam.workshop.itemStateToString(s)).join(', ')}`);
    }
  }
}

main().catch(console.error);
