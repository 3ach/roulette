import { useState } from 'react';

export function LLMResponse({content}: {content: string}) {
  return (
    <nav className="rounded-lg border border-gray-200 p-3 dark:border-gray-700 space-y-4">
      <p>{content}</p>
    </nav>
  );
}
