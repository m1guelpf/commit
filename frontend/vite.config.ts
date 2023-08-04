import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig(async () => ({
	plugins: [react()],
	clearScreen: false,
	server: {
		port: 1420,
		strictPort: true,
	},
	envPrefix: ['VITE_', 'TAURI_'],
}))
