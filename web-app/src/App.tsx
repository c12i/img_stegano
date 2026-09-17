import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { getAsByteArray } from "./utils/files";
import Header from "./components/Header";
import ModeSelector from "./components/ModeSelector";
import FileDropzone from "./components/FileDropzone";
import FileInfo from "./components/FileInfo";
import EncodePanel from "./components/EncodePanel";
import DecodePanel from "./components/DecodePanel";
import ErrorDisplay from "./components/ErrorDisplay";
import EncodedResult from "./components/EncodedResult";
import DecodedResult from "./components/DecodedResult";
import AboutModal from "./components/AboutModal";
import Footer from "./components/Footer";

type Mode = "encode" | "decode";
type WasmModule = typeof import("../pkg/img_stegano_wasm");

const App = () => {
  const [acceptedFile, setAcceptedFile] = useState<File | null>(null);
  const [encodedImage, setEncodedImage] = useState<Uint8Array>();
  const [decodedText, setDecodedText] = useState<string>("");
  const [mode, setMode] = useState<Mode>(getInitialMode());
  const [message, setMessage] = useState<string>("");
  const [capacity, setCapacity] = useState<number | null>(null);
  const [error, setError] = useState<string>("");
  const [loading, setLoading] = useState(false);
  const [wasmReady, setWasmReady] = useState(false);
  const [showAbout, setShowAbout] = useState(false);
  const wasmModule = useRef<WasmModule | null>(null);

  useEffect(() => {
    window.location.hash = mode;
  }, [mode]);

  useEffect(() => {
    import("../pkg/img_stegano_wasm")
      .then(async (mod) => {
        await mod.default();
        wasmModule.current = mod;
        setWasmReady(true);
      })
      .catch((err) => {
        console.error("Failed to load WASM module:", err);
        setError("Failed to load WASM module. Please refresh the page.");
      });
  }, []);

  const encodeText = useCallback(async () => {
    if (!acceptedFile || !message || !wasmReady || !wasmModule.current) return;

    setLoading(true);
    setError("");
    setEncodedImage(undefined);

    try {
      const buf = await getAsByteArray(acceptedFile);
      const result = wasmModule.current.encode_text(buf, message);
      setEncodedImage(result);
    } catch (err: any) {
      setError(err.message || err.toString() || "Failed to encode message");
    } finally {
      setLoading(false);
    }
  }, [acceptedFile, message, wasmReady]);

  const decodeText = useCallback(async () => {
    if (!acceptedFile || !wasmReady || !wasmModule.current) return;

    setLoading(true);
    setError("");
    setDecodedText("");

    try {
      const buf = await getAsByteArray(acceptedFile);
      const result = wasmModule.current.decode_text(buf);
      setDecodedText(result);
    } catch (err: any) {
      setError(err.message || err.toString() || "Failed to decode message");
    } finally {
      setLoading(false);
    }
  }, [acceptedFile, wasmReady]);

  const loadCapacity = useCallback(async () => {
    if (!acceptedFile || !wasmReady || !wasmModule.current) return;

    try {
      const buf = await getAsByteArray(acceptedFile);
      const cap = wasmModule.current.get_image_capacity(buf);
      setCapacity(cap);
    } catch (err) {
      console.error("Failed to get capacity:", err);
      setCapacity(null);
    }
  }, [acceptedFile, wasmReady]);

  const imageUrl = useMemo(() => {
    if (!encodedImage || !acceptedFile) return;
    const blob = new Blob([encodedImage as BlobPart], {
      type: acceptedFile.type || "image/png",
    });
    return URL.createObjectURL(blob);
  }, [acceptedFile, encodedImage]);

  useEffect(() => {
    return () => {
      if (imageUrl) URL.revokeObjectURL(imageUrl);
    };
  }, [imageUrl]);

  const handleModeChange = (newMode: Mode) => {
    setMode(newMode);
    setDecodedText("");
    setEncodedImage(undefined);
    setError("");
  };

  const handleFileAccepted = (file: File) => {
    setAcceptedFile(file);
    setDecodedText("");
    setEncodedImage(undefined);
    setError("");
  };

  const handleClearFile = () => {
    setAcceptedFile(null);
    setDecodedText("");
    setEncodedImage(undefined);
    setError("");
    setMessage("");
    setCapacity(null);
  };

  useEffect(() => {
    if (!(acceptedFile && wasmReady)) return;
    loadCapacity();
  }, [acceptedFile, wasmReady, loadCapacity]);

  return (
    <div className="min-h-screen bg-[#e9e9e4] text-[#202126]">
      <AboutModal isOpen={showAbout} onClose={() => setShowAbout(false)} />
      <Header onAboutClick={() => setShowAbout(true)} />

      <main className="mx-auto w-full max-w-3xl px-5 py-10 md:px-8 md:py-14">
        <section className="overflow-hidden border border-[#3c3b40] bg-[#fdfdfb]">
          <div className="border-b border-[#3c3b40] p-3 sm:p-4">
            <ModeSelector mode={mode} onModeChange={handleModeChange} />
          </div>

          <div className="p-5 sm:p-8">
            <div className="mb-5 flex items-baseline justify-between gap-4">
              <div>
                <h2 className="text-xl font-semibold tracking-[-0.01em]">
                  Choose an image
                </h2>
              </div>
              <span className="border border-[#b7b5ba] px-2 py-1 font-technical text-[10px] font-medium uppercase text-[#6e6d73]">
                PNG
              </span>
            </div>

            {acceptedFile ? (
              <FileInfo
                fileName={acceptedFile.name}
                fileSize={acceptedFile.size}
                capacity={capacity}
                onClear={handleClearFile}
              />
            ) : (
              <FileDropzone onFileAccepted={handleFileAccepted} />
            )}

            {acceptedFile && (
              <div className="mt-8 border-t border-black/[0.07] pt-7">
                <div className="mb-5">
                  <h2 className="text-xl font-semibold tracking-[-0.01em]">
                    {mode === "encode" ? "Write your message" : "Reveal the message"}
                  </h2>
                </div>

                {mode === "encode" ? (
                  <EncodePanel
                    message={message}
                    capacity={capacity}
                    loading={loading}
                    wasmReady={wasmReady}
                    onMessageChange={setMessage}
                    onEncode={encodeText}
                  />
                ) : (
                  <DecodePanel
                    loading={loading}
                    wasmReady={wasmReady}
                    onDecode={decodeText}
                  />
                )}
              </div>
            )}

            <ErrorDisplay error={error} />

            {encodedImage && imageUrl && acceptedFile && (
              <EncodedResult
                imageUrl={imageUrl}
                originalFileName={acceptedFile.name}
              />
            )}
            {decodedText && <DecodedResult decodedText={decodedText} />}
          </div>
        </section>
      </main>

      <Footer />
    </div>
  );
};

const getInitialMode = (): Mode => {
  const hash = window.location.hash.slice(1);
  if (hash === "encode" || hash === "decode") {
    return hash;
  }
  return "encode";
};

export default App;
