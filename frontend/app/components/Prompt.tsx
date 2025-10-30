import { useState } from 'react';

export function Prompt({setPrompt, prompt, dispatch, enabled}: {setPrompt: any, dispatch: any, prompt: string | null, enabled: boolean}) {
  function updatePrompt(e: React.ChangeEvent<HTMLTextAreaElement>) {
    setPrompt(e.target.value);
  }

  function doDispatch(e: React.MouseEvent<HTMLButtonElement>) {
    e.preventDefault();
    dispatch();
  }

  let button = enabled ? (
        <button onClick={doDispatch} className="inline-flex justify-center p-2 text-blue-600 rounded-full cursor-pointer hover:bg-blue-100 dark:text-blue-500 dark:hover:bg-gray-600">
          <svg className="w-5 h-5 rotate-90 rtl:-rotate-90 stroke-gray-300 dark:stroke-gray-700" strokeWidth="1.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 18 20">
            <path d="m17.914 18.594-8-18a1 1 0 0 0-1.828 0l-8 18a1 1 0 0 0 1.157 1.376L8 18.281V9a1 1 0 0 1 2 0v9.281l6.758 1.689a1 1 0 0 0 1.156-1.376Z" />
          </svg>
          <span className="sr-only">Send message</span>
        </button>
  ) : null;

  return (
    <form>
      <label className="sr-only">Your message</label>
      <div className="flex items-center rounded-lg border-gray-200 dark:border-gray-700">
        <textarea id="chat" disabled={!enabled} onChange={updatePrompt} rows={1} value={prompt ?? undefined} className="block p-2.5 w-full text-sm text-gray-900 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 resize-none" placeholder="Consult the machine god..."></textarea>
        { button }
      </div>
    </form>
  );
}
