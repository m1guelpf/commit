import { useEffect, useState } from 'react'
import useTauriReset from './useTauriReset'
import { listen } from '@tauri-apps/api/event'

const useTauriValue = <T>(event: string): T | null => {
	const [value, setValue] = useState<T | null>(null)
	useTauriReset(() => setValue(null))

	useEffect(() => {
		const unlisten = listen<T | null>(event, e => setValue(e.payload))

		return () => {
			unlisten.then(f => f())
		}
	})

	return value
}

export default useTauriValue
