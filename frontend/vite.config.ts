import path from 'path'
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig(async () => ({
	plugins: [react()],
	clearScreen: false,
	envPrefix: ['VITE_', 'TAURI_'],
	resolve: {
		alias: {
			'@': path.resolve(__dirname, './src'),
		},
	},
	server: {
		port: 1420,
		strictPort: true,
	},
}))
