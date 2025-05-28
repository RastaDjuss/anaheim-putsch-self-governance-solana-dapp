"use client";

import React, { useState, useEffect, useMemo } from 'react';
import formatDistance from 'date-fns/formatDistance';
import formatDistanceToNowStrict from 'date-fns/formatDistanceToNowStrict';
import add from 'date-fns/add';
import { fr } from 'date-fns/locale';

const LAST_CLAIM_KEY = 'last-claim-timestamp';
const ONE_DAY_MS = 24 * 60 * 60 * 1000;

// Vérifie si une date est valide
function isValidDate(date: unknown): date is Date {
	return date instanceof Date && !isNaN(date.getTime());
}

// Composant principal
export default function MiningPanel() {
	const [lastClaim, setLastClaim] = useState<Date | null>(null);
	const [isInitialized, setIsInitialized] = useState(false);
	const [now, setNow] = useState(() => Date.now());
	const [canClaim, setCanClaim] = useState(false);

	// Met à jour l'heure actuelle toutes les secondes
	useEffect(() => {
		if (typeof window === 'undefined') return; // Vérifie l'exécution côté client

		const interval = setInterval(() => setNow(Date.now()), 1000);

		try {
			const stored = localStorage.getItem(LAST_CLAIM_KEY);
			if (stored) {
				const date = new Date(stored);
				if (isValidDate(date)) {
					setLastClaim(date);
				} else {
					localStorage.removeItem(LAST_CLAIM_KEY); // Nettoyage si la date est invalide
				}
			}
		} catch (e) {
			console.error('Erreur lors du chargement de localStorage', e);
		} finally {
			setIsInitialized(true);
		}

		return () => clearInterval(interval); // Nettoyage de l'intervalle
	}, []);

	// Détermine si une réclamation est possible (au moins 1 jour écoulé)
	useEffect(() => {
		if (!lastClaim) {
			setCanClaim(true);
		} else {
			setCanClaim(now - lastClaim.getTime() > ONE_DAY_MS);
		}
	}, [lastClaim, now]);

	// Sauvegarde de la dernière date de minage dans localStorage
	useEffect(() => {
		if (lastClaim) {
			localStorage.setItem(LAST_CLAIM_KEY, lastClaim.toISOString());
		}
	}, [lastClaim]);


	const displayText = useMemo(() => {
		if (!isInitialized) return 'Chargement des informations...';
		if (!lastClaim) return 'Pas encore de minage disponible';
		return `Dernier minage : ${formatDistanceToNowStrict(lastClaim, { locale: fr })}`;
	}, [isInitialized, lastClaim]);

	const nextClaimIn = useMemo(() => {
		if (!lastClaim || canClaim) return null;

		try {
			const nextClaimDate = add(lastClaim, { days: 1 });
			return formatDistance(new Date(), nextClaimDate, { addSuffix: true, locale: fr });
		} catch (error) {
			console.error('Erreur lors du calcul de la prochaine réclamation', error);
			return null;
		}
	}, [lastClaim, canClaim]);

	// Progression de la barre (0-100%)
	const progress = useMemo(() => {
		if (!lastClaim) return 100;
		const elapsed = now - lastClaim.getTime();
		return Math.min(100, Math.round((elapsed / ONE_DAY_MS) * 100));
	}, [lastClaim, Math.floor(now / 1000)]); // Actualisé toutes les secondes

	// Gère le clic du bouton "Réclamer"
	const handleClaim = () => {
		if (canClaim) {
			setLastClaim(new Date());
		}
	};

	// Rendu du composant
	return (
		<div
			className="p-4 border rounded-xl bg-gray-900 text-white space-y-3 max-w-sm mx-auto"
			aria-live="polite"
			role="status"
		>
			<h2 className="text-xl font-bold">Panneau de Minage</h2>

			<p className={`text-red-500 font-bold ${canClaim ? 'animate-pulse' : ''}`}>
				{displayText}
			</p>

			{!canClaim && nextClaimIn ? (
				<p className="text-sm text-gray-400">Prochaine réclamation : {nextClaimIn}</p>
			) : null}
			<div className="h-2 bg-gray-700 rounded overflow-hidden">
				<div
					className="h-full bg-green-500 transition-all duration-500 ease-in-out"
					style={{ width: `${progress}%` }}
				/>
			</div>

			<button
				className={`w-full px-4 py-2 rounded font-semibold transition-all ${
					canClaim ? 'bg-green-600 hover:bg-green-700' : 'bg-gray-600 cursor-not-allowed'
				}`}
				onClick={handleClaim}
				disabled={!canClaim}
				aria-disabled={!canClaim}
				aria-label={canClaim ? 'Réclamer une récompense' : 'Réclamation indisponible, réessayez plus tard'}
			>
				{canClaim ? 'Réclamer' : 'Reviens plus tard'}
			</button>
		</div>
	);
}