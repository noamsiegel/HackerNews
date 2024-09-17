import OpenAI from "openai";
import { SYSTEM_PROMPTS } from "./system_prompts";

const openai = new OpenAI({
  apiKey: process.env.OPENAI_API_KEY,
});

async function streamChatCompletion(
  userPrompt: string,
  systemPrompt: keyof typeof SYSTEM_PROMPTS = "SUMMARIZER"
) {
  const stream = await openai.chat.completions.create({
    model: "gpt-4",
    messages: [
      { role: "system", content: SYSTEM_PROMPTS[systemPrompt] },
      { role: "user", content: userPrompt },
    ],
    stream: true,
  });

  for await (const chunk of stream) {
    process.stdout.write(chunk.choices[0]?.delta?.content || "");
  }
}

// Example usage
const userPrompt = "Tell me a short story about a robot learning to paint.";
streamChatCompletion(userPrompt, "PODCASTER");
