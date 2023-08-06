import './styles.css'
import React from 'react'
import Commit from './App'
import Settings from './Settings'
import ReactDOM from 'react-dom/client'
import ThemeManager from './ThemeManager'

type CommitSettings = {
	page: 'main' | 'settings'
}

declare global {
	interface Window {
		__COMMIT__?: CommitSettings
	}
}

const settings = window.__COMMIT__ ?? { page: 'main' }

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<ThemeManager>{settings.page == 'settings' ? <Settings /> : <Commit />}</ThemeManager>
	</React.StrictMode>
)
