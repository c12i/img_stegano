interface DecodedResultProps {
  decodedText: string;
}

const DecodedResult = ({ decodedText }: DecodedResultProps) => {
  return (
    <div className="mt-6 p-6 bg-[#0f1419] rounded-lg border-2 border-[#00d4ff]">
      <h3 className="text-lg font-bold text-[#00d4ff] mb-4 font-mono flex items-center gap-2">
        <span className="text-[#00ff88]">{">"}</span>
        [DECODING COMPLETE]
      </h3>
      <div className="p-4 bg-black/40 rounded-lg border border-[#00d4ff]/30">
        <p className="text-[#00ff88] whitespace-pre-wrap break-words font-mono">
          {decodedText}
        </p>
      </div>
    </div>
  );
};

export default DecodedResult;
