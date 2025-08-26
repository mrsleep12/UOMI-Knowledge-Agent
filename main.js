const { startAgent } = require("wasp");
const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

startAgent("agent/target/wasm32-unknown-unknown/release/agent.wasm", async (input) => {
  const askQuestion = () => {
    rl.question("Ask your question (or in Bahasa): ", async (question) => {
      const output = await input({ question });
      console.log("Agent Response:", output.response);
      console.log("History:", output.history.join(" | "));
      askQuestion();
    });
  };
  askQuestion();
});
