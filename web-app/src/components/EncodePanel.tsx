interface EncodePanelProps {
  message: string;
  capacity: number | null;
  loading: boolean;
  wasmReady: boolean;
  onMessageChange: (message: string) => void;
  onEncode: () => void;
}

const EncodePanel = ({
  message,
  capacity,
  loading,
  wasmReady,
  onMessageChange,
  onEncode,
}: EncodePanelProps) => {
  const isOverCapacity = capacity !== null && message.length > capacity;
  const isDisabled = !message || loading || !wasmReady || isOverCapacity;

  return (
    <div className="mt-6 space-y-4">
      <div>
        <label className="block text-sm font-bold text-[#00ff88] mb-2 font-mono flex items-center gap-2">
          <span className="text-[#00d4ff]">{'>'}</span>
          SECRET MESSAGE
        </label>
        <textarea
          value={message}
          onChange={(e) => onMessageChange(e.target.value)}
          placeholder="Enter your classified message..."
          className="w-full px-4 py-3 rounded-lg border-2 border-[#00ff88]/30 bg-[#0f1419] text-[#00ff88] placeholder-gray-600 focus:border-[#00ff88] focus:outline-none resize-none font-mono"
          rows={6}
        />
        {message && capacity && (
          <p
            className={`mt-2 text-sm font-mono ${
              isOverCapacity ? "text-red-500" : "text-[#00d4ff]"
            }`}
          >
            [{message.length} / {capacity} bytes]
            {isOverCapacity && " - OVERFLOW ERROR!"}
          </p>
        )}
      </div>

      <button
        onClick={onEncode}
        disabled={isDisabled}
        className="w-full py-4 px-6 bg-[#00ff88] hover:bg-[#00d4ff] disabled:bg-gray-700 text-black font-bold rounded-lg transition-all disabled:cursor-not-allowed font-mono text-lg border-2 border-[#00ff88] disabled:border-gray-600"
      >
        {!wasmReady
          ? "[INITIALIZING...]"
          : loading
          ? "[ENCODING...]"
          : "[EXECUTE ENCODE]"}
      </button>
    </div>
  );
};

export default EncodePanel;

