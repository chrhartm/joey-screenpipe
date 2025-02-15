import { useState } from "react";
import logo from "./assets/logo.png";
import { invoke } from "@tauri-apps/api/core";
import { useVision } from "../hooks/useVision";
import "./App.css";

function App() {
	const [greetMsg, setGreetMsg] = useState("");
	const [name, setName] = useState("");
	const { text } = useVision();

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
		setGreetMsg(await invoke("greet", { name }));
	}

	return (
		<main className="container">
			<h1>Joey</h1>

			<div>
				<p>First 50 characters detected on screen:</p>
				<p>{text.substring(0, 50)}</p>
			</div>

			<div className="row">
				<a href="https://screenpi.pe/" target="_blank">
					<img
						src={logo}
						className="logo"
						alt="Logo"
					/>
				</a>
			</div>
			<p>
				Click Joey to stop the screen recording.
			</p>

			<form
				className="row"
				onSubmit={(e) => {
					e.preventDefault();
					greet();
				}}
			>
				<input
					id="greet-input"
					onChange={(e) => setName(e.currentTarget.value)}
					placeholder="Enter a name..."
				/>
				<button type="submit">Greet</button>
			</form>
			<p>{greetMsg}</p>
		</main>
	);
}

export default App;
