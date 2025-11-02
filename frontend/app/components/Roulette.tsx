import { useState, useEffect } from 'react';
import { LLMResponse } from './LLMResponse';
import { Prompt } from './Prompt';
import { SSE } from 'sse.js';

const DEV = true;
const DOMAIN = DEV ? "http://localhost:3000" : "https://roulette-server.zach.network";

export function Roulette() {
  let [prompt, setPrompt] = useState(null);
  let [writable, setWritable] = useState(true);
  let [response, setResponse] = useState<string | null>(null);
  let [requestId, setRequestId] = useState<string | null>(null);

  function dispatch() {
    setWritable(false);
  }

  useEffect(() => {
    if (writable) {
      return;
    }
   
    let events = new SSE(`${DOMAIN}/prompt`, {
      payload: prompt
    });

    let chunks: string[] = [];
    let rendered = "";

    let clear = setInterval(() => {
      if (chunks.length == 0) {
        if (events.readyState == SSE.CLOSED) {
          clearTimeout(clear);
        }

        return;
      }

      rendered += chunks.shift();
      setResponse(rendered);
    }, 10);

    events.addEventListener("message", (message: any) => {
      chunks = chunks.concat(message.data.split(""));
    })

    events.addEventListener("open", (event: any) => {
      console.log(event.headers);
      let serverRequestId = event.headers["x-roulette-request"][0];
      if (serverRequestId != undefined && serverRequestId != null) {
        setRequestId(serverRequestId);
      }
    })

  }, [prompt, writable])

  return (
    <main className="flex items-center justify-center pt-16 pb-4">
      <div className="flex-1 flex flex-col items-center min-h-0">
        <header className="flex flex-col items-center gap-9">
          <div className="w-[500px] max-w-[100vw] p-4">
            <h1 className="mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl dark:text-white text-center">Roulette</h1>
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
