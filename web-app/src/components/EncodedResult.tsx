import { FiDownload } from "react-icons/fi";

interface EncodedResultProps {
  imageUrl: string;
  originalFileName: string;
}

const EncodedResult = ({ imageUrl, originalFileName }: EncodedResultProps) => {
  const getEncodedFileName = (originalName: string) => {
    const lastDotIndex = originalName.lastIndexOf(".");
    if (lastDotIndex === -1) {
      return `${originalName}-encoded.png`;
    }
    const nameWithoutExt = originalName.substring(0, lastDotIndex);
    const ext = originalName.substring(lastDotIndex);
    return `${nameWithoutExt}-encoded${ext}`;
  };
  return (
    <div className="mt-7 border-t border-black/[0.07] pt-7">
      <div className="mb-5 flex items-center gap-3">
        <div>
          <p className="text-lg font-semibold">Encoding complete</p>
          <p className="text-xs text-[#77767d]">The output contains the embedded message.</p>
        </div>
      </div>
      <div className="flex flex-col gap-5 border border-[#aaa8ae] bg-[#f4f3ef] p-4 sm:flex-row sm:items-center">
        <img
          src={imageUrl}
          alt="Image containing the hidden message"
          className="h-36 w-full border border-[#aaa8ae] object-cover sm:w-40"
        />
        <div className="flex-1">
          <p className="mb-4 text-sm leading-6 text-[#6e6d73]">
            Download and share this PNG without converting or compressing it.
          </p>
          <a
            href={imageUrl}
            download={getEncodedFileName(originalFileName)}
            className="inline-flex items-center gap-2 border border-black bg-[#202126] px-5 py-3 font-technical text-xs font-semibold uppercase text-white transition-colors hover:bg-[#38383f]"
          >
            <FiDownload size={17} />
            Download image
          </a>
        </div>
      </div>
    </div>
  );
};

export default EncodedResult;
