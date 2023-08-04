import './styles.css'
import React from 'react'
import Commit from './App'
import ReactDOM from 'react-dom/client'
import ThemeManager from './ThemeManager'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<ThemeManager>
			<Commit />
		</ThemeManager>
	</React.StrictMode>
)
