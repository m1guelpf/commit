import { FC, KeyboardEvent, memo, useCallback, useEffect, useState } from 'react'

type ParsedShortcut = {
	isCmd: boolean
	isAlt: boolean
	isCtrl: boolean
	isShift: boolean
	natural: string
}

type Props = {
	value: string
	onChange: (value: string) => void
}

const ShortcutPicker: FC<Props> = ({ value, onChange }) => {
	const [shortcut, setShortcut] = useState<ParsedShortcut>({
		isCmd: true,
		isAlt: true,
		natural: 'C',
		isCtrl: false,
		isShift: true,
	})

	console.log(shortcut)

	useEffect(() => {
		setShortcut(parseShortcut(value))
	}, [value])

	// useEffect(() => {
	// 	onChange(buildShortcut(shortcut))
	// }, [shortcut])

	const handleKeyDown = useCallback(
		(event: KeyboardEvent<HTMLDivElement>) => {
			event.preventDefault()

			const natural = getKey(event)
			const shortcut = {
				natural,
				isAlt: event.altKey,
				isCmd: event.metaKey,
				isCtrl: event.ctrlKey,
				isShift: event.shiftKey,
			} satisfies ParsedShortcut

			setShortcut(shortcut)

			if (natural.trim() != '') {
				;(event.target as HTMLDivElement).blur()
			}

			onChange(buildShortcut(shortcut))
		},
		[onChange]
	)

	return (
		<div
			tabIndex={0}
			onKeyUp={handleKeyDown}
			onKeyDown={handleKeyDown}
			onBlur={() => setShortcut(parseShortcut(value))}
			onFocus={() => setShortcut({ isCmd: false, isAlt: false, isCtrl: false, isShift: false, natural: '' })}
			className="py-1 bg-black/[.01] dark:bg-white/[.01] hover:bg-black/[.03] dark:hover:bg-white/[0.03] text-black/70 dark:text-white/70 focus:text-black/60 px-2 border dark:border-white/5 rounded-lg focus:cursor-pointer focus:outline focus:outline-blue-400 dark:focus:outline-blue-600/40 animate duration-100"
		>
			{displayShortcut(shortcut)}
		</div>
	)
}

const parseShortcut = (shortcut: string): ParsedShortcut => {
	const parts = shortcut.split('+')

	return {
		isAlt: parts.includes('Alt'),
		isCmd: parts.includes('Cmd'),
		isCtrl: parts.includes('Ctrl'),
		isShift: parts.includes('Shift'),
		natural: parts[parts.length - 1].toUpperCase(),
	}
}

const displayShortcut = (shortcut: ParsedShortcut): string => {
	let parts = []

	if (shortcut.isCmd) parts.push('⌘')
	if (shortcut.isCtrl) parts.push('⌃')
	if (shortcut.isShift) parts.push('⇧')
	if (shortcut.isAlt) parts.push('⌥')

	parts.push(shortcut.natural.toUpperCase())

	const display = parts.join('')

	return display.trim() == '' ? 'Recording...' : display
}

const buildShortcut = (shortcut: ParsedShortcut): string => {
	let parts = []

	if (shortcut.isCmd) parts.push('Cmd')
	if (shortcut.isAlt) parts.push('Alt')
	if (shortcut.isCtrl) parts.push('Ctrl')
	if (shortcut.isShift) parts.push('Shift')

	parts.push(shortcut.natural.toUpperCase())

	return parts.join('+')
}

const getKey = (event: KeyboardEvent<HTMLDivElement>) => {
	return [' ', 'Meta', 'Control', 'Shift', 'Alt'].includes(event.key)
		? ''
		: event.key.length > 1
		? event.key
		: event.code.startsWith('Key')
		? event.code.replace('Key', '')
		: event.key
				.normalize('NFD')
				.replace(/[\u0300-\u036f]/g, '')
				.toUpperCase()
				.replace('Dead', 'I')
}

export default memo(ShortcutPicker)
