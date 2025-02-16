import { pipe } from "@screenpipe/browser";
import { useEffect, useState } from "react";

export function useLastMinutes(minutes: number) {
	// Always return an array so array methods like filter are available.
	const [frames, setFrames] = useState<any[]>([]);

	useEffect(() => {
		async function fetchFrames() {
			const now = new Date();
			const lastMinutes = new Date(now.getTime() - minutes * 60 * 1000);
			const frame = await pipe.queryScreenpipe({
				contentType: "all",
				startTime: lastMinutes.toISOString(),
				endTime: now.toISOString(),
			});
			if (frame) {
				// Ensure frames is an array; fallback to an empty array if frame is not an array.
				const framesArray = Array.isArray(frame) ? frame : [frame];
				setFrames(framesArray);
			}
		}

		// Initial fetch
		fetchFrames();

		// Poll every 5000ms (5 seconds)
		const interval = setInterval(fetchFrames, 5000);
		return () => clearInterval(interval);
	}, [minutes]);

	return frames;
}
