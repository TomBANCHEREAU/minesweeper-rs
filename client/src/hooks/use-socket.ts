import { useEffect, useState } from "preact/hooks";
import { useEvent } from "./use-event";

export type SocketStatus = "connecting" | "open" | "closed" | "error";

export const useSocket = (
  url: string,
  options: { reconnectOnError?: boolean } = {}
) => {
  const { reconnectOnError } = options;
  const [[socket, status], setSocket] = useState<[WebSocket, SocketStatus]>(
    () => [new WebSocket(url), "connecting"]
  );

  useEvent(socket, "open", () => setSocket(([socket]) => [socket, "open"]), []);
  useEvent(
    socket,
    "error",
    (event: Event) => {
      if (!reconnectOnError) return setSocket(([socket]) => [socket, "error"]);
      console.info("Reconnecting socket after error:", event);
      setSocket([new WebSocket(url), "connecting"]);
    },
    [reconnectOnError, url]
  );
  useEvent(
    socket,
    "close",
    () => setSocket(([socket]) => [socket, "closed"]),
    []
  );

  useEffect(() => {
    if (socket.url.endsWith(url)) return;
    console.info(socket.url, url);
    setSocket([new WebSocket(url), "connecting"]);
  }, [socket, url, setSocket]);
  return { socket, status };
};
