import { FaInfoCircle } from "react-icons/fa";
import Logo from "./Logo";

interface HeaderProps {
  onAboutClick: () => void;
}

const Header = ({ onAboutClick }: HeaderProps) => {
  return (
    <header className="bg-[#0f1419] border-b-2 border-[#00ff88]/30 shadow-lg">
      <div className="max-w-5xl mx-auto px-4 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-4">
            <Logo className="w-12 h-12" />
            <div>
              <h1 className="text-2xl font-bold text-[#00ff88] text-shadow-glow font-mono tracking-wider">
                img_stegano
              </h1>
              <p className="text-xs text-[#00d4ff] font-mono">
                [LSB REPLACEMENT STEGANOGRAPHY]
              </p>
            </div>
          </div>
          <button
            onClick={onAboutClick}
            className="flex items-center gap-2 px-4 py-2 bg-[#00ff88]/10 hover:bg-[#00ff88]/20 border border-[#00ff88]/30 rounded text-[#00ff88] transition-all font-mono text-sm"
          >
            <FaInfoCircle />
            <span className="hidden sm:inline">ABOUT</span>
          </button>
        </div>
      </div>
    </header>
  );
};

export default Header;
