import { useState, useEffect } from 'react';
import { LLMResponse } from './LLMResponse';
import { Prompt } from './Prompt';
import { SSE } from 'sse.js';

export function Roulette() {
  let [prompt, setPrompt] = useState(null);
  let [writable, setWritable] = useState(true);
  let [response, setResponse] = useState<string | null>(null);

  function dispatch() {
    setWritable(false);
  }

  useEffect(() => {
    if (writable) {
      return;
    }
   
    /*
    let events = new SSE("http://localhost:3000/prompt", {
      payload: prompt
    });
    */

    let chunks = ["It's time f", "or you to", " go to the ", "movies"];
    chunks = chunks.flatMap(chunk => chunk.split(""));
    let rendered = "";

    let clear = setInterval(() => {
      if (chunks.length == 0) {
        return;
      }

      rendered += chunks.shift();

      setResponse(rendered);
    }, 20);

  }, [prompt, writable])

  return (
    <main className="flex items-center justify-center pt-16 pb-4">
      <div className="flex-1 flex flex-col items-center gap-16 min-h-0">
        <header className="flex flex-col items-center gap-9">
          <div className="w-[500px] max-w-[100vw] p-4">
            <p className="text-gray-700 dark:text-gray-200 text-center">Roulette</p>
          </div>
        </header>
        <div className="max-w-[900px] w-full space-y-6 px-4">
            <Prompt
              setPrompt={setPrompt}
              prompt={prompt}
              enabled={writable}
              dispatch={dispatch}
            />
            { (response != null)
             ? <LLMResponse content={response}  />
             : null }
        </div>
      </div>
    </main>
  );
}
