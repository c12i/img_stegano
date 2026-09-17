interface ErrorDisplayProps {
  error: string;
}

const ErrorDisplay = ({ error }: ErrorDisplayProps) => {
  if (!error) return null;

  return (
    <div className="mt-6 border border-[#b74e45] bg-[#fff3f1] p-4" role="alert">
      <p className="font-technical text-xs font-semibold uppercase text-[#973f38]">Error</p>
      <p className="mt-1 text-sm text-[#a8534b]">{error}</p>
    </div>
  );
};

export default ErrorDisplay;
