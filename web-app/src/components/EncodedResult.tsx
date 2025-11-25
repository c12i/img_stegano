import { FaDownload } from "react-icons/fa";

interface EncodedResultProps {
  imageUrl: string;
  originalFileName: string;
}

const EncodedResult = ({ imageUrl, originalFileName }: EncodedResultProps) => {
  const getEncodedFileName = (originalName: string) => {
    const lastDotIndex = originalName.lastIndexOf('.');
    if (lastDotIndex === -1) {
      return `${originalName}-encoded.png`;
    }
    const nameWithoutExt = originalName.substring(0, lastDotIndex);
    const ext = originalName.substring(lastDotIndex);
    return `${nameWithoutExt}-encoded${ext}`;
  };
  return (
    <div className="mt-6 p-6 bg-[#0f1419] rounded-lg border-2 border-[#00ff88]">
      <h3 className="text-lg font-bold text-[#00ff88] mb-4 font-mono flex items-center gap-2">
        <span className="text-[#00d4ff]">{'>'}</span>
        [ENCODING COMPLETE]
      </h3>
      <div className="flex flex-col md:flex-row gap-6 items-center">
        <img
          src={imageUrl}
          alt="Encoded"
          className="max-w-xs rounded-lg border-2 border-[#00ff88]/30"
        />
        <a
          href={imageUrl}
          download={getEncodedFileName(originalFileName)}
          className="flex items-center gap-3 px-8 py-4 bg-[#00ff88] hover:bg-[#00d4ff] text-black font-bold rounded-lg transition-all font-mono text-lg border-2 border-[#00ff88]"
        >
          <FaDownload />
          [DOWNLOAD]
        </a>
      </div>
    </div>
  );
};

export default EncodedResult;

