import { FC, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { PlusIcon, TrashIcon } from '@primer/octicons-react'

type Props = {
	value: string[]
	onChange: (value: string[]) => void
}
const FolderSelector: FC<Props> = ({ value, onChange }) => {
	useEffect(() => {
		const unlisten = listen<string>('folder_selected', ({ payload }) => onChange([...value, payload]))

		return () => {
			unlisten.then(f => f())
		}
	}, [onChange])

	return (
		<button
			onClick={() => invoke('select_folder')}
			className="p-2 px-2 border dark:border-white/5 rounded bg-black/[0.01] dark:bg-white/[0.01] w-full group/box"
		>
			<div className="space-y-2">
				{value.map(path => (
					<div
						className="bg-white dark:bg-white/5 border dark:border-white/5 rounded py-1 px-2 relative group"
						onClick={e => e.stopPropagation()}
					>
						<p className="text-left dark:text-white/60">{path}</p>
						<button
							onClick={() => onChange(value.filter(item => item != path))}
							className="absolute inset-y-0 right-2 text-black/40 dark:text-white/20 group-hover:text-black/60 dark:group-hover:text-white/30 hover:!text-red-500 dark:hover:!text-red-500 rounded-lg px-2 -mx-2 animate duration-300"
						>
							<TrashIcon />
						</button>
					</div>
				))}
			</div>
			{value.length == 0 ? (
				<div className="text-black/60 dark:text-white/80 text-center">
					<p>No folders selected</p>
					<p className="text-sm dark:text-white/60">Click here to add your first folder</p>
				</div>
			) : (
				<div className="text-black/40 dark:text-white/20 group-hover/box:text-black/60 dark:group-hover/box:text-white/30 text-center mt-2 flex items-center justify-center space-x-1 animate duration-300">
					<PlusIcon />
					<p className="text-sm">Add folder</p>
				</div>
			)}
		</button>
	)
}

export default FolderSelector
