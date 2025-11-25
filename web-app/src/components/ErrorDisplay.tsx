interface ErrorDisplayProps {
  error: string;
}

const ErrorDisplay = ({ error }: ErrorDisplayProps) => {
  if (!error) return null;

  return (
    <div className="mt-6 p-4 bg-red-900/20 border-2 border-red-500 rounded-lg">
      <p className="text-red-400 font-bold font-mono flex items-center gap-2">
        <span>[ERROR]</span>
        {error}
      </p>
    </div>
  );
};

export default ErrorDisplay;

