import { useState } from "react";
import { Channel, invoke } from "@tauri-apps/api/core";
import "./App.css";

// The interrogation room, for now: call Viktor in, ask him something,
// and watch his reply arrive piece by piece.
function App() {
  const [question, setQuestion] = useState("Where were you on Tuesday?");
  const [reply, setReply] = useState("");
  const [problem, setProblem] = useState("");

  async function callViktorIn() {
    setProblem("");
    try {
      await invoke("begin_interrogation", { slug: "the-ledger", suspect: 2 });
    } catch (error) {
      setProblem(JSON.stringify(error));
    }
  }

  async function ask() {
    setReply("");
    setProblem("");
    try {
      const onPiece = new Channel<string>();
      onPiece.onmessage = (piece) => {
        console.log("piece:", piece);
        setReply((before) => before + piece);
      };
      await invoke("ask_suspect", { question, onPiece });
    } catch (error) {
      setReply("");
      setProblem(JSON.stringify(error));
    }
  }

  // The player presses Stop: the suspect stops mid-sentence.
  async function stop() {
    setProblem("");
    try {
      await invoke("stop_suspect");
    } catch (error) {
      setProblem(JSON.stringify(error));
    }
  }

  return (
    <main className="container">
      <h1>Interrogation room</h1>

      <div className="row">
        <button onClick={callViktorIn}>Call Viktor in</button>
      </div>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          ask();
        }}
      >
        <input
          value={question}
          onChange={(e) => setQuestion(e.currentTarget.value)}
        />
        <button type="submit">Ask</button>
      </form>

      <div className="row">
        <button onClick={stop}>Stop</button>
      </div>

      <p>{reply}</p>
      <p>{problem}</p>
    </main>
  );
}

export default App;
