interface HeaderProps {
  onAboutClick: () => void;
}

const Header = ({ onAboutClick }: HeaderProps) => {
  return (
    <header className="border-b border-black/[0.07]">
      <div className="mx-auto flex max-w-3xl items-center justify-between px-5 py-5 md:px-8">
        <div>
          <p className="font-technical text-sm font-semibold uppercase tracking-[0.08em]">Img Stegano</p>
          <p className="font-technical text-[10px] uppercase tracking-[0.08em] text-[#77767d]">LSB image steganography</p>
        </div>
        <button
          onClick={onAboutClick}
          className="border border-transparent px-3 py-2 font-technical text-xs font-medium uppercase text-[#55545a] transition-colors hover:border-[#aaa8ae] hover:text-[#202126]"
        >
          How it works
        </button>
      </div>
    </header>
  );
};

export default Header;
