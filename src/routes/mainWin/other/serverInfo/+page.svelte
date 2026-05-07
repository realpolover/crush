<script lang="ts">
    import Button from '$lib/components/atoms/Button.svelte'
    import ExpandableSettingCard from '$lib/components/molecules/ExpandableSettingCard.svelte'
    import type { ServerInfoFromBackend } from '$lib/types'
    import { listen } from '@tauri-apps/api/event'
    import { getMessageFormatter } from 'svelte-i18n'
    import { writeText } from '@tauri-apps/plugin-clipboard-manager';
    import { fetch } from '@tauri-apps/plugin-http';
    import ClickableCard from '$lib/components/molecules/ClickableCard.svelte'

    let serverInstanceId: string = 'Unkown'
    let gameId: number = 1234
    let serverInviteLink: string
    let gameName: string = "Unkown game"

    async function getUniverse(
        placeId: number
    ): Promise<{ universeId: number } | null> {
        return await fetch(
            `https://apis.roblox.com/universes/v1/places/${placeId}/universe`
        )
            .then((r) => r.json())
            .catch(() => null)
    }

    async function getGameDetails(
        placeId: number,
        universeId: number
    ): Promise<{ name: string; imageUrl: string | null }> {
        const [nameRes, iconRes] = await Promise.all([
            await fetch(`https://games.roblox.com/v1/games?universeIds=${universeId}`)
                .then((r) => r.json())
                .catch(() => null),
            await fetch(
                `https://thumbnails.roblox.com/v1/games/icons?universeIds=${universeId}&returnPolicy=PlaceHolder&size=512x512&format=Png&isCircular=false`
            )
                .then((r) => r.json())
                .catch(() => null),
        ])

        const details = {
            name: nameRes?.data?.[0]?.name ?? 'Unknown Game',
            imageUrl: iconRes?.data?.[0]?.imageUrl ?? null,
        }

        return details
    }

    async function copyToClipboard(url) {
        await writeText(url);
    }

    listen<ServerInfoFromBackend>('serverInfomation', async (event) => {
        serverInstanceId = event.payload.server_id
        gameId = event.payload.game_id

        serverInviteLink = `https://deeplink.multicrew.dev?placeId=${gameId}&jobId=${serverInstanceId}`

        const universeData = await getUniverse(gameId)

        if (!universeData) {
            gameName = 'Unknown Game'
            return
        }

        const details = await getGameDetails(
            gameId,
            universeData.universeId
        )

        gameName = details.name
    })
</script>

<div class="flex flex-col gap-4">
    <h1 class="text-3xl">Server Infomation</h1>

    <ExpandableSettingCard
        title={gameName}
        description="Infomation about this server"
        isOpen={true}
    >
        <div class="flex flex-col gap-3 p-4">
            <p>Uptime : 1 hours</p>
            <p>Server Instance ID : {serverInstanceId}</p>
            <p class="flex items-center gap-2">
                Copy invite link : <Button variant="secondary" on:click={async () => { await copyToClipboard(serverInviteLink)}} >Copy to clipboard</Button>
            </p>
        </div>
    </ExpandableSettingCard>
</div>
