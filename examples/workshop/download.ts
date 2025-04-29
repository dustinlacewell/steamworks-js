import { command, run, positional } from 'cmd-ts'
import * as steamworks from '../../index.js'

const downloadCmd = command({
  name: 'download',
  args: {
    id: positional({ 
      displayName: 'workshop_id', 
      description: 'Workshop item ID to download' 
    }),
  },
  async handler({ id }) {
    const steam = new steamworks.SteamClient()
    const itemId = Number(id)
    await steam.workshop.downloadItem(itemId, true)
    console.log('Download started for:', itemId)
  }
})

run(downloadCmd, process.argv.slice(2))
