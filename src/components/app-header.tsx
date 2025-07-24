// FILE: src/components/app-header.tsx
'use client';

import { usePathname } from 'next/navigation';
import Link from 'next/link';
import { ThemeSelect } from '@/components/theme-select';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

// ===================================================================
// THIS IS THE DEFINITIVE FIX FOR THE HYDRATION ERROR.
// We are importing and using our new ClientOnly component to wrap the
// WalletMultiButton, preventing it from rendering on the server.
// ===================================================================
import { ClientOnly } from '@/components/ClientOnly';

export function AppHeader({ links = [] }: { links: { label: string; path: string }[] }) {
    const pathname = usePathname();

    function isActive(path: string) {
        return path === '/' ? pathname === '/' : pathname.startsWith(path);
    }

    return (
        <header className="relative z-50 px-4 py-2 bg-neutral-100 dark:bg-neutral-900 dark:text-neutral-400 border-b dark:border-neutral-800">
            <div className="mx-auto flex justify-between items-center">
                {/* Left Side: Brand and Navigation Links */}
                <div className="flex items-baseline gap-x-4">
                    <Link className="text-xl font-bold hover:text-neutral-500 dark:hover:text-white" href="/">
                        <span>Anaheim</span>
                    </Link>
                    <nav>
                        <ul className="flex gap-x-4 items-center">
                            {links.map(({ label, path }) => (
                                <li key={path}>
                                    <Link
                                        className={`hover:text-neutral-500 dark:hover:text-white ${isActive(path) ? 'font-semibold text-neutral-900 dark:text-white' : ''}`}
                                        href={path}
                                    >
                                        {label}
                                    </Link>
                                </li>
                            ))}
                        </ul>
                    </nav>
                </div>

                {/* Right Side: Wallet Button and Theme Selector */}
                <div className="flex items-center gap-x-4">
                    <ClientOnly>
                        <WalletMultiButton />
                    </ClientOnly>
                    <ThemeSelect />
                </div>
            </div>
        </header>
    );
}