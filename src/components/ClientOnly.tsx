// FILE: src/components/ClientOnly.tsx
'use client';

import React, { useState, useEffect, ReactNode } from 'react';

/**
 * This component ensures that its children are only ever rendered on the client-side.
 * It prevents hydration mismatch errors by returning null during the server render
 * and the initial client render, then re-rendering with the children once mounted.
 */
export function ClientOnly({ children }: { children: ReactNode }) {
  const [hasMounted, setHasMounted] = useState(false);

  useEffect(() => {
    setHasMounted(true);
  }, []);

  if (!hasMounted) {
    return null;
  }

  return <>{children}</>;
}