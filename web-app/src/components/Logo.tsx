const Logo = ({ className = "w-10 h-10" }: { className?: string }) => {
  return (
    <svg
      className={className}
      viewBox="0 0 100 100"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      {/* Outer hexagon frame */}
      <path
        d="M50 5 L85 27.5 L85 72.5 L50 95 L15 72.5 L15 27.5 Z"
        stroke="url(#gradient1)"
        strokeWidth="2"
        fill="none"
        className="animate-pulse"
      />
      
      {/* Inner hexagon */}
      <path
        d="M50 15 L75 30 L75 70 L50 85 L25 70 L25 30 Z"
        stroke="url(#gradient2)"
        strokeWidth="1.5"
        fill="rgba(0, 255, 136, 0.05)"
      />
      
      {/* Eye symbol */}
      <ellipse
        cx="50"
        cy="50"
        rx="20"
        ry="12"
        stroke="#00ff88"
        strokeWidth="2"
        fill="none"
      />
      
      {/* Pupil */}
      <circle
        cx="50"
        cy="50"
        r="6"
        fill="#00ff88"
        className="animate-pulse"
      />
      
      {/* Binary digits scattered around */}
      <text
        x="35"
        y="25"
        fill="#00d4ff"
        fontSize="8"
        fontFamily="monospace"
        opacity="0.6"
      >
        01
      </text>
      <text
        x="60"
        y="25"
        fill="#00d4ff"
        fontSize="8"
        fontFamily="monospace"
        opacity="0.6"
      >
        10
      </text>
      <text
        x="20"
        y="55"
        fill="#00d4ff"
        fontSize="8"
        fontFamily="monospace"
        opacity="0.6"
      >
        11
      </text>
      <text
        x="75"
        y="55"
        fill="#00d4ff"
        fontSize="8"
        fontFamily="monospace"
        opacity="0.6"
      >
        00
      </text>
      
      {/* Gradients */}
      <defs>
        <linearGradient id="gradient1" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stopColor="#00ff88" />
          <stop offset="50%" stopColor="#00d4ff" />
          <stop offset="100%" stopColor="#00ff88" />
        </linearGradient>
        <linearGradient id="gradient2" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stopColor="#00d4ff" />
          <stop offset="100%" stopColor="#00ff88" />
        </linearGradient>
      </defs>
    </svg>
  );
};

export default Logo;

