import { useDropzone } from "react-dropzone";
import { FaImage } from "react-icons/fa";

interface FileDropzoneProps {
  onFileAccepted: (file: File) => void;
}

const FileDropzone = ({ onFileAccepted }: FileDropzoneProps) => {
  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    accept: {
      "image/png": [".png"]
    },
    multiple: false,
    onDrop: (acceptedFiles) => {
      if (acceptedFiles[0]) {
        onFileAccepted(acceptedFiles[0]);
      }
    },
  });

  return (
    <div
      {...getRootProps()}
      className={`border-2 border-dashed rounded-lg p-12 text-center cursor-pointer transition-all hex-pattern ${
        isDragActive
          ? "border-[#00ff88] bg-[#00ff88]/10 border-glow"
          : "border-[#00ff88]/30 hover:border-[#00ff88] bg-[#0f1419]/50"
      }`}
    >
      <input {...getInputProps()} />
      <FaImage className="mx-auto text-6xl text-[#00ff88]/50 mb-4" />
      <p className="text-lg font-bold text-[#00ff88] mb-2 font-mono">
        {isDragActive
          ? "[RECEIVING FILE...]"
          : "[DROP IMAGE OR CLICK TO SELECT]"}
      </p>
      <p className="text-sm text-[#00d4ff] font-mono">
        PNG ONLY
      </p>
      <p className="text-xs text-gray-500 font-mono mt-2">
        (Other formats may not preserve LSB data reliably)
      </p>
    </div>
  );
};

export default FileDropzone;

