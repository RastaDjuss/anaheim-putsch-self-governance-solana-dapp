'use client';
import { TOKEN_2022_PROGRAM_ADDRESS, TOKEN_PROGRAM_ADDRESS } from 'gill/programs/token';
import { getTransferSolInstruction } from 'gill/programs';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useWalletAccountTransactionSendingSigner, useWalletUi, useWalletUiCluster, } from '@wallet-ui/react';
import { address, airdropFactory, appendTransactionMessageInstruction, assertIsTransactionMessageWithSingleSendingSigner, createTransactionMessage, getBase58Decoder, lamports, pipe, setTransactionMessageFeePayerSigner, setTransactionMessageLifetimeUsingBlockhash, signAndSendTransactionMessageWithSigners, } from 'gill';
import { toast } from 'sonner';
import { useTransactionToast } from '../use-transaction-toast';
export function useGetBalance({ address }) {
    const { cluster } = useWalletUiCluster();
    const { client } = useWalletUi();
    return useQuery({
        queryKey: ['get-balance', { cluster, address }],
        queryFn: () => client.rpc
            .getBalance(address)
            .send()
            .then((res) => res.value),
    });
}
export function useGetSignatures({ address }) {
    const { cluster } = useWalletUiCluster();
    const { client } = useWalletUi();
    return useQuery({
        queryKey: ['get-signatures', { cluster, address }],
        queryFn: () => client.rpc.getSignaturesForAddress(address).send(),
    });
}
export function useGetTokenAccounts({ address }) {
    const { cluster } = useWalletUiCluster();
    const { client } = useWalletUi();
    return useQuery({
        queryKey: ['get-token-accounts', { cluster, address }],
        queryFn: async () => Promise.all([
            client.rpc
                .getTokenAccountsByOwner(address, { programId: TOKEN_PROGRAM_ADDRESS }, { commitment: 'confirmed', encoding: 'jsonParsed' })
                .send()
                .then((res) => res.value ?? []),
            client.rpc
                .getTokenAccountsByOwner(address, { programId: TOKEN_2022_PROGRAM_ADDRESS }, { commitment: 'confirmed', encoding: 'jsonParsed' })
                .send()
                .then((res) => res.value ?? []),
        ]).then(([tokenAccounts, token2022Accounts]) => [...tokenAccounts, ...token2022Accounts]),
    });
}
export function useTransferSol({ address, account }) {
    const { cluster } = useWalletUiCluster();
    const { client } = useWalletUi();
    const toastTransaction = useTransactionToast();
    const txSigner = useWalletAccountTransactionSendingSigner(account, cluster.id);
    const queryClient = useQueryClient();
    return useMutation({
        mutationKey: ['transfer-sol', { cluster, address }],
        mutationFn: async (input) => {
            try {
                const { signature } = await createTransaction({
                    txSigner,
                    destination: input.destination,
                    amount: input.amount,
                    client,
                });
                console.log(signature);
                return signature;
            }
            catch (error) {
                console.log('error', `Transaction failed! ${error}`);
                return;
            }
        },
        onSuccess: (signature) => {
            if (signature?.length) {
                toastTransaction(signature);
            }
            return Promise.all([
                queryClient.invalidateQueries({
                    queryKey: ['get-balance', { cluster, address }],
                }),
                queryClient.invalidateQueries({
                    queryKey: ['get-signatures', { cluster, address }],
                }),
            ]);
        },
        onError: (error) => {
            toast.error(`Transaction failed! ${error}`);
        },
    });
}
export function useRequestAirdrop({ address }) {
    const { cluster } = useWalletUiCluster();
    const { client } = useWalletUi();
    const queryClient = useQueryClient();
    const toastTransaction = useTransactionToast();
    const airdrop = airdropFactory(client);
    return useMutation({
        mutationKey: ['airdrop', { cluster, address }],
        mutationFn: async (amount = 1) => airdrop({
            commitment: 'confirmed',
            recipientAddress: address,
            lamports: lamports(BigInt(Math.round(amount * 1_000_000_000))),
        }),
        onSuccess: (signature) => {
            toastTransaction(signature);
            return Promise.all([
                queryClient.invalidateQueries({ queryKey: ['get-balance', { cluster, address }] }),
                queryClient.invalidateQueries({ queryKey: ['get-signatures', { cluster, address }] }),
            ]);
        },
    });
}
async function createTransaction({ amount, destination, client, txSigner, }) {
    const { value: latestBlockhash } = await client.rpc.getLatestBlockhash({ commitment: 'confirmed' }).send();
    const message = pipe(createTransactionMessage({ version: 0 }), (m) => setTransactionMessageFeePayerSigner(txSigner, m), (m) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, m), (m) => appendTransactionMessageInstruction(getTransferSolInstruction({
        amount,
        destination: address(destination),
        source: txSigner,
    }), m));
    assertIsTransactionMessageWithSingleSendingSigner(message);
    const signature = await signAndSendTransactionMessageWithSigners(message);
    return {
        signature: getBase58Decoder().decode(signature),
        latestBlockhash,
    };
}
