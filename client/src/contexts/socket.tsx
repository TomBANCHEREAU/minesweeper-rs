import { ComponentChildren, createContext } from "preact";
import { PropsWithChildren } from "preact/compat";
import { SocketStatus, useSocket } from "../hooks/use-socket";

export const SocketContext = createContext<{
  socket: Pick<WebSocket, "send" | "url">;
  status: SocketStatus;
}>(undefined);

export function SocketContextProvider({
  url,
  reconnectOnError = false,
  children,
}: PropsWithChildren<{
  url: string;
  reconnectOnError?: boolean;
}>): ComponentChildren {
  console.log("hey");
  const { socket, status } = useSocket(url, { reconnectOnError });
  return (
    <SocketContext.Provider value={{ socket, status }}>
      {children}
    </SocketContext.Provider>
  );
}
