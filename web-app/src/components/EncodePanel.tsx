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
    <div className="space-y-4">
      <div>
        <label htmlFor="secret-message" className="mb-2 block text-sm font-medium text-[#45444a]">
          Message
        </label>
        <textarea
          id="secret-message"
          value={message}
          onChange={(e) => onMessageChange(e.target.value)}
          placeholder="Type something only the recipient should see..."
          className="w-full resize-none border border-[#aaa8ae] bg-white px-4 py-3.5 text-sm leading-6 text-[#29282e] placeholder:text-[#a4a2a8] focus:border-[#6658d3]"
          rows={5}
        />
        {message && capacity !== null && (
          <p
            className={`mt-2 text-right text-xs ${
              isOverCapacity ? "font-medium text-[#b74e45]" : "text-[#8a898f]"
            }`}
          >
            {message.length.toLocaleString()} / {capacity.toLocaleString()}
            {isOverCapacity && " - message is too long"}
          </p>
        )}
      </div>

      <button
        onClick={onEncode}
        disabled={isDisabled}
        className="w-full border border-[#41369b] bg-[#6658d3] px-6 py-3.5 font-technical text-xs font-semibold uppercase text-white transition-colors hover:bg-[#5749c3] disabled:cursor-not-allowed disabled:border-[#aaa8ae] disabled:bg-[#d2d0d4] disabled:text-[#77767d]"
      >
        {!wasmReady
          ? "Initializing..."
          : loading
            ? "Hiding message..."
            : "Hide message in image"}
      </button>
    </div>
  );
};

export default EncodePanel;
