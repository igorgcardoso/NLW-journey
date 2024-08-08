import { clsx } from "clsx";
import { createContext, useContext } from "react";
import {
  ActivityIndicator,
  Text,
  TextProps,
  TouchableOpacity,
  TouchableOpacityProps,
} from "react-native";

type Variants = "primary" | "secondary";

interface ButtonProps extends TouchableOpacityProps {
  variant?: Variants;
  isLoading?: boolean;
}

const ThemeContext = createContext<{ variant?: Variants }>({});

function Button({
  variant = "primary",
  isLoading,
  children,
  ...props
}: ButtonProps) {
  return (
    <TouchableOpacity
      className={clsx(
        "h-11 w-full flex-row items-center justify-center gap-2 rounded-lg",
        {
          "bg-lime-300": variant === "primary",
          "bg-zinc-800": variant === "secondary",
        },
      )}
      activeOpacity={0.7}
      disabled={isLoading}
      {...props}
    >
      <ThemeContext.Provider value={{ variant }}>
        {isLoading ? <ActivityIndicator className="text-lime-950" /> : children}
      </ThemeContext.Provider>
    </TouchableOpacity>
  );
}

function Title({ children }: TextProps) {
  const { variant } = useContext(ThemeContext);
  return (
    <Text
      className={clsx("font-semibold text-base", {
        "text-lime-950": variant === "primary",
        "text-zinc-200": variant === "secondary",
      })}
    >
      {children}
    </Text>
  );
}

Button.Title = Title;

export { Button };
