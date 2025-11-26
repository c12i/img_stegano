import { FaGithub, FaGlobe } from "react-icons/fa";

const Footer = () => {
  return (
    <footer className="mt-16 py-6 border-t-2 border-[#00ff88]/30">
      <div className="max-w-5xl mx-auto px-4">
        <div className="flex flex-col items-center gap-4">
          <div className="flex items-center gap-6">
            <a
              href="https://github.com/c12i/img_stegano"
              target="_blank"
              rel="noopener"
              className="flex items-center gap-2 text-[#00ff88] hover:text-[#00d4ff] transition-colors font-mono text-sm group"
            >
              <FaGithub className="text-xl group-hover:scale-110 transition-transform" />
              <span className="hidden sm:inline">[SOURCE]</span>
            </a>
            <a
              href="https://c12i.xyz"
              target="_blank"
              className="flex items-center gap-2 text-[#00d4ff] hover:text-[#00ff88] transition-colors font-mono text-sm group"
            >
              <FaGlobe className="text-xl group-hover:scale-110 transition-transform" />
              <span className="hidden sm:inline">[c12i.xyz]</span>
            </a>
          </div>
          <p className="text-gray-500 font-mono text-xs">
            [POWERED BY RUST + WASM]
          </p>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
