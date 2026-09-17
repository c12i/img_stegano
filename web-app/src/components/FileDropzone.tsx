import { useDropzone } from "react-dropzone";
import { FiImage, FiUpload } from "react-icons/fi";

interface FileDropzoneProps {
  onFileAccepted: (file: File) => void;
}

const FileDropzone = ({ onFileAccepted }: FileDropzoneProps) => {
  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    accept: {
      "image/png": [".png"],
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
      className={`group border border-dashed p-8 text-center cursor-pointer transition-all sm:p-10 ${
        isDragActive
          ? "border-[#6658d3] bg-[#f2f0ff]"
          : "border-[#cbc9ce] bg-[#faf9f5] hover:border-[#9087db] hover:bg-[#f8f6ff]"
      }`}
    >
      <input {...getInputProps()} />
      <div className="mx-auto mb-5 flex h-12 w-12 items-center justify-center border border-[#a49cdc] bg-[#ebe8ff] text-[#6658d3] transition-transform group-hover:-translate-y-0.5">
        {isDragActive ? <FiUpload size={22} /> : <FiImage size={23} />}
      </div>
      <p className="font-semibold text-[#29282e]">
        {isDragActive ? "Drop it here" : "Drop your image here"}
      </p>
      <p className="mt-1.5 text-sm text-[#77767d]">
        or <span className="font-medium text-[#6658d3]">browse your files</span>
      </p>
    </div>
  );
};

export default FileDropzone;
