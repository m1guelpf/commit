import { useEffect } from 'react'
import { listen } from '@tauri-apps/api/event'

const useTauriReset = (onReset: () => void) => {
	useEffect(() => {
		const unlisten = listen('reset', () => onReset())

		return () => {
			unlisten.then(f => f())
		}
	})
}

export default useTauriReset
