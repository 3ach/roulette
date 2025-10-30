import type { Route } from "./+types/home";
import { Prompt } from "../prompt/prompt";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Roulette" },
    { name: "description", content: "Can you tell which LLM is which?" },
  ];
}

export default function Home() {
  return <Prompt />;
}
