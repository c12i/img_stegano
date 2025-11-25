interface FileInfoProps {
  fileName: string;
  fileSize: number;
  capacity: number | null;
}

const FileInfo = ({ fileName, fileSize, capacity }: FileInfoProps) => {
  return (
    <div className="mt-6 p-4 bg-[#0f1419] rounded-lg border-2 border-[#00ff88]/30">
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

