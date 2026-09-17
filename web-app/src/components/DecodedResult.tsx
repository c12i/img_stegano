import { useState } from "react";
import { FiCopy } from "react-icons/fi";

interface DecodedResultProps {
  decodedText: string;
}

const DecodedResult = ({ decodedText }: DecodedResultProps) => {
  const [copied, setCopied] = useState(false);

  const copyMessage = async () => {
    await navigator.clipboard.writeText(decodedText);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1600);
  };

  return (
    <div className="mt-7 border-t border-black/[0.07] pt-7">
      <div className="mb-4 flex items-center justify-between gap-4">
        <div className="flex items-center gap-3">
          <p className="text-lg font-semibold">Decode result</p>
        </div>
        <button
          onClick={copyMessage}
          className="inline-flex items-center gap-1.5 border border-transparent px-3 py-2 font-technical text-xs font-semibold uppercase text-[#6658d3] transition-colors hover:border-[#a49cdc] hover:bg-[#f0eeff]"
        >
          <FiCopy size={15} />
          {copied ? "Copied" : "Copy"}
        </button>
      </div>
      <div className="border border-[#aaa8ae] bg-[#f4f3ef] p-5">
        <p className="whitespace-pre-wrap break-words text-sm leading-6 text-[#29282e]">
          {decodedText}
        </p>
      </div>
    </div>
  );
};

export default DecodedResult;
