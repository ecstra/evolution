import { useEffect, useRef, useState } from 'react'
import * as sim from "lib-simulation-wasm";

// ----------------- Interface Definitions -----------------------
interface AgentJs {
    x: number;          // 0.0 <-> 1.1
    y: number;          // 0.0 <-> 1.1
    rotation: number;   // Radians
}

interface InputJs {
    x: number;          // 0.0 <-> 1.1
    y: number;          // 0.0 <-> 1.1
}
// ---------------------------------------------------------------


// ---------------------- Drawing --------------------------------
function drawAgent(
    ctx: CanvasRenderingContext2D,
    width: number,
    height: number,
    agent: AgentJs
) {
    const size = 12;
    const padding = 20;
    const x = padding + agent.x * (width - padding * 2);
    const y = padding + agent.y * (height - padding * 2);

    ctx.save();
    ctx.translate(x, y);
    ctx.rotate(agent.rotation);

    ctx.beginPath();
    // Nose (pointing up)
    ctx.moveTo(0, -size);
    // Bottom Right
    ctx.lineTo(size, size);
    // Rear center (indentation)
    ctx.lineTo(0, size * 0.5);
    // Bottom Left
    ctx.lineTo(-size, size);
    ctx.closePath();

    ctx.strokeStyle = '#406661';
    ctx.lineWidth = 2;
    ctx.stroke();
    ctx.restore();
}

function drawInput(
    ctx: CanvasRenderingContext2D,
    width: number,
    height: number,
    input: InputJs
) {
    const size = 4;
    const padding = 20;
    const x = padding + input.x * (width - padding * 2);
    const y = padding + input.y * (height - padding * 2);

    ctx.beginPath();
    ctx.arc(x, y, size, 0, Math.PI * 2);
    ctx.fillStyle = '#406661';
    ctx.fill();
}
// ---------------------------------------------------------------


// --------------------- Main Component  -------------------------
function App() {
    const canvasRef = useRef<HTMLCanvasElement>(null);

    // Initialize simulation ONCE.
    const [simulation] = useState(() => new sim.Simulation());

    useEffect(() => {
        const canvas = canvasRef.current;
        if (!canvas) return;

        const ctx = canvas.getContext('2d');
        if (!ctx) return;

        const resizeCanvas = () => {
            // Make canvas full screen
            // And ensure it's crisp on HiDPI screens
            const pixelRatio = window.devicePixelRatio || 1;
            const width = window.innerWidth;
            const height = window.innerHeight;

            canvas.width = width * pixelRatio;
            canvas.height = height * pixelRatio;

            canvas.style.width = `${width}px`;
            canvas.style.height = `${height}px`;

            ctx.scale(pixelRatio, pixelRatio);
            ctx.imageSmoothingEnabled = true;
            ctx.imageSmoothingQuality = 'high';
        };

        // Initial resize
        resizeCanvas();

        // Resize listener
        window.addEventListener('resize', resizeCanvas);

        let animationFrameId: number;

        // Animate the canvas
        const render = () => {
            // Small step for agent, huge step for Agent Kind!
            simulation.step()

            // Get the latest world state from Rust
            const world = simulation.world();
            const agents = world.agents;
            const inputs = world.inputs;

            // Clear canvas
            const width = window.innerWidth;
            const height = window.innerHeight;

            ctx.fillStyle = '#F5F5F5'; // Light background
            ctx.fillRect(0, 0, width, height);

            // Draw inputs
            for (const input of inputs as unknown as InputJs[]) {
                drawInput(ctx, width, height, input);
            }

            // Draw agents
            for (const agent of agents as unknown as AgentJs[]) {
                drawAgent(ctx, width, height, agent);
            }

            // Loop
            animationFrameId = requestAnimationFrame(render);
        };

        // Start rendering
        render();

        return () => {
            window.removeEventListener('resize', resizeCanvas);
            cancelAnimationFrame(animationFrameId);
        };
    }, [simulation]);

    return (
        <canvas
            ref={canvasRef}
            className='h-screen w-screen block bg-black'
        />
    )
}
// ---------------------------------------------------------------


export default App