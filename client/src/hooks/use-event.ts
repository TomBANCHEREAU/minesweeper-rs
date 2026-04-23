import { useEffect } from "preact/hooks";

export function useEvent<T extends EventTarget>(
  target: T,
  eventType: Parameters<T["addEventListener"]>[0],
  callback: Parameters<T["addEventListener"]>[1],
  deps: unknown[]
) {
  useEffect(() => {
    target.addEventListener(eventType, callback);
    return () => target.removeEventListener(eventType, callback);
  }, [target, eventType, ...deps]);
}
