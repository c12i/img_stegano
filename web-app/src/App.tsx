import { useCallback, useEffect, useMemo, useState } from "react";
import { useDropzone } from "react-dropzone";
import { FaDownload, FaLock, FaUnlock, FaImage } from "react-icons/fa";
import { getAsByteArray } from "./utils/files";

type Mode = "encode" | "decode";

const App = () => {
  const { acceptedFiles, getRootProps, getInputProps, isDragActive } =
    useDropzone({
      accept: { "image/png": [".png"], "image/jpeg": [".jpg", ".jpeg"] },
      multiple: false,
    });

  const [encodedImage, setEncodedImage] = useState<Uint8Array>();
  const [decodedText, setDecodedText] = useState<string>("");
  const [mode, setMode] = useState<Mode>("encode");
  const [message, setMessage] = useState<string>("");
  const [capacity, setCapacity] = useState<number | null>(null);
  const [error, setError] = useState<string>("");
  const [loading, setLoading] = useState(false);
  const [wasmReady, setWasmReady] = useState(false);

  // Initialize WASM module on mount
  useEffect(() => {
    import("../pkg/img_stegano_wasm")
      .then((mod) => {
        // WASM module loaded
        setWasmReady(true);
      })
      .catch((err) => {
        console.error("Failed to load WASM module:", err);
        setError("Failed to load WASM module. Please refresh the page.");
      });
  }, []);

  const encodeText = useCallback(async () => {
    if (!acceptedFiles[0] || !message || !wasmReady) return;

    setLoading(true);
    setError("");

    try {
      const mod = await import("../pkg/img_stegano_wasm");
      const buf = await getAsByteArray(acceptedFiles[0]);
      const result = mod.encode_text(
        buf,
        acceptedFiles[0].type.split("/")[1],
        message
      );
      setEncodedImage(result);
    } catch (err: any) {
      setError(err.message || err.toString() || "Failed to encode message");
    } finally {
      setLoading(false);
    }
  }, [acceptedFiles, message, wasmReady]);

  const decodeText = useCallback(async () => {
    if (!acceptedFiles[0] || !wasmReady) return;

    setLoading(true);
    setError("");

    try {
      const mod = await import("../pkg/img_stegano_wasm");
      const buf = await getAsByteArray(acceptedFiles[0]);
      const result = mod.decode_text(buf);
      setDecodedText(result);
    } catch (err: any) {
      setError(err.message || err.toString() || "Failed to decode message");
    } finally {
      setLoading(false);
    }
  }, [acceptedFiles, wasmReady]);

  const loadCapacity = useCallback(async () => {
    if (!acceptedFiles[0] || !wasmReady) return;

    try {
      const mod = await import("../pkg/img_stegano_wasm");
      const buf = await getAsByteArray(acceptedFiles[0]);
      const cap = mod.get_image_capacity(buf);
      setCapacity(cap);
    } catch (err) {
      console.error("Failed to get capacity:", err);
      setCapacity(null);
    }
  }, [acceptedFiles, wasmReady]);

  const imageUrl = useMemo(() => {
    if (!encodedImage) return;
    const blob = new Blob([encodedImage.buffer as BlobPart], {
      type: acceptedFiles[0]?.type,
    });
    return URL.createObjectURL(blob);
  }, [acceptedFiles, encodedImage]);

  const handleModeChange = (newMode: Mode) => {
    setMode(newMode);
    setDecodedText("");
    setEncodedImage(undefined);
    setError("");
  };

  // Load capacity when file is selected
  useEffect(() => {
    if (acceptedFiles[0] && wasmReady) {
      loadCapacity();
    }
  }, [acceptedFiles, wasmReady, loadCapacity]);

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100 dark:from-slate-900 dark:to-slate-800">
      <header className="bg-white dark:bg-slate-800 shadow-sm border-b border-slate-200 dark:border-slate-700">
        <div className="max-w-4xl mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <FaImage className="text-2xl text-blue-600 dark:text-blue-400" />
              <h1 className="text-2xl font-bold text-slate-900 dark:text-white">
                img_stegano
              </h1>
            </div>
            <span className="text-sm text-slate-600 dark:text-slate-400">
              Hide messages in images
            </span>
          </div>
        </div>
      </header>

      <main className="max-w-4xl mx-auto px-4 py-8">
        <div className="flex gap-4 mb-8">
          <button
            onClick={() => handleModeChange("encode")}
            className={`flex-1 py-3 px-6 rounded-lg font-medium transition-all ${
              mode === "encode"
                ? "bg-blue-600 text-white shadow-lg shadow-blue-500/50"
                : "bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700"
            }`}
          >
            <FaLock className="inline mr-2" />
            Encode Message
          </button>
          <button
            onClick={() => handleModeChange("decode")}
            className={`flex-1 py-3 px-6 rounded-lg font-medium transition-all ${
              mode === "decode"
                ? "bg-green-600 text-white shadow-lg shadow-green-500/50"
                : "bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700"
            }`}
          >
            <FaUnlock className="inline mr-2" />
            Decode Message
          </button>
        </div>


        <div
          {...getRootProps()}
          className={`border-2 border-dashed rounded-xl p-12 text-center cursor-pointer transition-all ${
            isDragActive
              ? "border-blue-500 bg-blue-50 dark:bg-blue-900/20"
              : "border-slate-300 dark:border-slate-600 hover:border-blue-400 dark:hover:border-blue-500 bg-white dark:bg-slate-800"
          }`}
        >
          <input {...getInputProps()} />
          <FaImage className="mx-auto text-5xl text-slate-400 dark:text-slate-500 mb-4" />
          <p className="text-lg font-medium text-slate-700 dark:text-slate-300 mb-2">
            {isDragActive
              ? "Drop your image here"
              : "Drag & drop an image, or click to select"}
          </p>
          <p className="text-sm text-slate-500 dark:text-slate-400">
            Supports PNG and JPEG formats
          </p>
        </div>

        {acceptedFiles[0] && (
          <div className="mt-6 p-4 bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700">
            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium text-slate-900 dark:text-white">
                  {acceptedFiles[0].name}
                </p>
                <p className="text-sm text-slate-500 dark:text-slate-400">
                  {(acceptedFiles[0].size / 1024).toFixed(2)} KB
                </p>
              </div>
              {capacity !== null && (
                <div className="text-right">
                  <p className="text-sm font-medium text-slate-700 dark:text-slate-300">
                    Capacity
                  </p>
                  <p className="text-lg font-bold text-blue-600 dark:text-blue-400">
                    {capacity} bytes
                  </p>
                </div>
              )}
            </div>
          </div>
        )}

        {mode === "encode" && acceptedFiles[0] && (
          <div className="mt-6 space-y-4">
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                Message to hide
              </label>
              <textarea
                value={message}
                onChange={(e) => setMessage(e.target.value)}
                placeholder="Enter your secret message..."
                className="w-full px-4 py-3 rounded-lg border border-slate-300 dark:border-slate-600 bg-white dark:bg-slate-800 text-slate-900 dark:text-white placeholder-slate-400 focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
                rows={4}
              />
              {message && capacity && (
                <p
                  className={`mt-2 text-sm ${
                    message.length > capacity
                      ? "text-red-600 dark:text-red-400"
                      : "text-slate-600 dark:text-slate-400"
                  }`}
                >
                  {message.length} / {capacity} bytes
                  {message.length > capacity && " - Message too large!"}
                </p>
              )}
            </div>

            <button
              onClick={encodeText}
              disabled={!message || loading || message.length > (capacity || 0)}
              className="w-full py-3 px-6 bg-blue-600 hover:bg-blue-700 disabled:bg-slate-300 dark:disabled:bg-slate-700 text-white font-medium rounded-lg transition-colors disabled:cursor-not-allowed"
            >
              {loading ? "Encoding..." : "Encode Message"}
            </button>
          </div>
        )}

        {mode === "decode" && acceptedFiles[0] && (
          <div className="mt-6">
            <button
              onClick={decodeText}
              disabled={loading}
              className="w-full py-3 px-6 bg-green-600 hover:bg-green-700 disabled:bg-slate-300 dark:disabled:bg-slate-700 text-white font-medium rounded-lg transition-colors disabled:cursor-not-allowed"
            >
              {loading ? "Decoding..." : "Decode Message"}
            </button>
          </div>
        )}

        {error && (
          <div className="mt-6 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
            <p className="text-red-800 dark:text-red-200 font-medium">
              {error}
            </p>
          </div>
        )}

        {encodedImage && imageUrl && (
          <div className="mt-6 p-6 bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700">
            <h3 className="text-lg font-semibold text-slate-900 dark:text-white mb-4">
              ✓ Message Encoded Successfully
            </h3>
            <div className="flex flex-col md:flex-row gap-4 items-center">
              <img
                src={imageUrl}
                alt="Encoded"
                className="max-w-xs rounded-lg shadow-md"
              />
              <a
                href={imageUrl}
                download="encoded-image.png"
                className="flex items-center gap-2 px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-colors"
              >
                <FaDownload />
                Download Image
              </a>
            </div>
          </div>
        )}

        {decodedText && (
          <div className="mt-6 p-6 bg-white dark:bg-slate-800 rounded-lg shadow-sm border border-slate-200 dark:border-slate-700">
            <h3 className="text-lg font-semibold text-slate-900 dark:text-white mb-4">
              ✓ Message Decoded Successfully
            </h3>
            <div className="p-4 bg-slate-50 dark:bg-slate-900 rounded-lg">
              <p className="text-slate-900 dark:text-white whitespace-pre-wrap break-words">
                {decodedText}
              </p>
            </div>
          </div>
        )}
      </main>
    </div>
  );
};

export default App;