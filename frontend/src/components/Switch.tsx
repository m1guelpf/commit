import { cn } from '@/lib/utils'
import { forwardRef } from 'react'
import { Root, SwitchProps, Thumb } from '@radix-ui/react-switch'

const Switch = forwardRef<React.ElementRef<typeof Root>, SwitchProps>(({ className, ...props }, ref) => (
	<Root
		className={cn(
			'peer inline-flex h-[20px] w-[36px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-neutral-400 focus-visible:ring-offset-2 focus-visible:ring-offset-white disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-neutral-900 data-[state=unchecked]:bg-neutral-200 dark:focus-visible:ring-neutral-800 dark:focus-visible:ring-offset-neutral-950 dark:data-[state=checked]:bg-white/50 dark:data-[state=unchecked]:bg-white/40',
			className
		)}
		{...props}
		ref={ref}
	>
		<Thumb
			className={cn(
				'pointer-events-none block h-4 w-4 rounded-full bg-white shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-4 data-[state=unchecked]:translate-x-0 dark:bg-white/50'
			)}
		/>
	</Root>
))
Switch.displayName = Root.displayName

export default Switch
