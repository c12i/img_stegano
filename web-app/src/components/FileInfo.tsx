import { FaTimes } from "react-icons/fa";

interface FileInfoProps {
  fileName: string;
  fileSize: number;
  capacity: number | null;
  onClear: () => void;
}

const FileInfo = ({ fileName, fileSize, capacity, onClear }: FileInfoProps) => {
  return (
    <div className="mt-6 p-4 bg-[#0f1419] rounded-lg border-2 border-[#00ff88]/30 relative">
      <button
        onClick={onClear}
        className="absolute -top-3 -right-3 text-[#00d4ff] hover:text-[#00ff88] transition-colors bg-[#0f1419] rounded-full p-1 border-2 border-[#00ff88]/30 hover:border-[#00ff88]"
        title="Remove image"
      >
        <FaTimes size={20} />
      </button>
      <div className="flex items-center justify-between">
        <div className="font-mono">
          <p className="font-bold text-[#00ff88] flex items-center gap-2">
            <span className="text-[#00d4ff]">{'>'}</span>
            {fileName}
          </p>
          <p className="text-sm text-gray-400 mt-1">
            SIZE: {(fileSize / 1024).toFixed(2)} KB
          </p>
        </div>
        {capacity !== null && (
          <div className="text-right font-mono">
            <p className="text-sm text-gray-400">CAPACITY</p>
            <p className="text-2xl font-bold text-[#00ff88] text-shadow-glow">
              {capacity}
            </p>
            <p className="text-xs text-[#00d4ff]">bytes</p>
          </div>
        )}
      </div>
    </div>
  );
};

export default FileInfo;

