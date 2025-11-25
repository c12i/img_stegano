interface DecodePanelProps {
  loading: boolean;
  wasmReady: boolean;
  onDecode: () => void;
}

const DecodePanel = ({ loading, wasmReady, onDecode }: DecodePanelProps) => {
  return (
    <div className="mt-6">
      <button
        onClick={onDecode}
        disabled={loading || !wasmReady}
        className="w-full py-4 px-6 bg-[#00d4ff] hover:bg-[#00ff88] disabled:bg-gray-700 text-black font-bold rounded-lg transition-all disabled:cursor-not-allowed font-mono text-lg border-2 border-[#00d4ff] disabled:border-gray-600"
      >
        {!wasmReady
          ? "[INITIALIZING...]"
          : loading
          ? "[DECODING...]"
          : "[EXECUTE DECODE]"}
      </button>
    </div>
  );
};

export default DecodePanel;

