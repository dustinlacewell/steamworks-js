import * as steamworks from '../../index.js';

async function main() {
  const steam = new steamworks.SteamClient();
  const user = steam.getCurrentUser();
  const query = await steam.workshop.queryUser(
    2212330, // yomi hustle
    user.accountId,
    steamworks.UserWorkshopListType.Published,
    steamworks.WorkshopUGCType.All,
    steamworks.UserWorkshopListOrder.TitleAsc,
    1
  );
  console.log(query);
}

main().catch(console.error);
