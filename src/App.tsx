import logo from "./assets/logo.png";
import { useLastMinutes } from "../hooks/screenpipe";
import "./App.css";

function App() {
	const frames = useLastMinutes(50);
	
	// Filter raw JSON assuming each frame contains a "data" array
	// and the first element has a "type" field.
	const OCR = (frames || []).filter(
		(frame: any) => frame.data && frame.data[0] && frame.data[0].type === "OCR"
	);
	
	return (
		<main className="container">
			<div className="row">
				<a href="https://screenpi.pe/" target="_blank">
					<img
						src={logo}
						className="logo"
						alt="Logo"
					/>
				</a>
			</div>

			<div>
				<h3>OCR Frames</h3>
				{OCR.length > 0 ? (
					OCR.map((frame: any, index: number) => (
						<div key={index}>
							<p>{frame.data[0].content.text}</p>
						</div>
					))
				) : (
					<p>No OCR frames available</p>
				)}
				<h3>All Frames (raw):</h3>
				<pre>{JSON.stringify(frames, null, 2)}</pre>
			</div>
		</main>
	);
}

export default App;
