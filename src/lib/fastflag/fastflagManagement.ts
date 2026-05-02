import { appDataDir, join } from '@tauri-apps/api/path'
import {
    mkdir,
    exists,
    writeTextFile,
    readTextFile,
} from '@tauri-apps/plugin-fs'
import type { AppType } from '$lib/types'

async function ensureDir(path: string) {
    if (!(await exists(path))) {
        await mkdir(path, { recursive: true })
    }
}

function coerceValue(value: string): boolean | number | string {
    if (value === 'true') return true
    if (value === 'false') return false
    if (/^-?\d+(\.\d+)?$/.test(value)) return Number(value)
    return value
}

function decoerceValue(value: unknown): string {
    if (value === null || value === undefined) return ''
    return String(value)
}

export async function getFastFlags(
    version_hash: string,
    appType: AppType = 'player'
): Promise<Record<string, string>> {
    console.log(`[FastFlag] Loading flags for ${appType}, version: ${version_hash}`)
    if (!version_hash) return {}
    const baseDir = await appDataDir()
    const appFolder = appType === 'studio' ? 'Studio' : 'Player'
    const clientSettingFolder = await join(
        baseDir,
        appFolder,
        'Versions',
        version_hash,
        'ClientSettings'
    )
    const clientSettingFile = await join(
        clientSettingFolder,
        'ClientAppSettings.json'
    )
    
    try {
        if (!(await exists(clientSettingFile))) {
            console.log(`[FastFlag] No file found at ${clientSettingFile}`)
            return {}
        }

        const raw = await readTextFile(clientSettingFile)
        if (!raw.trim()) return {}
        const parsed: Record<string, unknown> = JSON.parse(raw)

        console.log(`[FastFlag] Loaded ${Object.keys(parsed).length} flags from disk`)

        return Object.fromEntries(
            Object.entries(parsed).map(([k, v]) => [k, decoerceValue(v)])
        )
    } catch (e) {
        console.error(`[FastFlag] Failed to load flags:`, e)
        return {}
    }
}

export async function saveFastFlags(
    version_hash: string,
    flags: Record<string, string>,
    appType: AppType = 'player'
): Promise<void> {
    console.log(`[FastFlag] Attempting save for ${appType}, version: ${version_hash}`)
    if (!version_hash) {
        console.error('[FastFlag] Aborting save: version_hash is empty')
        throw new Error('No version hash provided')
    }
    
    try {
        const baseDir = await appDataDir()
        const appFolder = appType === 'studio' ? 'Studio' : 'Player'
        const clientSettingFolder = await join(
            baseDir,
            appFolder,
            'Versions',
            version_hash,
            'ClientSettings'
        )
        const clientSettingFile = await join(
            clientSettingFolder,
            'ClientAppSettings.json'
        )
        
        await ensureDir(clientSettingFolder)

        const coerced = Object.fromEntries(
            Object.entries(flags)
                .filter(([_, v]) => v !== null && v !== undefined)
                .map(([k, v]) => [k, coerceValue(v)])
        )

        const content = JSON.stringify(coerced, null, 2)
        console.log(`[FastFlag] Writing ${content.length} bytes to ${clientSettingFile}`)

        await writeTextFile(clientSettingFile, content)
        
        // Final check
        const verifyExists = await exists(clientSettingFile)
        if (!verifyExists) throw new Error('File was not created after write operation')
        
        console.log('[FastFlag] Save successful and verified')
    } catch (err) {
        console.error('[FastFlag] CRITICAL ERROR during save:', err)
        throw err
    }
}
