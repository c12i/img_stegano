import { FaTimes } from "react-icons/fa";

interface AboutModalProps {
  isOpen: boolean;
  onClose: () => void;
}

const AboutModal = ({ isOpen, onClose }: AboutModalProps) => {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm">
      <div className="relative w-full max-w-3xl max-h-[90vh] flex flex-col bg-[#0f1419] border-2 border-[#00ff88] rounded-lg shadow-2xl scanline">
        <div className="flex-shrink-0 bg-[#0f1419] border-b border-[#00ff88]/30 p-6 flex items-center justify-between">
          <h2 className="text-2xl font-bold text-[#00ff88] text-shadow-glow">
            [ABOUT] LSB Steganography
          </h2>
          <button
            onClick={onClose}
            className="text-[#00d4ff] hover:text-[#00ff88] transition-colors"
          >
            <FaTimes size={24} />
          </button>
        </div>

        <div className="flex-1 overflow-y-auto p-6 space-y-6 text-gray-300 font-mono text-sm">
          <section>
            <h3 className="text-lg font-bold text-[#00d4ff] mb-3 flex items-center gap-2">
              <span className="text-[#00ff88]">{'>'}</span> What is Steganography?
            </h3>
            <p className="leading-relaxed">
              Steganography is the practice of concealing messages within other non-secret data.
              Unlike encryption which makes data unreadable, steganography hides the very existence
              of the message. The word comes from Greek: <span className="text-[#00ff88]">steganos</span> (covered)
              and <span className="text-[#00ff88]">graphein</span> (writing).
            </p>
          </section>

          <section>
            <h3 className="text-lg font-bold text-[#00d4ff] mb-3 flex items-center gap-2">
              <span className="text-[#00ff88]">{'>'}</span> LSB (Least Significant Bit) Technique
            </h3>
            <p className="leading-relaxed mb-3">
              This tool uses LSB steganography to hide text messages in images. Here's how it works:
            </p>
            <div className="bg-black/40 border border-[#00ff88]/20 rounded p-4 space-y-3">
              <div>
                <span className="text-[#00ff88]">1.</span> Each pixel in an image has RGB color channels (Red, Green, Blue)
              </div>
              <div>
                <span className="text-[#00ff88]">2.</span> Each channel is an 8-bit value (0-255)
              </div>
              <div>
                <span className="text-[#00ff88]">3.</span> We modify only the <span className="text-[#00d4ff] font-bold">least significant bit</span> of each channel
              </div>
              <div>
                <span className="text-[#00ff88]">4.</span> This creates imperceptible changes to the human eye
              </div>
            </div>
          </section>

          <section>
            <h3 className="text-lg font-bold text-[#00d4ff] mb-3 flex items-center gap-2">
              <span className="text-[#00ff88]">{'>'}</span> Why PNG Only?
            </h3>
            <p className="leading-relaxed mb-3">
              This tool <span className="text-[#00ff88] font-bold">only supports PNG images</span> for reliable LSB steganography:
            </p>
            <div className="bg-black/40 border border-[#00ff88]/20 rounded p-4 space-y-3">
              <div>
                <span className="text-[#00ff88] font-bold">✓ PNG:</span> Lossless compression, preserves exact pixel values
              </div>
              <div>
                <span className="text-red-400 font-bold">✗ JPEG/WebP:</span> Lossy compression destroys LSB data
              </div>
              <div>
                <span className="text-yellow-400 font-bold">⚠ BMP/TIFF:</span> Technically lossless but have format-specific quirks that can corrupt LSB data during encoding/decoding cycles
              </div>
            </div>
            <p className="leading-relaxed mt-3 text-sm">
              <span className="text-[#00d4ff] font-bold">Technical Details:</span> While BMP and TIFF are lossless,
              they use different internal representations (padding, byte ordering, color spaces) that may not preserve
              RGB pixel data identically through the image library's encoding/decoding pipeline. PNG's widespread
              support and standardized format makes it the most reliable choice for LSB steganography.
            </p>
          </section>

          <section>
            <h3 className="text-lg font-bold text-[#00d4ff] mb-3 flex items-center gap-2">
              <span className="text-[#00ff88]">{'>'}</span> Implementation
            </h3>
            <p className="leading-relaxed">
              This tool is built with <span className="text-[#00ff88]">Rust</span> (core library)
              and compiled to <span className="text-[#00d4ff]">WebAssembly</span> for browser execution.
              All processing happens locally in your browser - no data is sent to any server.
            </p>
          </section>
        </div>

        <div className="flex-shrink-0 bg-[#0f1419] border-t border-[#00ff88]/30 p-4 text-center">
          <button
            onClick={onClose}
            className="px-6 py-2 bg-[#00ff88] hover:bg-[#00d4ff] text-black font-bold rounded transition-colors"
          >
            [CLOSE]
          </button>
        </div>
      </div>
    </div>
  );
};

export default AboutModal;

