import Switch from '@/components/Switch'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'
import { useCallback, useEffect, useState } from 'react'
import ShortcutPicker from './components/ShortcutPicker'
import FolderSelector from './components/FolderSelector'
import { FileDirectoryIcon, RepoPushIcon, RocketIcon, VersionsIcon } from '@primer/octicons-react'

type Config = {
	shortcut: string
	autostart: boolean
	repo_paths: string[]
	should_push: boolean
}

const Settings = () => {
	const [config, setConfig] = useState<Config | null>(null)

	useEffect(() => {
		if (config) return

		requestAnimationFrame(() => invoke('get_config'))
	}, [config])

	const save = useCallback((newConfig: Config) => {
		invoke('update_config', { newConfig })
	}, [])

	useEffect(() => {
		const unlisten = listen<Config>('config', ({ payload }) => setConfig(payload))

		return () => {
			unlisten.then(f => f())
		}
	})

	return (
		<div className="bg-white/[.99] dark:bg-black/20 min-h-screen">
			<div
				className="fixed top-0 inset-y-0 h-[28px] w-full cursor-grab active:cursor-grabbing bg-white/[.99] dark:bg-black/20 shadow"
				data-tauri-drag-region
			>
				<div className="h-full w-16 cursor-pointer" />
			</div>
			{config && (
				<div className="p-12 pt-14 space-y-12">
					<div>
						<div className="mb-2">
							<div className="flex items-center space-x-2 mb-0.5 dark:text-white/80">
								<FileDirectoryIcon />
								<p>Project Folders</p>
							</div>
							<p className="text-sm text-black/60 dark:text-white/50">
								Folders to crawl for repos. Commit will only be able to find your repos if they&apos;re
								inside one of these folders.
							</p>
						</div>
						<FolderSelector
							value={config.repo_paths}
							onChange={repo_paths => save({ ...config, repo_paths })}
						/>
					</div>
					<div className="">
						<div className="flex items-center space-x-2 mb-0.5 dark:text-white/80">
							<VersionsIcon />
							<p>Global Shortcut</p>
						</div>
						<p className="text-sm text-black/60 mb-2 max-w-prose dark:text-white/50">
							This shortcut will invoke Commit from anywhere in the system. Make sure it doesn't conflict
							with other local shortcuts.
						</p>
						<ShortcutPicker value={config.shortcut} onChange={shortcut => save({ ...config, shortcut })} />
					</div>
					<div className="flex items-center justify-between space-x-6">
						<div>
							<div className="flex items-center space-x-2 mb-0.5 dark:text-white/80">
								<RocketIcon />
								<p>Launch at log in</p>
							</div>
							<p className="text-sm text-black/60 dark:text-white/50">
								Start Commit in the background when you log in to your computer. You&apos;ll be able to
								invoke it from the tray icon or with the global shortcut.
							</p>
						</div>
						<Switch
							checked={config.autostart}
							onCheckedChange={autostart => save({ ...config, autostart })}
						/>
					</div>
					<div className="flex items-center justify-between space-x-6">
						<div>
							<div className="flex items-center space-x-2 mb-0.5 dark:text-white/90">
								<RepoPushIcon />
								<p>Push to remote</p>
							</div>
							<p className="text-sm text-black/60 dark:text-white/50">
								Sync your commits to the remote after you&apos;ve committed. This will only work if you
								have an origin remote set up.
							</p>
						</div>
						<Switch
							checked={config.should_push}
							onCheckedChange={should_push => save({ ...config, should_push })}
						/>
					</div>
				</div>
			)}
		</div>
	)
}

export default Settings
