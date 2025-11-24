import { useEffect, useRef, useState, useCallback } from 'react'
import * as sim from "lib-simulation-wasm";
import { LineChart, Line, YAxis, ResponsiveContainer, Tooltip } from 'recharts';
import { Play, FastForward, Pause, AlertTriangle, RefreshCw, Timer } from 'lucide-react';

// ----------------- Constants & Types -----------------------
const GENERATION_LENGTH = 2500;
const MAX_GENERATIONS = 100;
const HISTORY_CHECK_LENGTH = 4; // Stop if last 4 avgs are same

interface AgentJs {
    x: number;
    y: number;
    rotation: number;
}

interface InputJs {
    x: number;
    y: number;
}

interface GenStats {
    generation: number;
    min: number;
    max: number;
    avg: number;
}

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
    // Correcting rotation: Math says 0 is Right, Canvas Up is -Y.
    // + PI/2 rotates the "Up" nose to point "Right"
    ctx.rotate(agent.rotation + Math.PI / 2);

    ctx.beginPath();
    ctx.moveTo(0, -size);
    ctx.lineTo(size, size);
    ctx.lineTo(0, size * 0.5);
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
    ctx.fillStyle = '#ef4444'; // Red food
    ctx.fill();
}

// --------------------- Helper Functions -------------------------
// Parses "min=0.00, max=0.00, avg=0.00"
function parseStats(statsStr: string, genIndex: number): GenStats {
    const regex = /min=([\d.]+), max=([\d.]+), avg=([\d.]+)/;
    const match = statsStr.match(regex);
    if (!match) return { generation: genIndex, min: 0, max: 0, avg: 0 };
    return {
        generation: genIndex,
        min: parseFloat(match[1]),
        max: parseFloat(match[2]),
        avg: parseFloat(match[3])
    };
}

function formatTime(seconds: number) {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
}

// --------------------- Main Component  -------------------------
function App() {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const [simulation] = useState(() => new sim.Simulation());
    
    // Simulation State
    const [genCount, setGenCount] = useState(1);
    const [stepCount, setStepCount] = useState(0);
    const [statsHistory, setStatsHistory] = useState<GenStats[]>([]);
    const [isRunning, setIsRunning] = useState(true);
    const [elapsedTime, setElapsedTime] = useState(0);
    const [simSpeed, setSimSpeed] = useState(1); // Steps per frame

    // Stop Condition State
    const [isFinished, setIsFinished] = useState(false);
    const [finishReason, setFinishReason] = useState("");

    // Refs for animation loop to access latest state without re-rendering
    const stepCountRef = useRef(0);
    const isRunningRef = useRef(true);
    const isFinishedRef = useRef(false);

    // ------------------- Logic -----------------------

    // Handle "Fast Forward" (Complete current gen immediately)
    const handleFastForward = useCallback(() => {
        if (isFinishedRef.current) return;
        
        // This Rust function loops until evolution happens
        const statsStr = simulation.complete_one_evolution();
        const newStats = parseStats(statsStr, genCount);
        
        // Update State
        setStatsHistory(prev => [...prev, newStats]);
        setGenCount(prev => prev + 1);
        setStepCount(0);
        stepCountRef.current = 0;

        checkStopConditions(newStats, statsHistory);
    }, [simulation, genCount, statsHistory]);

    // Check if we should stop simulation
    const checkStopConditions = (current: GenStats, history: GenStats[]) => {
        // 1. Limit reached
        if (current.generation >= MAX_GENERATIONS) {
            setIsFinished(true);
            isFinishedRef.current = true;
            setIsRunning(false);
            isRunningRef.current = false;
            setFinishReason("Max generations reached");
            return;
        }

        // 2. Stagnation (last 4 avgs are identical)
        // We use a small epsilon for float comparison, or string comparison if exact
        if (history.length >= HISTORY_CHECK_LENGTH - 1) {
            const lastFew = [...history.slice(-(HISTORY_CHECK_LENGTH - 1)), current];
            const allSame = lastFew.every(s => Math.abs(s.avg - current.avg) < 0.0001);
            
            if (allSame) {
                setIsFinished(true);
                isFinishedRef.current = true;
                setIsRunning(false);
                isRunningRef.current = false;
                setFinishReason("Evolution stagnated (Fitness converged)");
            }
        }
    };

    // ------------------- Effects -----------------------

    // Timer
    useEffect(() => {
        const timer = setInterval(() => {
            if (isRunning && !isFinished) {
                setElapsedTime(t => t + 1);
            }
        }, 1000);
        return () => clearInterval(timer);
    }, [isRunning, isFinished]);

    // Refresh Warning
    useEffect(() => {
        const handleBeforeUnload = (e: BeforeUnloadEvent) => {
            e.preventDefault();
            e.returnValue = ''; // Chrome requires returnValue to be set
        };
        window.addEventListener('beforeunload', handleBeforeUnload);
        return () => window.removeEventListener('beforeunload', handleBeforeUnload);
    }, []);

    // Canvas & Simulation Loop
    useEffect(() => {
        const canvas = canvasRef.current;
        if (!canvas) return;
        const ctx = canvas.getContext('2d');
        if (!ctx) return;

        // Resize Logic
        const resizeCanvas = () => {
            const pixelRatio = window.devicePixelRatio || 1;
            const width = window.innerWidth;
            const height = window.innerHeight;
            canvas.width = width * pixelRatio;
            canvas.height = height * pixelRatio;
            canvas.style.width = `${width}px`;
            canvas.style.height = `${height}px`;
            ctx.scale(pixelRatio, pixelRatio);
        };
        resizeCanvas();
        window.addEventListener('resize', resizeCanvas);

        let animationId: number;

        const render = () => {
            // 1. UPDATE PHYSICS
            if (isRunningRef.current && !isFinishedRef.current) {
                // We step multiple times per frame to speed it up
                // BUT we stop right before the generation ends to handle stats
                for (let i = 0; i < simSpeed; i++) {
                    
                    // If we are at the end of a generation...
                    if (stepCountRef.current >= GENERATION_LENGTH) {
                        // ...force evolution via the specific Rust method that returns stats
                        // Because 'step()' implies void return in the binding
                        const statsStr = simulation.complete_one_evolution();
                        parseStats(statsStr, genCount); // Uses closure genCount (might be laggy visually but logic holds)
                        
                        // We need to break the loop and trigger a React state update
                        // We can't update state inside this loop easily without refs, 
                        // so we emit an event or just force it next frame.
                        // Ideally:
                        handleFastForward(); // Re-uses the logic
                        break; 
                    } else {
                        simulation.step();
                        stepCountRef.current++;
                    }
                }
                // Sync ref to state for UI (progress bar)
                setStepCount(stepCountRef.current);
            }

            // 2. DRAW
            const world = simulation.world();
            const width = window.innerWidth;
            const height = window.innerHeight;

            ctx.fillStyle = '#F5F5F5';
            ctx.fillRect(0, 0, width, height);

            for (const input of world.inputs as unknown as InputJs[]) {
                drawInput(ctx, width, height, input);
            }

            for (const agent of world.agents as unknown as AgentJs[]) {
                drawAgent(ctx, width, height, agent);
            }

            animationId = requestAnimationFrame(render);
        };

        render();

        return () => {
            window.removeEventListener('resize', resizeCanvas);
            cancelAnimationFrame(animationId);
        };
    }, [simulation, simSpeed, handleFastForward]); // Dependencies trigger restart of loop if changed

    // Toggle Play/Pause
    const togglePlay = () => {
        if (isFinished) return;
        setIsRunning(!isRunning);
        isRunningRef.current = !isRunningRef.current;
    };

    const latestStats = statsHistory[statsHistory.length - 1] || { min: 0, max: 0, avg: 0 };
    const progressPercent = Math.min(100, (stepCount / GENERATION_LENGTH) * 100);

    return (
        <div className="relative w-screen h-screen overflow-hidden bg-black font-sans text-slate-800">
            {/* Background Canvas */}
            <canvas ref={canvasRef} className="block w-full h-full" />

            {/* Dashboard Overlay */}
            <div className="absolute top-4 left-4 w-[calc(100vw-2rem)] md:w-96 backdrop-blur-xl rounded-2xl shadow-xl border border-red/20 p-6 flex flex-col gap-5">
                {/* Header & Timer */}
                <div className="flex justify-between items-start">
                    <div>
                        <h1 className="text-l font-thin text-slate-900 tracking-tight">Evolve Sim</h1>
                        <div className="flex items-center gap-2 text-slate-500 text-sm mt-1">
                            <Timer size={14} />
                            <span className="font-mono">{formatTime(elapsedTime)}</span>
                            <span>•</span>
                            <span>Gen {genCount}</span>
                        </div>
                    </div>
                    {isFinished && (
                        <div className="bg-red-100 text-red-700 px-3 py-1 rounded-full text-xs font-bold border border-red-200 flex items-center gap-1">
                            <AlertTriangle size={12}/> DONE
                        </div>
                    )}
                </div>

                {/* Progress to Next Gen */}
                <div>
                    <div className="flex justify-between text-xs font-semibold text-slate-500 mb-1.5">
                        <span>Evolution Progress</span>
                        <span>{Math.floor(progressPercent)}%</span>
                    </div>
                    <div className="h-2 w-full bg-slate-300 rounded-full overflow-hidden">
                        <div 
                            className="h-full bg-indigo-500 transition-all duration-100 ease-linear"
                            style={{ width: `${progressPercent}%` }}
                        />
                    </div>
                </div>

                {/* Fitness Graph */}
                <div className="h-32 w-full -ml-2 select-none [&_.recharts-wrapper]:!outline-none">
                    <ResponsiveContainer width="100%" height="100%">
                        <LineChart data={statsHistory}>
                            <YAxis domain={[0, 'auto']} hide />
                            <Tooltip 
                                labelStyle={{ color: 'black' }}
                                contentStyle={{ borderRadius: '8px', border: 'none', boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.1)' }} 
                            />
                            <Line 
                                type="monotone" 
                                dataKey="avg" 
                                stroke="#6366f1" 
                                strokeWidth={2} 
                                dot={false} 
                                isAnimationActive={false} // Performance
                            />
                            <Line type="monotone" dataKey="max" stroke="#10b981" strokeWidth={1} strokeDasharray="3 3" dot={false} isAnimationActive={false} />
                        </LineChart>
                    </ResponsiveContainer>
                    <div className="flex justify-between px-2 text-[10px] text-slate-400 -mt-2">
                        <span>Start</span>
                        <span>History</span>
                    </div>
                </div>

                {/* Statistics Grid */}
                <div className="grid grid-cols-3 gap-2">
                    <div className="p-2 rounded-lg text-center border-2 border-slate-500/50">
                        <div className="text-[10px] uppercase font-bold text-slate-400">Min</div>
                        <div className="font-mono text-lg leading-tight text-slate-700">
                            {latestStats.min.toFixed(2)}
                        </div>
                    </div>
                    <div className="p-2 rounded-lg text-center border-2 border-indigo-500/50">
                        <div className="text-[10px] uppercase font-bold text-indigo-400">Avg</div>
                        <div className="font-mono text-lg leading-tight text-indigo-600 font-bold">
                            {latestStats.avg.toFixed(2)}
                        </div>
                    </div>
                    <div className="p-2 rounded-lg text-center border-2 border-green-500/50">
                        <div className="text-[10px] uppercase font-bold text-green-500">Max</div>
                        <div className="font-mono text-lg leading-tight text-green-600">
                            {latestStats.max.toFixed(2)}
                        </div>
                    </div>
                </div>

                {/* Finish Reason (if any) */}
                {finishReason && (
                    <div className="text-xs text-center text-red-500 font-medium bg-red-50 p-2 rounded border border-red-100">
                        {finishReason}
                    </div>
                )}

                {/* Controls */}
                <div className="flex items-center gap-3 mt-1">
                    <button 
                        onClick={togglePlay}
                        disabled={isFinished}
                        className={`flex-1 flex items-center justify-center gap-2 py-2.5 rounded-xl font-bold text-sm transition-colors
                            ${isRunning 
                                ? 'bg-amber-100 text-amber-500 hover:bg-amber-200' 
                                : 'bg-green-100 text-green-500 hover:bg-green-200'
                            } disabled:opacity-50 disabled:cursor-not-allowed`}
                    >
                        {isRunning ? <><Pause size={16}/> Pause</> : <><Play size={16}/> Resume</>}
                    </button>

                    <button 
                        onClick={handleFastForward}
                        disabled={isFinished}
                        className="flex-1 flex items-center justify-center gap-2 py-2.5 bg-indigo-600 hover:bg-indigo-700 text-white rounded-xl font-bold text-sm transition-colors shadow-md disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        <FastForward size={16} /> Next Gen
                    </button>
                </div>
                
                {/* Speed Slider */}
                <div className='flex items-center gap-3 text-xs text-slate-500'>
                    <span>Speed:</span>
                    <input 
                        type="range" 
                        min="1" 
                        max="20" 
                        value={simSpeed} 
                        onChange={(e) => setSimSpeed(parseInt(e.target.value))}
                        className="flex-1 h-1.5 bg-slate-300 rounded-lg appearance-none cursor-pointer accent-indigo-600"
                    />
                    <span className='w-4'>{simSpeed}x</span>
                </div>

                <div className="text-[10px] text-center text-slate-500 flex items-center justify-center gap-1">
                     <RefreshCw size={8} /> Refreshing will reset progress
                </div>
            </div>
        </div>
    )
}

export default App