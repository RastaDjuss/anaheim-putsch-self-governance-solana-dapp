// FILE: src/components/mining/mining-panel.tsx
'use client';

import React from 'react';
import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { Button } from '@/components/ui/button';
import { getAnaheimAccount, ANAHEIM_ACCOUNT_PUBKEY, ANAHEIM_PROGRAM_ID, createAnaheimProgram } from '@/lib/anaheim-program';
import { web3 } from '@coral-xyz/anchor';

import anaheimAccountKey from '@/lib/anaheim-account-key.json';

// ===================================================================
// THIS IS THE DEFINITIVE FIX.
// The hooks are now complete, and the main component at the bottom
// now correctly uses them to render the UI and handle button clicks.
// This will resolve all the "Unused function" and "Property does not exist" errors.
// ===================================================================

// --- HOOKS ---

function useAnaheimAccountQuery() {
    const { connection } = useConnection();
    return useQuery({
        queryKey: ['anaheim-account'],
        queryFn: () => getAnaheimAccount(connection),
    });
}

function useIncrementMutation() {
    const { connection } = useConnection();
    const wallet = useWallet();
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: async () => {
            if (!wallet.connected || !wallet.publicKey) { throw new Error('Wallet not connected'); }
            const program = createAnaheimProgram(connection, wallet);
            const signature = await program.methods
                .increment()
                .accounts({ anaheim: ANAHEIM_ACCOUNT_PUBKEY! })
                .rpc();
            const latestBlockhash = await connection.getLatestBlockhash();
            await connection.confirmTransaction({ signature, ...latestBlockhash });
            return signature;
        },
        onSuccess: () => {
            return queryClient.invalidateQueries({ queryKey: ['anaheim-account'] });
        },
        onError: (error: Error) => console.error('Increment failed:', error),
    });
}

function useInitializeMutation() {
    const { connection } = useConnection();
    const wallet = useWallet();
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: async () => {
            if (!wallet.connected || !wallet.publicKey) { throw new Error('Wallet not connected'); }
            const program = createAnaheimProgram(connection, wallet);
            const newAccount = web3.Keypair.fromSecretKey(new Uint8Array(anaheimAccountKey));
            const signature = await program.methods
                .initialize()
                .accounts({
                    anaheim: newAccount.publicKey,
                    payer: wallet.publicKey,
                })
                .signers([newAccount])
                .rpc();
            const latestBlockhash = await connection.getLatestBlockhash();
            await connection.confirmTransaction({ signature, ...latestBlockhash });
            return { signature, publicKey: newAccount.publicKey };
        },
        onSuccess: (data) => {
            console.log(`Initialize successful! New account: ${data.publicKey.toString()}`);
            return queryClient.invalidateQueries({ queryKey: ['anaheim-account'] });
        },
        onError: (error: Error) => console.error('Initialize failed:', error),
    });
}

// --- MAIN COMPONENT ---
export default function MiningPanel() {
    const { data: anaheimAccount, isLoading } = useAnaheimAccountQuery();
    const incrementMutation = useIncrementMutation();
    const initializeMutation = useInitializeMutation();
    const { connected } = useWallet();

    if (isLoading) {
        return <p className="text-center">Loading on-chain data...</p>;
    }

    if (!anaheimAccount) {
        return (
            <div className="p-4 border rounded-xl bg-destructive text-destructive-foreground max-w-sm mx-auto space-y-4">
                <h3 className="text-lg font-bold text-center">Program Not Initialized</h3>
                <p className="text-sm text-center">The main program account has not been created on this network yet.</p>
                <Button
                    className="w-full"
                    variant="secondary"
                    onClick={() => initializeMutation.mutate()}
                    disabled={!connected || initializeMutation.isPending}
                >
                    {!connected ? 'Connect Wallet to Initialize' : (initializeMutation.isPending ? 'Initializing...' : 'Initialize Program')}
                </Button>
            </div>
        );
    }

    const currentCount = anaheimAccount.count.toNumber();

    return (
        <div className="p-6 border rounded-xl bg-card text-card-foreground max-w-sm mx-auto">
            <h2 className="text-xl font-bold text-center">Community Counter</h2>
            <div className="text-center">
                <p className="text-muted-foreground">Current Count:</p>
                <p className="text-5xl font-bold font-mono animate-pulse">{currentCount}</p>
            </div>
            <Button
                className="w-full text-lg py-6"
                onClick={() => incrementMutation.mutate()}
                disabled={!connected || incrementMutation.isPending}
            >
                {incrementMutation.isPending ? 'Incrementing...' : 'Increment Count'}
            </Button>
        </div>
    );
}