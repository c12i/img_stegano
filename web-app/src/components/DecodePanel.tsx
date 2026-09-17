interface DecodePanelProps {
  loading: boolean;
  wasmReady: boolean;
  onDecode: () => void;
}

const DecodePanel = ({ loading, wasmReady, onDecode }: DecodePanelProps) => {
  return (
    <div>
      <p className="mb-5 text-sm leading-6 text-[#6e6d73]">
        We’ll inspect the image for a message embedded with this tool.
      </p>
      <button
        onClick={onDecode}
        disabled={loading || !wasmReady}
        className="w-full border border-black bg-[#202126] px-6 py-3.5 font-technical text-xs font-semibold uppercase text-white transition-colors hover:bg-[#3b3b40] disabled:cursor-not-allowed disabled:border-[#aaa8ae] disabled:bg-[#d2d0d4] disabled:text-[#77767d]"
      >
        {!wasmReady
          ? "Initializing..."
          : loading
            ? "Looking for a message..."
            : "Reveal hidden message"}
      </button>
    </div>
  );
};

export default DecodePanel;
