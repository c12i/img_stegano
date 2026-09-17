import { FiX } from "react-icons/fi";

interface AboutModalProps {
  isOpen: boolean;
  onClose: () => void;
}

const AboutModal = ({ isOpen, onClose }: AboutModalProps) => {
  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-[#202126]/40 p-4 backdrop-blur-sm"
      role="dialog"
      aria-modal="true"
      aria-labelledby="about-title"
      onMouseDown={(event) => {
        if (event.target === event.currentTarget) onClose();
      }}
    >
      <div className="flex max-h-[90vh] w-full max-w-xl flex-col overflow-hidden border border-[#3c3b40] bg-[#fdfdfb]">
        <header className="flex items-center justify-between border-b border-black/[0.07] px-6 py-5">
          <h2 id="about-title" className="text-xl font-semibold tracking-[-0.01em]">
            How steganography works
          </h2>
          <button
            onClick={onClose}
            className="flex h-9 w-9 items-center justify-center border border-transparent text-[#77767d] transition-colors hover:border-[#aaa8ae] hover:text-[#29282e]"
            aria-label="Close dialog"
          >
            <FiX size={19} />
          </button>
        </header>

        <div className="overflow-y-auto px-6 py-6 text-sm leading-6 text-[#5e5d64] sm:px-8">
          <p>
            Steganography conceals a message inside ordinary data. This tool
            places text in tiny variations in a PNG image, hiding the existence
            of the message rather than visibly encrypting it.
          </p>

          <section className="mt-7">
            <h3 className="font-semibold text-[#29282e]">Least significant bits</h3>
            <p className="mt-2">
              Every pixel contains red, green, and blue values. Changing the
              smallest bit of those values is usually invisible to the eye, but
              those bits can carry a text message that this tool can read later.
            </p>
          </section>

          <section className="mt-7 border border-[#aaa8ae] bg-[#f3f1ec] p-5">
            <h3 className="font-semibold text-[#29282e]">Why PNG?</h3>
            <p className="mt-2">
              PNG uses lossless compression, so the hidden bits remain intact.
              JPEG and WebP compression can alter those bits and destroy the
              message.
            </p>
          </section>

          <section className="mt-7">
            <h3 className="font-semibold text-[#29282e]">Your files stay local</h3>
            <p className="mt-2">
              The Rust engine runs through WebAssembly in your browser. Images
              and messages are never uploaded to a server.
            </p>
          </section>
        </div>

        <footer className="border-t border-black/[0.07] px-6 py-4 text-right">
          <button
            onClick={onClose}
            className="border border-[#41369b] bg-[#6658d3] px-5 py-2.5 font-technical text-xs font-semibold uppercase text-white transition-colors hover:bg-[#5749c3]"
          >
            Close
          </button>
        </footer>
      </div>
    </div>
  );
};

export default AboutModal;
