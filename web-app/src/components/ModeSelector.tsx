type Mode = "encode" | "decode";

interface ModeSelectorProps {
  mode: Mode;
  onModeChange: (mode: Mode) => void;
}

const ModeSelector = ({ mode, onModeChange }: ModeSelectorProps) => {
  return (
    <div className="grid grid-cols-2 border border-[#aaa8ae] bg-[#e7e6e2] p-1" role="tablist" aria-label="Steganography mode">
      <button
        onClick={() => onModeChange("encode")}
        role="tab"
        aria-selected={mode === "encode"}
        className={`border px-4 py-3 font-technical text-xs font-semibold uppercase transition-all ${
          mode === "encode"
            ? "border-[#3c3b40] bg-[#fdfdfb] text-[#202126]"
            : "border-transparent text-[#77767d] hover:text-[#202126]"
        }`}
      >
        Hide a message
      </button>
      <button
        onClick={() => onModeChange("decode")}
        role="tab"
        aria-selected={mode === "decode"}
        className={`border px-4 py-3 font-technical text-xs font-semibold uppercase transition-all ${
          mode === "decode"
            ? "border-[#3c3b40] bg-[#fdfdfb] text-[#202126]"
            : "border-transparent text-[#77767d] hover:text-[#202126]"
        }`}
      >
        Reveal a message
      </button>
    </div>
  );
};

export default ModeSelector;
