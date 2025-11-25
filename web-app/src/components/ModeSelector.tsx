import { FaLock, FaUnlock } from "react-icons/fa";

type Mode = "encode" | "decode";

interface ModeSelectorProps {
  mode: Mode;
  onModeChange: (mode: Mode) => void;
}

const ModeSelector = ({ mode, onModeChange }: ModeSelectorProps) => {
  return (
    <div className="flex gap-4 mb-8">
      <button
        onClick={() => onModeChange("encode")}
        className={`flex-1 py-4 px-6 rounded-lg font-bold transition-all font-mono border-2 ${
          mode === "encode"
            ? "bg-[#00ff88]/20 text-[#00ff88] border-[#00ff88] shadow-lg border-glow"
            : "bg-[#0f1419] text-gray-400 border-gray-700 hover:border-[#00ff88]/50"
        }`}
      >
        <FaLock className="inline mr-2" />
        [ENCODE]
      </button>
      <button
        onClick={() => onModeChange("decode")}
        className={`flex-1 py-4 px-6 rounded-lg font-bold transition-all font-mono border-2 ${
          mode === "decode"
            ? "bg-[#00d4ff]/20 text-[#00d4ff] border-[#00d4ff] shadow-lg border-glow"
            : "bg-[#0f1419] text-gray-400 border-gray-700 hover:border-[#00d4ff]/50"
        }`}
      >
        <FaUnlock className="inline mr-2" />
        [DECODE]
      </button>
    </div>
  );
};

export default ModeSelector;

