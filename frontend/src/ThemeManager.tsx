import { Theme, getCurrent } from '@tauri-apps/api/window'
import { FC, PropsWithChildren, useEffect, useState } from 'react'

const ThemeManager: FC<PropsWithChildren<{}>> = ({ children }) => {
	const [theme, setTheme] = useState<Theme>('light')

	useEffect(() => {
		const tauri = getCurrent()

		tauri.theme().then(theme => setTheme(theme ?? 'light'))
		const unlisten = tauri.onThemeChanged(({ payload }) => setTheme(payload))

		return () => {
			unlisten.then(f => f())
		}
	})

	return <div className={theme}>{children}</div>
}

export default ThemeManager
