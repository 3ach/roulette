import type { Route } from "./+types/home";
import { Roulette } from "../components/Roulette";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Roulette" },
    { name: "description", content: "Can you tell which LLM is which?" },
  ];
}

export default function Home() {
  return <Roulette />;
}
