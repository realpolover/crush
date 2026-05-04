<script lang="ts">
    import { onMount } from 'svelte'
    import FastFlagTable from '$lib/components/organisms/FastFlagTable.svelte'
    import { getLatestVersion, getCurrentInstallation } from '$lib/downloadRoblox'
    import {
        getFastFlags,
        saveFastFlags,
    } from '$lib/fastflag/fastflagManagement'
    import { invoke } from '@tauri-apps/api/core'
    import { _ } from 'svelte-i18n'
    import { goto } from '$app/navigation'
    import { get } from 'svelte/store'
    import { launchAppType } from '$lib/stores/launchAppType'
    import type { AppType } from '$lib/types'
    import Button from '$lib/components/atoms/Button.svelte'

    let flags: Record<string, string> = {}
    let version = ''
    let appType: AppType = 'player'

    onMount(async () => {
        appType = (get(launchAppType) as AppType) || 'player'
        
        const installation = await getCurrentInstallation(appType)
        if (installation && installation.exists) {
            version = installation.version
            console.log('[Editor] Detected installed version:', version)
        } else {
            version = await getLatestVersion(appType)
            console.log('[Editor] No verified installation, falling back to latest in store:', version)
        }

        flags = await getFastFlags(version, appType)

        console.log('Current fast flags:', flags)

        invoke('set_rpc', {
            details: $_('rpc.general'),
            stateText: $_('rpc.fastflag'),
        })

    })

    async function handleDelete(event: CustomEvent<string>) {
        const latestFlags = await getFastFlags(version, appType)
        const { [event.detail]: _, ...rest } = latestFlags
        flags = rest
        await saveFastFlags(version, flags, appType)
    }

    async function handleAdd(
        event: CustomEvent<{ name: string; value: string }>
    ) {
        const { name, value } = event.detail
        const latestFlags = await getFastFlags(version, appType)
        if (name in latestFlags) return
        flags = { ...latestFlags, [name]: value }
        await saveFastFlags(version, flags, appType)
    }

    async function handleUpdate(
        event: CustomEvent<{ name: string; value: string }>
    ) {
        const { name, value } = event.detail
        const latestFlags = await getFastFlags(version, appType)
        flags = { ...latestFlags, [name]: value }
        await saveFastFlags(version, flags, appType)
    }

    function handleSearch(event: CustomEvent<string>) {
        const query = event.detail
        console.log(`Searching for: ${query}`)
    }
</script>

<div class="flex flex-col gap-8">
    <div class="flex items-center justify-between">
        <div>
            <h1 class="text-3xl font-bold tracking-tight text-stone-100">
                {$_('pages.fastflag.fastflag')}
            </h1>
            <p class="text-stone-400 mt-1">
                {$_('pages.fastflag.description')}
            </p>
        </div>
        <div class="flex items-center gap-2">
            <Button variant="secondary" onclick={() => goto('../fastflags')}>
                {$_('pages.fastflag.generalBack')}
            </Button>
        </div>
    </div>
    <FastFlagTable
        {flags}
        on:delete={handleDelete}
        on:add={handleAdd}
        on:update={handleUpdate}
        on:search={handleSearch}
    />
</div>
