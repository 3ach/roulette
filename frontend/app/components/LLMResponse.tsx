import Markdown from 'react-markdown';

export function LLMResponse({content}: {content: string}) {
  return (
    <nav className="rounded-lg border border-gray-200 p-3 dark:border-gray-700 space-y-4">
      <Markdown>{content}</Markdown>
    </nav>
  );
}
