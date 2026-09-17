import { FaGithub } from "react-icons/fa";

const Footer = () => {
  return (
    <footer className="border-t border-black/[0.07] py-6">
      <div className="mx-auto flex max-w-3xl items-center justify-between px-5 text-xs text-[#77767d] md:px-8">
        <p>Built with Rust and WebAssembly</p>
        <a
          href="https://github.com/c12i/img_stegano"
          target="_blank"
          rel="noopener noreferrer"
          className="flex items-center gap-2 border border-transparent px-2 py-1.5 transition-colors hover:border-[#aaa8ae] hover:text-[#29282e]"
        >
          <FaGithub className="text-base" />
          Source
        </a>
      </div>
    </footer>
  );
};

export default Footer;
