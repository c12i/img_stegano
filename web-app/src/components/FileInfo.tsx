import { FiFile, FiX } from "react-icons/fi";

interface FileInfoProps {
  fileName: string;
  fileSize: number;
  capacity: number | null;
  onClear: () => void;
}

const FileInfo = ({ fileName, fileSize, capacity, onClear }: FileInfoProps) => {
  return (
    <div className="flex items-center gap-4 border border-[#aaa8ae] bg-[#f4f3ef] p-4">
      <div className="flex h-10 w-10 flex-none items-center justify-center border border-[#a49cdc] bg-[#ebe8ff] text-[#6658d3]">
        <FiFile size={19} />
      </div>
      <div className="min-w-0 flex-1">
        <p className="truncate text-sm font-semibold text-[#29282e]">{fileName}</p>
        <div className="mt-1 flex flex-wrap gap-x-3 text-xs text-[#77767d]">
          <span>{(fileSize / 1024).toFixed(1)} KB</span>
          {capacity !== null && <span>Up to {capacity.toLocaleString()} characters</span>}
        </div>
      </div>
      <button
        onClick={onClear}
        className="flex h-9 w-9 flex-none items-center justify-center border border-transparent text-[#77767d] transition-colors hover:border-[#aaa8ae] hover:text-[#29282e]"
        title="Remove image"
        aria-label="Remove image"
      >
        <FiX size={18} />
      </button>
    </div>
  );
};

export default FileInfo;
