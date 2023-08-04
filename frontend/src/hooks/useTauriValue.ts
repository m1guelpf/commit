import { useEffect, useState } from 'react'
import useTauriReset from './useTauriReset'
import { listen } from '@tauri-apps/api/event'

const useTauriValue = <T>(event: string): T | null => {
	const [value, setValue] = useState<T | null>(null)
	useTauriReset(() => setValue(null))

	useEffect(() => {
		const unlisten = listen(event, e => setValue(e.payload as T | null))

		return () => {
			unlisten.then(f => f())
		}
	})

	return value
}

export default useTauriValue
